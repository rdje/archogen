#!/usr/bin/env python3
"""Refuse source forms the pinned mdBook silently misrenders; never rewrite them.

This is conservative validation of standalone fence syntax, not a Markdown parser.
Owner: M3.6.9.6.3.2.2.15.2.1.4.3.4.0. The upstream compatibility successor
owns retiring these checks after qualifying a corrected renderer.
"""
from pathlib import Path
import os
import re
import shutil
import subprocess
import sys
import tempfile

OPEN = re.compile(r"^ {0,3}(`{3,}|~{3,})(.*)$")
CLOSE = re.compile(r"^ {0,3}(`{3,}|~{3,})([ \t]*)$")


def problem(data: bytes) -> tuple[int, str] | None:
    """Return the first unsupported source form, with its LF-based source line."""
    try:
        text = data.decode("utf-8")
    except UnicodeDecodeError as error:
        return data[:error.start].count(b"\n") + 1, "source is not UTF-8"
    bare_cr = re.search(r"\r(?!\n)", text)
    if bare_cr:
        return (
            text[:bare_cr.start()].count("\n") + 1,
            "bare CR line ending; use LF or CRLF so the pinned renderer preserves code/prose boundaries",
        )
    fence = None
    for number, raw in enumerate(text.split("\n"), 1):
        line = raw.removesuffix("\r")
        if fence is not None:
            close = CLOSE.fullmatch(line)
            if close and close[1][0] == fence[0] and len(close[1]) >= fence[1]:
                if "\t" in close[2]:
                    return number, "tab after closing markers; replace closing-line tabs with spaces"
                fence = None
            continue
        opening = OPEN.fullmatch(line)
        if opening and (opening[1][0] != "`" or "`" not in opening[2]):
            fence = opening[1][0], len(opening[1])
    return None


def check(root: Path) -> int:
    source = root / "docs/book/src"
    paths = sorted(source.rglob("*.md"))
    if not paths:
        print("book-sources: docs/book/src: no Markdown source found", file=sys.stderr)
        return 1
    failed = False
    for path in paths:
        name = path.relative_to(root).as_posix()
        try:
            result = problem(path.read_bytes())
        except OSError as error:
            print(f"book-sources: {name}: cannot read source: {error}", file=sys.stderr)
            failed = True
            continue
        if result:
            number, message = result
            print(f"book-sources: {name}:{number}: {message}", file=sys.stderr)
            failed = True
    if failed:
        return 1
    print("book-sources: OK (pinned-renderer source forms; no rewrite)")
    return 0


def self_test(root: Path) -> int:
    cases = [
        ("ordinary prose", b"nine tests\n", None),
        ("LF tilde", b"~~~text\nseven tests\n~~~\n", None),
        ("CRLF tilde", b"~~~text\r\nseven tests\r\n~~~\r\n", None),
        ("bare-CR tilde", b"~~~text\rseven tests\r~~~\r", (1, "bare CR")),
        ("bare-CR backtick", b"```text\rseven tests\r```\rnine tests\r", (1, "bare CR")),
        ("mixed line endings", b"# Chapter\r\n\nprose\rnext\n", (3, "bare CR")),
        ("tilde closing tab", b"~~~text\nseven tests\n~~~ \t\nnine tests\n", (3, "tab after closing")),
        ("backtick closing tab", b"```text\nseven tests\n```\t\nnine tests\n", (3, "tab after closing")),
        ("indented closing tab", b"  ~~~~text\nseven tests\n   ~~~~ \t\n", (3, "tab after closing")),
        ("longer closing tab", b"~~~text\nseven tests\n~~~~~\t\n", (3, "tab after closing")),
        ("closing spaces", b"~~~text\nseven tests\n~~~   \nnine tests\n", None),
        ("short literal tab markers", b"~~~~text\n~~~ \t\nseven tests\n~~~~\n", None),
        ("mismatched literal tab markers", b"~~~text\n``` \t\nseven tests\n~~~\n", None),
        ("literal text after markers", b"~~~text\n~~~ \tstill data\nseven tests\n~~~\n", None),
        ("overindented literal tab markers", b"~~~text\n    ~~~ \t\nseven tests\n~~~\n", None),
        ("opening whitespace", b"~~~ \t\nseven tests\n~~~\n", None),
        ("invalid backtick info", b"```info`bad\n~~~\t\nseven tests\n~~~\n", None),
        ("unclosed literal tab data", b"~~~text\npayload\t\n", None),
        ("escaped CR text", br"source\nwith\rcontrol" + b"\n", None),
        ("invalid UTF-8", b"# Chapter\n\xff\n", (2, "not UTF-8")),
    ]
    passed = 0
    arms = 0

    def arm(name: str, good: bool, detail: object = "") -> None:
        nonlocal passed, arms
        arms += 1
        if good:
            passed += 1
        else:
            print(f"SELF-TEST: {name}: {detail}", file=sys.stderr)

    for name, source, expected in cases:
        actual = problem(source)
        good = actual is None if expected is None else (
            actual is not None and actual[0] == expected[0] and expected[1] in actual[1]
        )
        arm(name, good, actual)

    scratch = root / "target/doctrine_scratch/book_sources"
    scratch.mkdir(parents=True, exist_ok=True)
    work = Path(tempfile.mkdtemp(prefix="selftest-", dir=scratch))
    try:
        (work / "scripts").mkdir()
        (work / ".github").mkdir()
        (work / "docs/book/src/nested").mkdir(parents=True)
        (work / "bin").mkdir()
        shutil.copyfile(root / "scripts/check_book_sources.py", work / "scripts/check_book_sources.py")
        shutil.copyfile(root / ".github/ci-tools.env", work / ".github/ci-tools.env")
        pins = (work / ".github/ci-tools.env").read_text()
        pin = re.search(r"^MDBOOK_VERSION_PINNED=(.+)$", pins, re.M)[1]
        stub = work / "bin/mdbook"
        stub.write_text(
            "#!/usr/bin/env bash\n"
            f'[ "$1" = --version ] && {{ echo "mdbook v{pin}"; exit 0; }}\n'
            '[ "$1" = build ] || exit 2\n'
            "printf called > renderer-ran\n"
        )
        stub.chmod(0o755)
        subprocess.run(["git", "init", "-q", str(work)], check=True)
        env = dict(os.environ, PATH=str(work / "bin") + os.pathsep + os.environ["PATH"])
        chapter = work / "docs/book/src/nested/control.md"
        phases = [
            ("ordinary", b"~~~text\nseven tests\n~~~\n", 0, ""),
            ("bare-CR tilde", b"~~~text\rseven tests\r~~~\r", 1, "control.md:1: bare CR"),
            ("bare-CR backtick", b"```text\rseven tests\r```\rnine tests\r", 1, "control.md:1: bare CR"),
            ("closing tab", b"~~~text\nseven tests\n~~~\t\nnine tests\n", 1, "control.md:3: tab after closing"),
            ("recovered", b"~~~text\r\nseven tests\r\n~~~  \r\nnine tests\r\n", 0, ""),
        ]
        marker = work / "renderer-ran"
        for name, data, expected, message in phases:
            chapter.write_bytes(data)
            marker.unlink(missing_ok=True)
            result = subprocess.run(
                ["bash", str(root / "scripts/build_book.sh")], cwd=work, env=env,
                text=True, capture_output=True,
            )
            arm(
                "public wrapper " + name,
                result.returncode == expected and (not message or message in result.stderr)
                and marker.exists() == (expected == 0) and chapter.read_bytes() == data,
                (result.returncode, marker.exists(), result.stdout, result.stderr),
            )
        # A checker failure must remain a build failure with the wrapper's
        # documented exit code, and must never fall through to the renderer.
        (work / "scripts/check_book_sources.py").write_text("raise SystemExit(7)\n")
        marker.unlink(missing_ok=True)
        result = subprocess.run(
            ["bash", str(root / "scripts/build_book.sh")], cwd=work, env=env,
            text=True, capture_output=True,
        )
        arm(
            "public wrapper checker failure",
            result.returncode == 1 and not marker.exists() and chapter.read_bytes() == phases[-1][1],
            (result.returncode, marker.exists(), result.stdout, result.stderr),
        )
    finally:
        shutil.rmtree(work)
    print(f"book-source self-test: {passed} pass / {arms - passed} fail ({arms} arms)")
    return int(passed != arms)


def main() -> int:
    root = Path(__file__).resolve().parents[1]
    if sys.argv[1:] == ["--self-test"]:
        return self_test(root)
    if sys.argv[1:]:
        print("book-sources: takes no arguments, or --self-test", file=sys.stderr)
        return 2
    return check(root)


if __name__ == "__main__":
    sys.exit(main())
