- ID: `PROGRAM.104`
  Status: `done` — P2, found `2026-10-22` by `PROGRAM.102`'s census of the record it moves, taken first: that leaf
    edits the declaration this one registers
  Reproduce / issue: `crates/rt-analysis/src/runtime.rs:19`, `pub const MODEL: &str =` with its value
    `"fixed-priority-with-overheads/2 (…)"` on the next line, as rustfmt wraps it. `VERSION-REGISTER` reads declarations
    one line at a time (`scripts/check_version_register.sh`, its `sed -nE` over `pub const …: &str = "…";`), so this one
    escapes its census: the model every runtime-variant conclusion names, bumped `/1` → `/2` by `M2.11`, has no entry
    in `docs/book/src/versions.md`, and its next bump could land unrecorded — the hole the register exists to close.
    Impact: a reader of the register cannot learn the runtime variant's model or its change rule; nothing else breaks.
  Goal: the census reads a declaration whatever its line breaks, a RED arm holding a wrapped one; the runtime model an
    entry of the register, its change rule the record's.
  Acceptance:
    - [x] ROOT CAUSE — every `pub const …: &str =` whose value is on the next line, census over `crates/*/src`:
      `crates/rt-analysis/src/runtime.rs:20`, `MODEL`, the one; `git grep -c "fixed-priority-with-overheads"
      docs/book/src/versions.md`: none. `declared()` ran each pattern over the file line by line, so a wrapped
      declaration matched none, and the register's census, "derived from the code", was blind to it.
    - [x] FIX — `scripts/check_version_register.sh`: `joined()` puts each `pub const` of type `&str` or `Version` on one
      line, up to its `;`, before the patterns read it, a wrapped `Version`'s trailing comma accepted; three arms — a
      wrapped identifier, a wrapped `Version`, and a profile `id:` inside a public constant's literal still read.
      `docs/book/src/versions.md`: the `runtime-analysis-model` entry and the census sentence; the book index
      regenerated. `crates/rt-analysis/tests/runtime_variant.rs`: a pin holding `MODEL` to the identifier its record
      states and to the record's path, which nothing held before.
    - [x] ADDRESSED — `bash scripts/check_version_register.sh` before the entry: exit 1, "the code declares
      const:crates/rt-analysis/src/runtime.rs:MODEL = 'fixed-priority-with-overheads/2', and no entry … claims it";
      with it, `version-register: OK (12 entries; 12 declared version(s), each carried by its entry)`, from 11. Its
      `--self-test`: `17 pass / 0 fail (17 arms)`; under a line-at-a-time reader `14 pass / 3 fail` — the two wrapped
      arms and the real tree; under my first draft, joining every `pub const`, `15 pass / 2 fail` — the profile arm and
      the real tree. The pin: `cargo test -p rt-analysis --test runtime_variant -- the_model_names`: `1 passed`; the
      identifier bumped alone, `0 passed; 1 failed`; the citation pointed elsewhere, `0 passed; 1 failed`; restored.
    - [x] NO REGRESSION — `bash scripts/run_self_tests.sh`: every self-test passes once the book index is regenerated
      (`book-index` refused the unregenerated index, `7 pass / 1 fail`, its real-book arm); `make focused` on this
      staged tree: exit code 0, `tier focused: passed — 3 passed, 0 failed, 0 unavailable, 0 not built, 0 quarantined`.
    - [x] LOCKSTEP — the register's entry and census sentence, the generated book index; `DOCTRINE_ENFORCEMENT.md`'s
      `VERSION-REGISTER` row stays true ("every `pub const` whose value is a format identifier") and is 22 bytes under
      its cap, so the detail is the book's.
  Verification: the register failing first; its self-test armed both ways; the pin armed; every self-test; focused.
  Commit: `ARCHOGEN-PROGRAM-0865 (leaf PROGRAM.104)`
