#!/usr/bin/env python3
"""Finite, project-local pipe fixture for M3.6.9.6.3.2.2.15.2.1.3.3.3.2.8.7."""

import argparse
import errno
import os
from pathlib import Path
import stat
import time

PAYLOAD = b"stream artifact bytes\n"


def owned_path(value):
    root = Path(__file__).resolve().parents[2]
    base = (root / "target/trust-tests/artifact-output-kind-pipes").resolve(strict=True)
    if not base.is_relative_to(root):
        raise ValueError("pipe fixture directory is outside its project")
    path = Path(value)
    parent = path.parent.resolve(strict=True)
    if not parent.is_relative_to(base) or parent.stat().st_dev != root.stat().st_dev:
        raise ValueError("pipe fixture path is outside its project-volume directory")
    return parent / path.name


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("action", choices=["create", "write"])
    parser.add_argument("--pipe", required=True, type=owned_path)
    parser.add_argument("--stop", type=owned_path)
    args = parser.parse_args()
    if args.action == "create":
        os.mkfifo(args.pipe, 0o600)
        if not stat.S_ISFIFO(args.pipe.lstat().st_mode):
            raise ValueError("created fixture is not a named pipe")
        print("created")
        return
    if args.stop is None:
        parser.error("write requires --stop")
    if not stat.S_ISFIFO(args.pipe.lstat().st_mode):
        raise ValueError("writer fixture is not a named pipe")
    deadline = time.monotonic() + 3
    while time.monotonic() < deadline:
        try:
            stopped = args.stop.lstat()
        except FileNotFoundError:
            pass
        else:
            if not stat.S_ISREG(stopped.st_mode):
                raise ValueError("writer stop marker is not a regular file")
            print("stopped")
            return
        try:
            descriptor = os.open(args.pipe, os.O_WRONLY | os.O_NONBLOCK | os.O_NOFOLLOW)
        except OSError as error:
            if error.errno != errno.ENXIO:
                raise
            time.sleep(0.005)
            continue
        try:
            if not stat.S_ISFIFO(os.fstat(descriptor).st_mode):
                raise ValueError("opened writer fixture is not a named pipe")
            if os.write(descriptor, PAYLOAD) != len(PAYLOAD):
                raise OSError("incomplete pipe fixture write")
        finally:
            os.close(descriptor)
        print("wrote")
        return
    raise SystemExit("pipe fixture deadline expired without a reader or stop marker")


if __name__ == "__main__":
    main()
