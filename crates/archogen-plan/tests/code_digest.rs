//! What an application's `code` digest is of, held by asking the compiler (`docs/decisions/decision_application-inputs.md`, "Which code"; round 6, AI6-1 and AI6-2).
//!
//! Three rounds of the format's review found the digest's recipe wrong, each time in what it was a digest of: every file at a place, then every file under `src`. A
//! source names files a directory's listing cannot know, by `include!`, `#[path]` and `include_bytes!`, and a directory holds files no source names. So the digest is of
//! what the compiler read, by its own account, its dep-info, and this is the instrument that holds that account to be what the record says of it
//! (`docs/decisions/decision_executable-design-reviews.md`, "A measuring instrument"): the pinned compiler itself, run on a crate whose sources name files beside
//! `src` with files under `src` that none names, and on names a line of text can hardly hold; and the reader of an account, held over every small account of such names.
//!
//! Everything is written under `CARGO_TARGET_TMPDIR`: inside `target/`, on the repository's own volume.

use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

use archogen_digest::Digest;
use archogen_plan::application::{Account, Code};

/// The variables these tests have the compiler read, each unset but where a test sets it.
const VARIABLES: [&str; 5] = [
    "ARCHOGEN_CODE_DIGEST_RATE",
    "ARCHOGEN_CODE_DIGEST_SET",
    "ARCHOGEN_CODE_DIGEST_LINES",
    "ARCHOGEN_CODE_DIGEST_UNSET",
    "ARCHOGEN_CODE_DIGEST_EMPTY",
];

/// A fresh directory of one test's own.
fn scratch(name: &str) -> PathBuf {
    let dir = Path::new(env!("CARGO_TARGET_TMPDIR"))
        .join("code-digest")
        .join(name);
    let _ = fs::remove_dir_all(&dir);
    fs::create_dir_all(&dir).expect("made");
    dir
}

fn write(place: &Path, file: &str, text: &str) {
    let at = place.join(file);
    fs::create_dir_all(at.parent().expect("a directory")).expect("made");
    fs::write(at, text).expect("written");
}

/// The compiler's account of compiling `src/main.rs` at `place`, as it wrote it: the pinned `rustc`, run there, with these variables set.
fn the_account(place: &Path, made: &str, emit: &[&str], set: &[(&str, &str)]) -> String {
    let mut compiler = Command::new("rustc");
    compiler
        .current_dir(place)
        .args(["--edition", "2021"])
        .args(emit)
        .arg("src/main.rs");
    for variable in VARIABLES {
        compiler.env_remove(variable);
    }
    for (variable, value) in set {
        compiler.env(variable, value);
    }
    let ran = compiler.output().expect("the compiler runs");
    assert!(
        ran.status.success(),
        "{}",
        String::from_utf8_lossy(&ran.stderr)
    );
    fs::read_to_string(place.join(made)).expect("its account")
}

/// Its account alone, written beside the crate.
const ALONE: [&str; 3] = ["--emit=dep-info", "-o", "../main.d"];

/// An account as the compiler writes one, the model [`Code::read_by_the_compiler`] is the inverse of: a rule for each file made and an empty line after it, a line
/// for each file read, then an empty line and a line for each variable where there is one; a space of a name after a backslash, and nothing else of one escaped.
/// `the_compiler_writes_its_account_as_the_reader_reads_one` holds it to the compiler's own bytes.
fn written(made: &[&str], files: &[&str], environment: &[&str]) -> String {
    let names: Vec<String> = files.iter().map(|file| file.replace(' ', "\\ ")).collect();
    let rules = made
        .iter()
        .map(|made| format!("{made}: {}\n\n", names.join(" ")));
    let lines = names.iter().map(|name| format!("{name}:\n"));
    let variables = environment
        .iter()
        .enumerate()
        .map(|(at, variable)| format!("{}# env-dep:{variable}\n", if at == 0 { "\n" } else { "" }));
    rules.chain(lines).chain(variables).collect()
}

fn owned(names: &[&str]) -> Vec<String> {
    names.iter().map(|&name| name.to_owned()).collect()
}

/// The digest of the crate at `place` as the record gives it: of what the compiler says it read in compiling it there, and of its manifest and its lock.
fn digest(place: &Path, set: &[(&str, &str)]) -> (String, Account) {
    let account = Code::read_by_the_compiler(&the_account(place, "../main.d", &ALONE, set))
        .expect("an account");
    let sources: Vec<(String, Vec<u8>)> = account
        .files
        .iter()
        .map(String::as_str)
        .chain(["Cargo.toml", "Cargo.lock"])
        .map(|file| {
            (
                file.to_owned(),
                fs::read(place.join(file)).expect("a source"),
            )
        })
        .collect();
    (
        Code::digest_of(&sources, &account.environment).expect("a digest"),
        account,
    )
}

#[test]
fn the_digest_is_of_what_the_compiler_read_and_of_nothing_else() {
    const RATE: &str = "ARCHOGEN_CODE_DIGEST_RATE";
    const MAIN: &str = "include!(\"../gen/beat.rs\");\n#[path = \"../shared/tick.rs\"]\nmod tick;\nfn main() {\n    println!(\"{} {} {:?} {:?}\", beat(), tick::tick(), include_bytes!(\"../data/table.bin\"), option_env!(\"ARCHOGEN_CODE_DIGEST_RATE\"));\n}\n";
    let place = scratch("app").join("app");
    for (file, text) in [
        (
            "Cargo.toml",
            "[package]\nname = \"app\"\nversion = \"0.1.0\"\nedition = \"2021\"\n",
        ),
        (
            "Cargo.lock",
            "version = 4\n\n[[package]]\nname = \"app\"\nversion = \"0.1.0\"\n",
        ),
        ("src/main.rs", MAIN),
        ("gen/beat.rs", "pub fn beat() -> u32 { 1 }\n"),
        ("shared/tick.rs", "pub fn tick() -> u32 { 10 }\n"),
        ("data/table.bin", "A"),
        // What no source names: an inputs file for each of two targets, an editor's copy of one, a realization's file, a hidden file, a source no `mod` names, and notes.
        ("src/heartbeat.app", "(format \"archogen-application/1\")\n"),
        ("src/virt.app", "(format \"archogen-application/1\")\n"),
        ("src/heartbeat.app~", "an editor's copy\n"),
        (
            "src/figures.realization",
            "(format \"archogen-realization/1\")\n",
        ),
        ("src/.hidden", "hidden\n"),
        ("src/unused.rs", "pub fn unused() {}\n"),
        ("notes.txt", "notes\n"),
    ] {
        write(&place, file, text);
    }
    let (first, account) = digest(&place, &[]);
    // Each file a source names is among them wherever it lies, by the name it was read by, and no file that none names. A variable it asked for and did not find is one too.
    let mut files = account.files.clone();
    files.sort();
    assert_eq!(
        files,
        [
            "src/../data/table.bin",
            "src/../gen/beat.rs",
            "src/../shared/tick.rs",
            "src/main.rs"
        ]
    );
    assert_eq!(account.environment, [RATE]);
    // What a job runs moves the digest: a file `include!` reads, one `#[path]` names, one `include_bytes!` holds, the root source, the manifest and the lock.
    let mut last = first;
    for (file, text) in [
        ("gen/beat.rs", "pub fn beat() -> u32 { 2 }\n"),
        ("shared/tick.rs", "pub fn tick() -> u32 { 20 }\n"),
        ("data/table.bin", "B"),
        ("src/main.rs", &format!("{MAIN}// and a comment\n")),
        (
            "Cargo.toml",
            "[package]\nname = \"app\"\nversion = \"0.2.0\"\nedition = \"2021\"\n",
        ),
        (
            "Cargo.lock",
            "version = 4\n\n[[package]]\nname = \"app\"\nversion = \"0.2.0\"\n",
        ),
    ] {
        write(&place, file, text);
        let (moved, _) = digest(&place, &[]);
        assert_ne!(moved, last, "a change of `{file}` moved no digest");
        last = moved;
    }
    // What no source names moves none, a figure written into either inputs file among it: so each can hold the digest, and writing it moves nothing.
    for file in [
        "src/heartbeat.app",
        "src/virt.app",
        "src/heartbeat.app~",
        "src/figures.realization",
        "src/.hidden",
        "src/unused.rs",
        "notes.txt",
    ] {
        write(&place, file, &format!("(digest \"{last}\")\n"));
        assert_eq!(
            digest(&place, &[]).0,
            last,
            "a change of `{file}`, which no source names, moved the digest"
        );
    }
    // And what it read of the environment moves it: a variable set, set otherwise, and set to nothing, which is not one unset.
    let mut seen = vec![last];
    for (value, said) in [
        ("100", "ARCHOGEN_CODE_DIGEST_RATE=100"),
        ("200", "ARCHOGEN_CODE_DIGEST_RATE=200"),
        ("", "ARCHOGEN_CODE_DIGEST_RATE="),
    ] {
        let (moved, account) = digest(&place, &[(RATE, value)]);
        assert_eq!(account.environment, [said]);
        assert!(!seen.contains(&moved), "`{said}` moved no digest");
        seen.push(moved);
    }
}

#[test]
fn the_compiler_writes_its_account_as_the_reader_reads_one() {
    let place = scratch("odd").join("app");
    // Names a line of text can hardly hold: a space, first, within and last; a backslash, one before a space and one last; `#`, `$`, a colon, a tab, a letter past
    // ASCII; and an account's own words.
    let odd = [
        "a b.rs",
        " lead.rs",
        "trail ",
        "a\\b.rs",
        "a\\ b.rs",
        "end\\",
        "a#b.rs",
        "a$b.rs",
        "a:b.rs",
        "tab\there.rs",
        "caf\u{e9}.rs",
        "# env-dep:X=1",
    ];
    let mut main = String::new();
    for (at, name) in odd.iter().enumerate() {
        write(
            &place,
            &format!("src/{name}"),
            &format!("pub fn f{at}() -> u32 {{ {at} }}\n"),
        );
        main.push_str(&format!("mod m{at} {{ include!({name:?}); }}\n"));
    }
    main.push_str("pub const SET: &str = env!(\"ARCHOGEN_CODE_DIGEST_SET\");\npub const LINES: &str = env!(\"ARCHOGEN_CODE_DIGEST_LINES\");\n");
    main.push_str("pub const UNSET: Option<&str> = option_env!(\"ARCHOGEN_CODE_DIGEST_UNSET\");\npub const EMPTY: Option<&str> = option_env!(\"ARCHOGEN_CODE_DIGEST_EMPTY\");\n");
    main.push_str("fn main() {}\n");
    write(&place, "src/main.rs", &main);
    let set = [
        ("ARCHOGEN_CODE_DIGEST_SET", "a value = with spaces"),
        ("ARCHOGEN_CODE_DIGEST_LINES", "one\ntwo\\three\rfour"),
        ("ARCHOGEN_CODE_DIGEST_EMPTY", ""),
    ];
    let text = the_account(&place, "../main.d", &ALONE, &set);
    let files: Vec<String> = std::iter::once("src/main.rs".to_owned())
        .chain(odd.iter().map(|name| format!("src/{name}")))
        .collect();
    let names: Vec<&str> = files.iter().map(String::as_str).collect();
    // A value's line feed, carriage return and backslash are written after a backslash, so a variable is one line; one unset has no `=`, and one set to nothing has.
    let environment = [
        "ARCHOGEN_CODE_DIGEST_EMPTY=",
        "ARCHOGEN_CODE_DIGEST_LINES=one\\ntwo\\\\three\\rfour",
        "ARCHOGEN_CODE_DIGEST_SET=a value = with spaces",
        "ARCHOGEN_CODE_DIGEST_UNSET",
    ];
    // The model of what the compiler writes is what it wrote, byte for byte, and it reads back as the files and the variables it is of.
    assert_eq!(text, written(&["../main.d"], &names, &environment));
    let read = Some(Account {
        files: files.clone(),
        environment: owned(&environment),
    });
    assert_eq!(Code::read_by_the_compiler(&text), read);
    // Making more than its account, as it does under cargo, it writes a rule for each file it makes, and that reads as the same account.
    fs::create_dir_all(place.join("../out dir")).expect("made");
    let several = the_account(
        &place,
        "../out dir/odd.d",
        &[
            "--crate-name",
            "odd",
            "--emit=dep-info,metadata",
            "--out-dir",
            "../out dir",
        ],
        &set,
    );
    assert!(
        several
            .lines()
            .filter(|line| line.contains(": src/main.rs "))
            .count()
            > 1,
        "{several}"
    );
    assert_eq!(Code::read_by_the_compiler(&several), read);
    // A name holding a line feed breaks the account's lines, and such an account is none: no piece of a name is taken for a file.
    write(&place, "src/a\nb.rs", "pub fn broken() -> u32 { 0 }\n");
    write(
        &place,
        "src/main.rs",
        &format!("{main}mod broken {{ include!(\"a\\nb.rs\"); }}\n"),
    );
    let broken = the_account(&place, "../main.d", &ALONE, &set);
    assert!(broken.contains("src/a\nb.rs"), "{broken}");
    assert_eq!(Code::read_by_the_compiler(&broken), None);
}

/// The reader over every small account: each of names holding no line feed reads back as the files and variables it was written of, and each with a name holding one
/// is none, whatever the name's pieces look like, a file's line, an empty line or a variable's among them.
#[test]
fn every_small_account_reads_as_written_or_as_none() {
    const PIECES: [&str; 8] = ["a", " ", "\\", "\n", "\n\n", ":", "a:", "# env-dep:a"];
    /// Each name of up to `most` pieces.
    fn names(most: usize) -> Vec<String> {
        let (mut all, mut last) = (Vec::new(), vec![String::new()]);
        for _ in 0..most {
            last = last
                .iter()
                .flat_map(|head| PIECES.iter().map(move |piece| format!("{head}{piece}")))
                .collect();
            all.extend(last.iter().cloned());
        }
        all
    }
    let (long, short, one) = (names(3), names(2), names(1));
    let mut lists: Vec<Vec<&str>> = long.iter().map(|name| vec![name.as_str()]).collect();
    for a in &short {
        for b in &short {
            lists.push(vec![a, b]);
        }
    }
    for a in &long {
        for b in &one {
            lists.push(vec![a, b]);
            lists.push(vec![b, a]);
        }
    }
    for a in &one {
        for b in &one {
            for c in &one {
                lists.push(vec![a, b, c]);
            }
        }
    }
    let shapes: [(&[&str], &[&str]); 2] = [(&["o"], &[]), (&["o", "p q"], &["A=b", "C"])];
    let (mut read, mut none) = (0_u32, 0_u32);
    for files in &lists {
        for (made, environment) in shapes {
            let text = written(made, files, environment);
            let got = Code::read_by_the_compiler(&text);
            if files.iter().any(|name| name.contains('\n')) {
                assert_eq!(got, None, "{text:?}");
                none += 1;
            } else {
                assert_eq!(
                    got,
                    Some(Account {
                        files: owned(files),
                        environment: owned(environment)
                    }),
                    "{text:?}"
                );
                read += 1;
            }
        }
    }
    // 15,624 lists of names, each under two shapes of account.
    assert_eq!((read, none), (10_668, 20_580));
}

#[test]
fn a_text_that_is_not_wholly_an_account_is_none() {
    // As the compiler wrote it of a library, three files made, into a directory whose name holds a space: a rule's own file is written as it is.
    let cargo = "../out dir/app.d: src/lib.rs\n\n../out dir/libapp.rlib: src/lib.rs\n\n../out dir/libapp.rmeta: src/lib.rs\n\nsrc/lib.rs:\n";
    assert_eq!(
        Code::read_by_the_compiler(cargo),
        Some(Account {
            files: owned(&["src/lib.rs"]),
            environment: Vec::new()
        })
    );
    let good = written(&["o"], &["src/main.rs", "src/a b.rs"], &["A=b"]);
    assert_eq!(
        good,
        "o: src/main.rs src/a\\ b.rs\n\nsrc/main.rs:\nsrc/a\\ b.rs:\n\n# env-dep:A=b\n"
    );
    assert_eq!(
        Code::read_by_the_compiler(&good),
        Some(Account {
            files: owned(&["src/main.rs", "src/a b.rs"]),
            environment: owned(&["A=b"])
        })
    );
    for (bad, why) in [
        (String::new(), "nothing"),
        ("no rule here\n".to_owned(), "no rule"),
        (good.trim_end().to_owned(), "a last line unended"),
        (
            good.replacen("\n\n", "\n", 1),
            "a rule with no empty line after it",
        ),
        (
            good.replacen("\n\n", "\n\n\n", 1),
            "two empty lines after a rule",
        ),
        (
            good.replace("o: ", "o:"),
            "a rule with no space after its colon",
        ),
        (
            good.replace("src/main.rs:\n", "src/main.rs\n"),
            "a file's line with no colon",
        ),
        (
            good.replace("src/main.rs:\n", ""),
            "a file its rule names and no line gives",
        ),
        (
            good.replace("o: src/main.rs ", "o: "),
            "a file a line gives and its rule does not name",
        ),
        (
            good.replace("src/a\\ b.rs:\n", "src/a b.rs:\n"),
            "a space of a name with no backslash before it",
        ),
        (
            good.replace("# env-dep:A=b\n", "# a comment\n"),
            "a line that is none of an account's",
        ),
        (
            good.replace("# env-dep:A=b\n", ""),
            "an empty line and no variable after it",
        ),
        (
            format!("{good}src/other.rs:\n"),
            "a file's line after the environment's",
        ),
        (good.replace('\n', "\r\n"), "lines ended otherwise"),
    ] {
        assert_ne!(bad, good, "{why}");
        assert_eq!(Code::read_by_the_compiler(&bad), None, "{why}: {bad:?}");
    }
}

#[test]
fn the_digest_is_of_its_lines_in_order_as_bytes() {
    let source = |named: &str, bytes: &str| (named.to_owned(), bytes.as_bytes().to_vec());
    let (a, b) = (source("a.rs", "one"), source("b/c.rs", "two"));
    let environment = owned(&["RATE=100", "UNSET"]);
    let mut lines = [
        format!("{}  a.rs\n", Digest::of(b"one").hex()),
        format!("{}  b/c.rs\n", Digest::of(b"two").hex()),
        "# env-dep:RATE=100\n".to_owned(),
        "# env-dep:UNSET\n".to_owned(),
    ];
    lines.sort();
    assert!(
        lines[0].starts_with('#') && lines[1].starts_with('#'),
        "a variable's line comes before a file's"
    );
    let want = Some(format!(
        "sha256:{}",
        Digest::of(lines.concat().as_bytes()).hex()
    ));
    assert_eq!(Code::digest_of(&[a.clone(), b.clone()], &environment), want);
    // Whatever order they are given in, and a source or a variable given twice is written once.
    let turned = owned(&["UNSET", "RATE=100", "UNSET"]);
    assert_eq!(
        Code::digest_of(&[b.clone(), a.clone(), b.clone()], &turned),
        want
    );
    // Each part of it moves it: a source's bytes, its name, a source more, a variable's value, a variable set to nothing where it was not set, a variable fewer.
    for (sources, environment) in [
        (
            vec![source("a.rs", "ONE"), b.clone()],
            owned(&["RATE=100", "UNSET"]),
        ),
        (
            vec![source("A.rs", "one"), b.clone()],
            owned(&["RATE=100", "UNSET"]),
        ),
        (
            vec![a.clone(), b.clone(), source("c.rs", "one")],
            owned(&["RATE=100", "UNSET"]),
        ),
        (vec![a.clone(), b.clone()], owned(&["RATE=200", "UNSET"])),
        (vec![a.clone(), b.clone()], owned(&["RATE=100", "UNSET="])),
        (vec![a.clone(), b.clone()], owned(&["RATE=100"])),
    ] {
        let moved = Code::digest_of(&sources, &environment);
        assert!(
            moved.is_some() && moved != want,
            "{sources:?} {environment:?}"
        );
    }
    // And there is none where its lines cannot be written: a name holding a line feed, no name, one name of two contents, a variable holding a line feed.
    assert_eq!(Code::digest_of(&[source("a\nb.rs", "one")], &[]), None);
    assert_eq!(Code::digest_of(&[source("", "one")], &[]), None);
    assert_eq!(
        Code::digest_of(&[a.clone(), source("a.rs", "two")], &[]),
        None
    );
    assert_eq!(Code::digest_of(&[a], &owned(&["A=one\ntwo"])), None);
}
