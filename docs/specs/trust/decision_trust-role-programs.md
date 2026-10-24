# The checkers' programs: the configuration checker's root, its shared derivation, invoked programs, what roles share

- **Type:** `decision`
- **Date:** `2026-10-23`
- **Status:** `active` — written `2026-10-23` by leaf `M3.5.2`; rounds 1 to 14 answered by `2026-10-24`; under review
  ([`decision_trust-role-programs-reviews.md`](../../reviews/decision_trust-role-programs-reviews.md)), from round 5 read
  against its readers' code and fixtures, `M3.5.2`'s children, its rules in force as each lands.
- **External sources:** none beyond the parent's: `ROADMAP.md` §4.4 (its "externally invoked tools", and the classes
  of a shared item, "reference derivation" among them), §7.1's properties and §14.4's cases.
- **Owner / source:** leaf `M3.5.2` (`docs/tasks/M3.md`), as `M3.5`'s and `M3.5.1`'s reviewed designs assign it, under
  the director's ruling of `2026-10-22` (findings §16 (a): each checker an executable of its own, the human command
  line alone running it). It amends [`decision_trust-inventory.md`](decision_trust-inventory.md), "the parent", and
  [`decision_trust-generated-sources.md`](decision_trust-generated-sources.md), whose terms it uses unchanged.

## The fact / decision

### 1. What the parent cannot yet hold, measured

- **One derivation, invisible.** `M3.5` moves the substitutability relation's executable model, byte for byte, into
  `crates/archogen-plan-check` (`M3.5.3`); the tests that hold the generator's production code equal to it stay in
  `crates/eadl-resolve/tests`, the checker's package a development dependency. A census of their sources, comments and
  literals stripped and each `use` group read whole, finds two naming both the model and production modules:
  `production.rs` (the relation, over `common/universe.rs`) and `vocabulary.rs` (the typed table, entry by entry);
  `checker.rs`, `corpus.rs` and `common/universe.rs` name the model alone. Each pair is one derivation held equal,
  sharing transcribed code — the production relation follows the model line for line in places (`side_items`,
  `side_name_lists`, `horizon`; `M3.3`). The parent builds no test target but the harness's (`PROGRAM_KINDS` names no
  test), and follows development edges for the harness and an example's classification alone (parent §2, §3), so
  neither comparison is seen.
- **A harness for any pair.** The parent's prose gives a harness to a reference model's pair; the instrument reads a
  `defharness` naming any two roots (`check_pairs`), pairs it with every other root, and the verifier accepts a
  result whose role text is a recorded program's (`trust_verify.rs`).
- **A program that runs another.** The front, `archogen`, will run `archogen-plan-check` and `archogen-analyze` as
  processes (`ROADMAP.md` §10.2; `M3.5.1.4`) and read the install manifest (`archogen-install/1`), no blob of the
  commit. The parent sees what a root compiles and the data its form hands it (§3). §4.4's "externally invoked tools"
  are, in context, a build's; reading them as the programs a root runs is this record's extension, as `M3.5.1`'s.
- **What the roles share.** `cargo xtask trust-inventory --commit 0a5a4b12`, `aarch64-apple-darwin`: exit 0, `5
  program(s), 8 pair(s), 10 shared item(s)`, the tenth `generator+scheduling-checker package crates/archogen-digest`,
  reached through `archogen-outcome` (`archogen-api`'s since `5bc5627d`) and through `archogen-evidence`.
- **The census today** (`a7c45a88`, read from the sources, nothing built, on a snapshot of it holding `RP-H19`'s admission:
  the commit itself refuses that one site, `crates/rt-analysis/tests/runtime_variant.rs:510`, until `M3.5.8` admits
  it): `crates/rt-core`'s `lifecycle`, unit tests and doc tests join the reference model and the implementation beside
  the harness, unnamed (`RP-H18`) — the three the retired text scan found at `b9a070d4` and the retired build census at
  `76e10884`.
- **The gate is built from the base** (parent `TI-H6`; `.github/workflows/trust-gate.yml`): on a push, the commit
  `main` pointed at before it, which the report names. Its `read_roots` fails on a form or clause it does not know
  (exit 2), and its item encoding is the base's: `origin/main` is `8cdac117`, which does not hold `62099a39`, the
  structured-key repair (`M3.6.9.6.3.2.2.9.2`; its runner follow-up `M3.6.3.2.1.1`).

### 2. The configuration checker's root

From `M3.5.3` the role is rooted at its library, as the parent §2 roots a role no executable runs yet: `(defroot
configuration-checker (role configuration-checker) (package "crates/archogen-plan-check") (target lib) (role-packages
"crates/archogen-plan-check"))`, replacing `(defrole configuration-checker …)`; from `M3.5.6` its target is `(target bin
archogen-plan-check)`. The kind modules, `docs/semantics/kinds/core.eadl` and `os-rt.eadl`, are compiled in by
`include_str!` from `M3.5.4`, when check (2) re-checks the description — the model moved at `M3.5.3` reads none — as
`archogen-api` compiles them: shared files of the parent §4's second kind, each site admitted by a `defadmit` of its
own. They are not `(data …)`; any other fixed data it comes to read at run time is declared there by path. The
description and the assignment record it checks are claim subjects, never dependencies (`TI-H19`).

### 3. The shared derivation

**A derivation is a comparison harness for the generator and a checker, but for two things, beside its form's own
roles, reason and item kind (below):** it is
paired with no root — its pair takes its item, as a harness's pair does — and no result may name it as its producer.
Wherever the parent, the generated-sources record or their ledgers name the comparison harness as a program — what it
compiles, reads and reaches; how it is built, recorded, scanned, admitted, exempted, judged and reported; its item,
its provenance and its forms — a derivation is read the same way, but for those two things. That is complete by
construction: no passage needs listing, and a passage added later reads so too.

**The form.** `(defderivation <name> (pair generator <checker>) (package "<dir>") (test <target>) (reason "…"))`
names one test target comparing a reading of the generator's with a checker's; each form's
class is its own review's. Its text read wrong fails `read_roots` (exit 2), as a harness's does: a pair naming no root,
one root twice, or other than the generator's root and a checker's; a target another form names, a
harness's, a derivation's or a classification's. A package or target the commit lacks is `trust-baseline-stale`, from metadata, wherever
the gate runs. **`defharness` is narrowed:** its pair is one root of role `reference-model` and one of role
`implementation`, in either order; today's `differential` is.

**Its record and item.** Recorded in the inventory's programs as the harness is, its role `derivation`, its artifact
the test binary; its item kind `shared-derivation`, identity `<a>+<b> shared-derivation <name>`, the pair in the
inventory's order, one item a form, aspects the harness item's. Its exemption from `trust-shared-program` is its pair's
role packages: the checker's from `M3.5.3.1`, when the comparison compiles `archogen-plan-check`, and the generator's
from `M3.4`, when `crates/eadl-resolve` becomes one. Building it compiles `eadl-resolve`'s library, whose two
`include_str!` sites (`src/vocabulary.rs`: `kinds/deffact.eadl`, `vocabulary/vocabulary.eadl`) enter the scope and the
item, each admitted by a `defadmit`. The verifier refuses a result whose role is not one of `trust::ROLES`, and one
whose producer is a derivation's artifact; a reference model's result produced by its pair's harness stays accepted.

**Found by the sources, not by names.** Each test-like unit — an integration test, a bench, a library's or an
executable's unit tests, a library's doc tests — that some build of its package makes, whatever its flags, and whose
build compiles role packages of the two roles of a pair a property rests on (§5: the generator and either checker, the
reference model and the implementation) — by its normal and build edges over every feature and platform, a
non-member's own feature's among them, its declared dependencies edges (round 14's D4, `M3.5.2.11.3`), and its
package's own development edges, or by a file of one that its sources reach, however they name it (`M3.5.2.7.1`):
each target's crate root and every file it reaches by `mod`, `#[path]` and the `include!` family are read with nothing
built, whatever the profile, the features, the flags or the host (round 10's change of method), and each unit counts
its own target's files and its closure's libraries', each for the innermost package holding it by whole path
components, as a unit's package is decided (round 9's D3) — is a program target of the parent's §2, whatever its sources name: an integration test named by
a pair form if it compares them, any unit else classified by `defprogram`, `(target test|bench|lib-test|bin-test|doctest
<name>)`, its reason the claim that it compares nothing they hold equal. Unnamed, it is `trust-unclassified-program`,
and `trust-form-missing` on the baseline's host. §1's census of names found the two comparisons `RP-H11` names. A
comparison is an integration test, a pair form's; a classification says its unit compares nothing. Counting what
some build compiles over-reports what the default build makes, by design (`M3.5.2.3.1`). The inventory
records each one's pairs (`joins`), so the remedy names its pair's form, or says none can for a unit joining two or
compiling a role package of a root beside its pair's two, for which a pair form's build is refused (§2; round 8's R5,
`M3.5.2.5.2`); a classification of a test a pair form names is a read failure. Every file a target's crate root
reaches is read for every token the census knows can make rustc read a file — `mod`, `#[path]`, the `include!` family, a
`debugger_visualizer` file, an assembly macro's template, a macro's arguments and body, in code and in doc text,
whatever their `cfg` — and one the census does not read whole is refused unless an admission, rule `inclusion`, names
it by its extent's sha256, so a spelling of those it does not know is a refusal, never a miss, and a channel it does
not name is no part of that rule (the premise, below): no single string literal, an alias or a macro's, a module a
macro's expansion places, a `doc` attribute no literal gives, an assembly directive that reads a file or assembles one,
a doc test's own, a path outside the tree's text, through a symbolic link at any component, or spelt as hosts read apart
— a backslash, a drive, a trailing dot or space, a byte outside ASCII, a name the commit holds only in another case — and
a file that does not tokenize (`M3.5.2.7.1`, `M3.5.2.8.1`, `RP-H19`). A macro may rearrange what it is given (round
13's D1): in the arguments of any macro but std's that keep theirs in place — `assert!`, `format!`, `vec!` and their
kin, named as std's — whatever its name, a keyword only a later edition reserves among them, as the census reads no
edition (round 14's D2), every name of a macro that reads a file, invoked or not, a `mod` and a debugger visualizer are
refused, and in a `macro_rules!` body those, a macro a metavariable names and an attribute one fills; and a name of
those std macros bound to another, `std` renamed, a `macro_use`, a `macro_export` and a doc test's `macro_rules` are
refused, so every macro a crate or a doc test calls is std's, or read where it is called (`M3.5.2.10.1`,
`M3.5.2.11.1`). And the
census reads only what it can check it reads as rustc does: outside comments and literals a file's every byte is printable ASCII, a space, a tab, a form feed,
a newline or a carriage return — the whitespace rustc and the catalog's lexer both skip, where rustc also skips a
vertical tab and five Unicode spaces — so a file holding any other byte there, a character outside ASCII in a name
among them, or opening with a line rustc removes as a shebang, by rustc's own rule — the repository's, held to the pin — is
refused whole (`M3.5.2.9.1`, `M3.5.2.10.2`); `\` before a CRLF
continues a literal's line, as rustc normalises it, and a `doc` literal the census cannot decode is refused. Every
`path` attribute is followed with the module's own file beside it — a `cfg_attr`'s holds only where its `cfg` does,
and rustc reads the module's own file elsewhere — an inline module's name beside its paths alike; an inner `path`
attribute, naming its own module's directory, is refused; and every path value is checked as it is read, an inline
module's among them (`M3.5.2.9.2`). A doc
test's own is read with no markdown: in a library rustdoc tests — a `lib`, `rlib` or `proc-macro` — every doc comment,
`doc` literal and document a `doc` attribute includes is joined in source order, as rustdoc joins an item's — and what
a `doc` attribute's `test(…)` holds, which rustdoc adds to every doc test of the item's, read from that doc test's own
file (round 14's D3, `M3.5.2.11.2`) — and each
word in it that can make a doc test read a file — an inclusion or assembly macro before `!`, an alias or a comment, a
`mod` before a name, `debugger_visualizer`, `macro_rules`, or one ending the whole — read past any whitespace and any character
outside printable ASCII, is refused, but one a backtick opens with no quote or star between while no doc text of the library names an error
code: no compiled Rust holds a backtick outside a literal or comment, so that doc test cannot compile, whatever a file
it includes says, unless a `compile_fail` block names the code a file could make it fail with (`M3.5.2.8.2`). That is a
stated trade, no reading: rustc goes on past the stray backtick and still reads the file such a doc test includes, so
its package is compiled by a doc test whose outcome no file decides, and the census does not count it (round 14's R8).
What a module under exactly `#[cfg(test)]`,
outside any macro, or a module file whose leading inner attributes hold `#![cfg(test)]`, alone reaches counts for its
crate's own test units — `cfg(test)` is set on no dependency but by a build script's `cargo::rustc-cfg`, so a package
holding one narrows nothing (`M3.5.2.9.3`) — never for its library as another unit links it, a doc
test among them, nor for a binary built as no test; `cfg(test)` anywhere else is read as on (`M3.5.2.8.1`); only a
workspace member's test-like units are counted, a non-member's library, build script and executables read as an
edge — a build of the workspace can make each — its tests, benches and examples unread; a build script's files count as its package's library's; a symbolic link in a package's
directory or beside one, where a member glob may find it, and a crate root the census cannot read are refused — the
written tree holds no link — and the census's refusals, judged before any build, stop none and stand beside every
refusal and exit of their step after the census reads, past `cargo metadata` (`M3.5.2.8.3`); and a pair form's test is held to what the census reads of it as to
its build, a root's role package beside its pair refused, `trust-shared-program` (`M3.5.2.7.2`). A file counts for the package holding
it whatever it is — a manifest, a table an `include_str!` reads — so a document another role compiles from a role
package makes that package compiled, joining here and refused under `trust-shared-program`, not the parent's §4 file
item: share it from a package of no role (round 9's R7).

**What decides what a unit compiles** (rounds 9 and 10's changes of method), each rule quoted from the Cargo Book or the rustdoc
book of the pin ([`rust-toolchain`](../../book/src/ledger.md#rust-toolchain)) or the Rust reference it ships
([`rust-reference`](../../book/src/ledger.md#rust-reference)) where its cell holds quotation marks, and said in their
terms where it does not. Each row the census reads or refuses names the fixtures
that hold it, and each row with a part outside the census the report's line that says so; a test,
`the_census_table_is_held_by_its_fixtures_and_the_report`, reads this table and holds both to the code. And the census
is held to rustc and rustdoc themselves: an oracle builds a corpus of every probe of rounds 5 to 12 that the census
counts — those it refuses, a link or an assembly directive, are the fixtures' — in a matrix of configurations, `cargo
test`'s own selection, every target, release, every feature, none, a plain build in both profiles, each target a
selection skips by name with its required features, a non-member's own feature, through a compiler wrapper of its own, and finds every file of
the tree that a unit, every crate it links, transitively, and their packages' build scripts read counted by the
census for that unit and its closure, over a renamed, an optional, a platform's and a development dependency; and it
runs rustdoc on a library whose doc tests, one in each spelling rounds 11 to 13 found, each include a file the corpus
lacks, so rustdoc's failures name every block it compiled, and finds each refused (`M3.5.2.7.3`, `M3.5.2.8.4`); and a
generated leg holds the census's reading of where a literal or a comment ends to rustc's — every lexical construct of
the reference's, strings of every prefix and hash count, characters and lifetimes, comments nested and not, alone and
in every ordered pair with a trigger between, compiled by the pin as one crate, each trigger's file counted or refused
— which fails on each of four misread boundaries (`M3.5.2.9.4`); and three more legs hold its reading of a file's
head, of a trigger a macro spells and of an inline module's children — every head a file can open with before each
trigger, every spelling of five kinds of trigger through a macro, a macro a later edition's keyword names among them
in a crate of the edition before, every kind of file under every chain of up to two inline modules with and without
a `path`, in a block and not — each compiled by the pin as one crate, every file it reads counted or
refused (`M3.5.2.10.3`). Where the
Rust reference is silent, the children of a `path`-loaded module and a module an `include!`d file declares, the pin's
own refusal of a missing child names the directory, the loaded file's, which the census follows; and it names an inline
module's `path` relative to its file's own directory, a non-mod-rs file's too, the non-mod-rs module's name taken only
by an inline module with none outside a block, a function's or a constant's body taking no such name (`M3.5.2.10.2`,
`M3.5.2.11.1`). The oracle checks
what the census counts and that it refuses what rustdoc compiles; not what it refuses beyond that, which the fixtures
hold, nor what it counts beyond what a build reads, a non-member's tests, which no build of the workspace makes, or
what a build script or a procedural macro generates. **The premise, as it is checked** (round 13's D4): the census reads as rustc does as
far as its rules reach and its checks hold them, no further. Its rules name the channels through which rustc reads a
file — a module and its `path`, the `include!` family, a debugger visualizer, an assembly template, doc text, a macro's
arguments and body, a file's head and its alphabet — each read whole or refused; a channel they do not name is a gap no
check of theirs finds, as rounds 11 to 13 found three. Within them the corpus is finite: the fixtures hold each rule's
cases, and the four generated legs hold four families against the pin — where a literal or a comment ends, a file's
head, a trigger a macro spells, an inline module's children — and a spelling beyond those families is not claimed
checked.

| Input | The build tool's rule | The census | Held by | Report |
| --- | --- | --- | --- | --- |
| target selection | `cargo test` builds the library, the binaries, the examples "to ensure they compile", the unit tests and the integration tests; "Setting targets to `test = false` will stop them from being tested by default", and an option naming a target "will always test the given target" | read: every target the metadata lists, whatever its flags, nothing built | `the_census_reads_each_target_whatever_its_configuration`, `the_census_counts_every_file_any_build_of_the_corpus_reads` | — |
| required features | "If any of the required features are not enabled, the target will be skipped" | read: whatever features are on | `the_census_reads_each_target_whatever_its_configuration`, `the_census_counts_every_file_any_build_of_the_corpus_reads` | — |
| profile | "The `test` profile is the default profile used by cargo test", inheriting `dev`'s `debug-assertions = true` | read: no profile decides it, nothing being built | `the_census_reads_each_target_whatever_its_configuration`, `a_target_that_does_not_build_is_read_as_any_other`, `the_census_counts_every_file_any_build_of_the_corpus_reads` | — |
| `cfg(test)` | "Tests have the `cfg(test)` conditional expression enabled whether or not the harness is enabled" | read: every `cfg` at once but this one on a module no macro holds, or at a module file's head — what only such a module reaches counts for its crate's own test units, never for a unit linking it or built as no test, nor in a package holding a build script, which can set it | `a_test_only_inclusion_counts_for_its_crate_s_tests_alone`, `a_file_reached_through_cfg_test_alone_counts_for_its_crate_s_tests_alone`, `a_build_script_can_set_cfg_test_so_its_package_narrows_nothing`, `the_census_counts_every_file_any_build_of_the_corpus_reads` | — |
| features | a package's default features, unless `--no-default-features` | read: every `cfg` at once; a dependency behind any feature an edge, a non-member's own feature's among them; one on a package no resolve lists refused | `the_census_reads_each_target_whatever_its_configuration`, `a_unit_joining_two_roles_is_a_program_target_whatever_it_names`, `a_non_member_s_dependency_no_resolve_lists_is_refused`, `the_census_counts_every_file_any_build_of_the_corpus_reads` | — |
| platform | a `[target.'cfg(…)'.dependencies]` table applies on its platform | read: every `cfg` at once; a dependency for any platform an edge | `a_unit_joining_two_roles_is_a_program_target_whatever_it_names` | — |
| configuration | `.cargo/config.toml`, `RUSTFLAGS` and `--cfg` reach every unit | refused before any build (the parent's §3), so none applies; a build script's `--cfg` is the build scripts row's | `a_cargo_configuration_on_the_build_s_path_is_a_refusal_and_not_an_error` | — |
| source text | "If the first character in the sequence is U+FEFF (BYTE ORDER MARK), it is removed"; "Each pair of characters U+000D (CR) immediately followed by U+000A (LF) is replaced by a single U+000A (LF)"; "If the remaining sequence begins with the characters #!, the characters up to and including the first U+000A (LF) are removed", but not if they "are followed (ignoring intervening comments or whitespace) by a [ token" (the Rust reference) | read: printable ASCII, a space, a tab, a form feed, a newline and a carriage return where no comment or literal holds them, `\` before a CRLF continuing a literal's line; refused whole: any other byte there, a byte-order mark among them, and a line rustc removes as a shebang, by rustc's own rule, a doc comment ending its look-ahead | `the_census_reads_its_alphabet_alone_and_refuses_what_lies_outside`, `the_census_reads_every_pairing_of_lexical_constructs_as_rustc_does`, `the_census_reads_every_head_a_file_can_open_with_as_rustc_does` | — |
| file inclusion | a module's file "mirrors the logical module path"; "For `path` attributes on modules not inside inline module blocks, the file path is relative to the directory the source file is located"; a `debugger_visualizer` file "a path relative to the source file" (the Rust reference) | read: every file a crate root reaches by `mod`, `#[path]`, the `include!` family and `debugger_visualizer`, each the innermost package's, a module's own file beside every `path` attribute, an inner one refused; every token the census knows can make rustc read a file read whole or refused unless admitted — a macro's module, an assembly directive reading a file, a path out of the tree, through a link or spelt as hosts read apart among them; what an admitted site names outside | `a_unit_joins_by_what_a_dependency_or_a_chain_of_paths_compiles`, `a_file_counts_for_the_innermost_package_holding_it`, `an_inclusion_the_census_cannot_follow_is_refused_unless_admitted`, `a_path_attribute_resolves_as_the_reference_says`, `a_raw_identifier_and_every_mod_are_read_whole_or_refused`, `every_attribute_and_macro_that_reads_a_file_is_read_whole_or_refused`, `a_path_meets_the_links_at_every_component_and_hosts_read_none_apart`, `every_path_is_followed_with_the_module_s_own_file_or_refused`, `the_census_reads_every_inline_module_s_children_where_rustc_does`, `the_census_counts_every_file_any_build_of_the_corpus_reads` | `what the census does not read` |
| macros | a rule's "transcriber, describing the syntax that will replace a successfully matched invocation"; "The ident, lifetime, and tt fragment types are an exception, and can be matched by literal tokens"; a repetition's "separator token can be any token other than a delimiter or one of the repetition operators" (the Rust reference) | read: an inclusion in the arguments of std's macros that keep them in place, named as std's, as it is written; refused unless admitted: in any other macro's arguments every name of a macro that reads a file, invoked or not, a `mod` and a debugger visualizer; in a `macro_rules!` body those, a macro a metavariable names and an attribute one keys or fills; a name of those std macros defined or imported otherwise, `std` renamed, a `macro_use`, a `macro_export`; a doc test's `macro_rules` | `a_site_no_literal_pins_is_unfollowed`, `a_raw_identifier_and_every_mod_are_read_whole_or_refused`, `a_macro_that_may_rearrange_its_arguments_is_read_whole_or_refused`, `the_census_reads_every_macro_spelling_of_a_trigger_as_rustc_does`, `the_census_reads_every_macro_a_name_2018_reserves_bears_as_rustc_does`, `the_census_counts_every_file_any_build_of_the_corpus_reads` | — |
| dependency kinds | normal, build and development dependencies | edges: normal and build, and the unit's own package's development | `a_unit_joining_two_roles_is_a_program_target_whatever_it_names` | — |
| build scripts, procedural macros | each runs code as its package builds; "The rustc-cfg instruction tells Cargo to pass the given value to the --cfg flag to the compiler" | read as any package's sources and never run, a build script's files its package's library's, its package narrowing no `cfg(test)`; what they generate outside | `a_package_with_a_build_script_is_read_and_its_script_never_run`, `a_package_reaching_a_build_script_is_read_as_any_other`, `a_build_script_s_files_count_for_its_package`, `a_build_script_can_set_cfg_test_so_its_package_narrows_nothing` | `what the census does not read` |
| doc tests | "each code block compiles to a doctest executable on the fly with `rustc`" (the Cargo Book's `cargo test`); "Rustdoc also accepts indented code blocks as an alternative to fenced code blocks"; "`compile_fail` tells rustdoc that the compilation should fail" (the rustdoc book); the `doctest` field "is only relevant for libraries" (the Cargo Book's targets); `test(attr(...))`: "This form of the doc attribute allows you to add arbitrary attributes to all your doctests" (the rustdoc book) | the library's files; its doc text read with no markdown, its pieces joined in source order, what a `doc` attribute's `test(…)` adds to every doc test among them, every word that can make a doc test read a file refused unless admitted, but one a backtick opens while the library's doc text names no error code; a binary's or a test's doc text no doc test's | `a_unit_joins_by_what_a_dependency_or_a_chain_of_paths_compiles`, `a_site_no_literal_pins_is_unfollowed`, `every_doc_test_rustdoc_may_compile_is_refused_however_it_is_spelt`, `a_backtick_exempts_prose_unless_a_doc_test_names_an_error_code`, `doc_text_is_read_for_doc_tested_libraries_alone`, `doc_text_is_read_past_every_space_and_joined_so_prose_stays_prose`, `the_census_counts_every_file_any_build_of_the_corpus_reads` | — |
| non-members, other workspaces | neither's tests are this workspace's | a non-member's library, build script and executables read as an edge, its tests, benches and examples unread; another workspace's outside | `the_census_counts_members_tests_alone` | `what the census does not read` |
| a program a test runs | a process, compiled by no unit of the test | outside | — | `a program a test runs` |

**Outside it, stated in the report:** its run-time reads of the kind modules (`std::fs`); and the transcriptions
themselves — the model's sources are in the checker's package and the production relation's, from `M3.4`, in the
generator's, so from then an edit to either moves no shared item and is reported nowhere (before it the relation,
compiled beside the pair's builds, is in the derivation's item); the comparison failing is what holds
them together; whether a classified unit compares what two roles hold equal, which its review decides, and an edit
inside it, reviewed again only when its role packages grow (`M3.6.10`); what a build script or a procedural macro
generates; the file an admitted inclusion site names; a non-member's tests and another workspace's; a program a test
runs as a process; what a test-like unit reads at run time, `std::fs` naming another role's files — a test
compiles none of them (round 12's D9); the file a doc test the backtick exemption excuses still reads; and a channel
the census's rules do not name, or a spelling beyond the families its generated legs check, which the premise claims
nothing of (round 14's R5).

**Its class** (§5), §4.4's **reference derivation.** The comparison holding the two equal is the coupling itself,
not a control: a fault both carry passes it. Controls independent of the transcription: the model's exhaustive
checker for the properties the relation record states, and the falsification corpus written from the record (both
built); F12's hand-written corruptions and accepted plans (`M3.5.4`, `M3.5.5`, *pending*). The frozen verdicts, once
`M3.4` wires the relation into `archogen check`, are change control; the record's 31 review rounds, and amending the
checker's model before the generator's relation, are procedures. The comparison of the checker's fenced joint reading
with `joint::System::read` (`M3.5.5`) compares two readings written apart: a form of its own, its class its review's.

### 4. Invoked programs and installed data

**`(invokes <root> …)` and `(installed "archogen-install.json")`**, together, on the front's form alone — the one
root of role `generator`, whose target is an executable. Read wrong, either fails `read_roots` (exit 2): either clause
on another root's form, or on a front not an executable, or while two roots have role `generator`; either without the
other; an empty clause or a root named twice; an invoked name that is no root, the generator's, a root whose target is
not an executable, the harness or a derivation; `(installed …)` holding more than one name, or a name other than
`archogen-install.json` (`archogen_install::FILE`). An invocation is a shared item of the two roots: kind
`invocation`, identity `<a>+<b> invocation`, the pair in the inventory's order; its content the invoked root's name,
role and target and the sha256 of the launcher, `crates/archogen-cli/src/sibling.rs`, the one file `NO-SUBPROCESS`
is to admit; a front with `(invokes …)` whose commit holds no such blob is `trust-undeclared-input`, decided in step 2 of
the generated-sources record's order, when the tree is read. A process is not a compilation: an invocation is never
`trust-shared-program`. The installed name is no blob and no `(data …)`, so the blob rule of `trust-undeclared-input`
does not reach it. Today the install places the generator's executable as the front and every other executable root
as a sibling (`M3.5.1.3`); from `M3.5.1.4` its siblings are exactly the roots the front's form invokes. The package
records the manifest beside the artifacts it describes, never as a handed dependency (`M4.7`).

**What stays outside, stated in the report:** that the front runs only, or all, the programs its form invokes.
From `M3.5.1.4` — until then no front invokes — `NO-SUBPROCESS` admitting `sibling.rs` alone, declared from the front's `main.rs` (`M3.5.1.4`), the launcher running
only what the manifest names, and the install placing only invoked roots stand there.

### 5. What the roles share

Proposed for each form's review (parent §5): one class, one §7.1 property, the residual risk and the controls;
*pending* names the owner; a package's own tests, change control and procedures are named as such. **The property is
the pair's:** configuration validity for the generator and the configuration checker; timing for the generator and the
scheduling checker; runtime functional behavior for the reference model and the implementation; for any other pair,
none rests on its independence, and its form says so.

| Item | Pairs | Class | Residual risk | Controls |
| --- | --- | --- | --- | --- |
| `crates/eadl-front` | generator, configuration checker | interpretation/normalization | a misreading both make | the third reader (`scripts/third_opinion.sh`); its own conformance suite; the frozen verdicts, change control |
| `crates/eadl-model` | generator, configuration checker | semantic/analysis logic | a wrong quantity, comparison or pass both apply | the profile's classified rules; its own tests; the frozen verdicts, change control |
| the kind modules, two files | generator, configuration checker | authoritative data | a wrong definition both read | `LANGUAGE-FREEZE` and the migration notes, change control |
| `crates/archogen-digest` | every pair it joins | infrastructure | a wrong digest in every role's records | three oracles independent of it: derived constants, published examples, the system tool's boundaries |
| `crates/archogen-outcome` | every pair it joins | interpretation/normalization | a refusal read as an acceptance for every role | its own tests — one text per record, its consistency rules, the damage sweep; the exit code's second channel *pending* `M3.5.1.4` |
| `crates/archogen-json` | every pair it joins | interpretation/normalization | a parsing fault common to every record and server | its own tests — RFC 8259 strictness, a member named twice refused; the records' readers re-write what they read |
| `crates/archogen-transport` | every pair it joins | interpretation/normalization | a framing fault common to every server | *pending* `M3.5.1.6` |
| the relation's derivations, `production` and `vocabulary` | generator, configuration checker | reference derivation | as §3 | as §3 |
| an invocation | the front and each invoked root | interpretation/normalization | the front reporting a refused or unchecked result as checked | *pending* `M3.5.1.4`: the exit code agreeing with the record, the subject's digests, the program's sha256 |
| the build configuration | every new pair | infrastructure | a toolchain, profile or flag fault common to every role | the pinned toolchain (`SOURCE-LEDGER`) and the item compared on the baseline's host, change control |

`crates/archogen-install` is the front's alone; `xtask` is no root. `eadl-front` and `eadl-model` join the scheduling
checker's pairs when it reads descriptions (`M2.7.5`), with that pair's property.

### 6. What else reads with it

Beyond §3's reading rule: a new kind of item, the invocation, is read by the parent's §4–§6 as every shared item is;
the parent's §2 gains this record's forms and clauses and its sentence on `archogen analyze` and `archogen resolve` is
answered; case 4, `trust-inventory-stale` and §7's package contents gain the install manifest as `M4.7` records it;
`TI-H19` is amended in the parent's ledger in this change and `TI-H18` stands. Ledger rows owned elsewhere that the
reading rule extends — `GS-H10`'s acceptance of live `defgenerated` forms among them — are quoted below for their
leaves. Both records state, from `b788ec4e`, that these read with it as `M3.5.2`'s readers land.

## Why

- **Disclosed, not hidden.** A derivation two roles share is the loss of independence §4.4 exists to surface; a form
  of its own puts it before a reviewer with its class, where today the gate sees nothing of it.
- **One way to write a comparison.** A harness and a derivation differ in what they may produce and whom they are
  paired with; a comparison writable as either would take whichever its author found convenient.
- **A process is an input.** The program a root runs is another root's artifact, and the invocation names it.

## How to apply

**A reader before its form.** The gate is built from the base (§1), so a form's reader lands first (`M3.5.2`'s),
reaches `main` through a cadence push, and only a commit of a later push writes the form. **A form from the runner.**
The forms of the shared items a commit adds, or whose measured aspects it moves, are the proposal of a runner run whose
base holds the encoding the judging gate uses — today `62099a39` and, for the new kinds, `3f0a27e0`, decided
by ancestry against the base the report names — committed in a leaf of their own that waits on the runner, writing §5's
row and recording that base. A proposal holds for the item as the run measured it, so the gate on the baseline's host
refuses the item until a push holds the form and nothing after it moves the item; each push's report says which.
**The census `RP-H2` names**, of names, is superseded by §3's, read from the sources (`M3.5.2.2.2`, `M3.5.2.7`).

<!-- machine-read: handoffs -->
| Id | Leaf | Hand-off |
| --- | --- | --- |
| `RP-H1` | `M3.5.3` | The configuration checker's library is its root in `trust/roots.eadl` from the commit that creates `crates/archogen-plan-check`, replacing `(defrole configuration-checker …)`. |
| `RP-H2` | `M3.5.2.1` | The gate reads `defderivation`, the narrowed `defharness`, `(invokes …)` and `(installed …)` as `decision_trust-role-programs.md` §3 and §4 state, reads a derivation as the harness but for its two differences, records and reports both new kinds of item, reports a test naming crates of the generator's and a checker's role packages that no form names, and refuses in `trust-verify` a result whose role is no role of `trust::ROLES` or whose producer is a derivation's artifact; its report's list of what stays outside gains §3's and §4's lines; it is tested on fixtures — each read failure, case 5 inside and outside a derivation's units, a derivation compiling a role package outside its pair, a derivation's target gone, a `#[no_mangle]` and a procedural macro on a derivation's development edge, an unnamed comparison reported, a missing launcher, an invocation moved by a launcher edit, a harness-produced reference-model result accepted, a derivation's and a no-role result refused — the existing pairs of a generator and a checker rewritten; the book's trust chapter says so; and it writes none of these forms. |
| `RP-H3` | `M3.5.4` | Each `include_str!` site of `crates/archogen-plan-check` is admitted in `trust/roots.eadl` by a `defadmit` of its own. |
| `RP-H4` | `M3.5.5` | The comparison of the checker's fenced joint reading with `joint::System::read` is a `defderivation` form of its own, written after its reader reaches `main`, its class written by its review. |
| `RP-H5` | `M3.5.1.4` | The generator's form gains `(invokes configuration-checker)` and `(installed "archogen-install.json")` with the launcher, after `M3.5.6` roots the checker at its executable and in a push after the one that brings `M3.5.2`'s readers, whole, to `main`, a test holding the installed name to `archogen_install::FILE` and the launcher's path to the file `NO-SUBPROCESS` admits, and `cargo xtask install` places the front and, as siblings, exactly the roots its form invokes. |
| `RP-H6` | `M2.21.5` | The generator's form invokes `scheduling-checker` in the commit in which `archogen analyze` runs `archogen-analyze` through the front's launcher, after `M2.21.3` and `M3.5.1.4`, and the forms of the items that commit adds or moves come from the runner as `decision_trust-role-programs.md`'s *How to apply* says, in a leaf of their own. |
| `RP-H7` | `M3.4` | `crates/eadl-resolve` joins the generator's role packages in `trust/roots.eadl` in the commit that makes the generator compile it, after `M3.5.3.1`, which admits its two `include_str!` sites, and a push carrying `M3.5.2`'s readers, whole, each of its test-like units no `defderivation` names classified by a `defprogram`, and the forms of the items its new edges add or move come from the runner as `decision_trust-role-programs.md`'s *How to apply* says, in a leaf of their own. |
| `RP-H8` | `M4.7` | The package records the install manifest of its own trust build beside the artifacts it describes, never as a handed dependency, and `cargo xtask trust-verify` refuses one whose digests are not the inventory's, `TI-H14` amended to say so. |
| `RP-H9` | `M3.5.1.8` | The form of `generator+scheduling-checker package crates/archogen-digest` comes from the runner as `decision_trust-role-programs.md`'s *How to apply* says — the first run whose base holds `62099a39` — writing its §5's class, property, residual risk and controls. |
| `RP-H10` | `M3.5.1.6` | `crates/archogen-transport`'s controls are written into `decision_trust-role-programs.md` §5's row, through that record's review, before a checker compiles it. |
| `RP-H11` | `M3.5.3.1` | `trust/roots.eadl` holds a `defderivation` form for each comparison §1 names — `production` and `vocabulary` in `crates/eadl-resolve` — after `M3.5.2`'s readers, whole, reach `main`, `crates/eadl-resolve`'s two `include_str!` sites admitted. |
| `RP-H12` | `M3.5` | Each leaf of `M3.5` that adds a shared item, or moves one's measured aspects, has its forms come from the runner as `decision_trust-role-programs.md`'s *How to apply* says, in a leaf of their own. |
| `RP-H13` | `M2.21.3` | The forms of the shared items the scheduling checker's executable root adds, or whose measured aspects it moves, come from the runner as `decision_trust-role-programs.md`'s *How to apply* says, in a leaf of their own. |
| `RP-H14` | `M3.5.3.2` | The forms of the shared items `M3.5.3`'s root and `M3.5.3.1`'s derivations add come from the runner as `decision_trust-role-programs.md`'s *How to apply* says, each writing its §5's row. |
| `RP-H15` | `M4.8` | `GS-H10`'s acceptance of the live `defgenerated` forms a root or the comparison harness reaches takes those a derivation reaches, as `decision_trust-role-programs.md` §3 reads a derivation. |
| `RP-H16` | `M3.5` | Each leaf of `M3.5` whose commit makes a test-like unit join two roles a property rests on names it by a pair form or classifies it by a `defprogram` in that commit, after a push carrying `M3.5.2`'s readers, whole. |
| `RP-H17` | `M2.21` | Each leaf of `M2.21` whose commit makes a test-like unit join two roles a property rests on names it by a pair form or classifies it by a `defprogram` in that commit, after a push carrying `M3.5.2`'s readers, whole. |
| `RP-H18` | `M3.5.8` | `trust/roots.eadl` classifies each test-like unit the census of the judging base finds joining two roles a property rests on and no pair form names — today `crates/rt-core`'s `lifecycle`, unit tests and doc tests — after the first push that brings the census to `main` and before the push after it. |
| `RP-H19` | `M3.5.8` | `trust/roots.eadl` admits, by rule `inclusion`, each site of the tree the census of the judging base cannot follow by its text — `crates/rt-analysis/tests/runtime_variant.rs:510`'s `include_str!(concat!(…))`, as `M3.5.2.7.1` measured it — after the first push that brings the census's reading of the sources to `main` and before the push after it. |

## Review

| Round | Date | Defects | Remarks | Verdict |
| --- | --- | --- | --- | --- |
| 1 | `2026-10-23` | 23 | 10 | not acceptable; every finding answered |
| 2 | `2026-10-23` | 14 | 6 | not acceptable; every finding answered |
| 3 | `2026-10-23` | 10 | 10 | not acceptable; every finding answered |
| 4 | `2026-10-23` | 10 | 10 | not acceptable; every finding answered, and the method changed: §6's enumeration replaced by a reading rule complete by construction, a missing comparison found by machine, and from round 5 the record read against `M3.5.2.1`'s code and fixtures |
| 5 | `2026-10-23` | 7 | 10 | not acceptable, read against the code; every finding answered, the census read from the build (`M3.5.2.2`) |
| 6 | `2026-10-23` | 5 | 8 | not acceptable, read against `M3.5.2.2`'s code; every finding answered, the census counting what some build compiles (`M3.5.2.3`) |
| 7 | `2026-10-23` | 2 | 6 | not acceptable, read against `M3.5.2.3`'s code; every finding answered, a literal path into a role package joining (`M3.5.2.4`) |
| 8 | `2026-10-23` | 3 | 10 | not acceptable, read against `M3.5.2.4`'s code; every finding answered, and the method changed: the census read from what rustc compiled (`M3.5.2.5`) |
| 9 | `2026-10-23` | 4 | 7 | not acceptable, read against `M3.5.2.5`'s code; every finding answered, and the method changed again: what decides what a unit compiles a table, each row a fixture against cargo's dep-info (`M3.5.2.6`) |
| 10 | `2026-10-23` | 7 | 8 | not acceptable, read against `M3.5.2.6`'s code; every finding answered, and the method changed a third time: the census independent of every build configuration — metadata's edges and every source's inclusion sites read lexically, default-deny — its soundness held by an oracle over a matrix of builds (`M3.5.2.7`) |
| 11 | `2026-10-23` | 8 | 9 | not acceptable, read against `M3.5.2.7`'s code; every finding answered, and the method changed a fourth time: every token that can make rustc or rustdoc read a file read whole or refused, doc text read with no markdown, the oracle running rustdoc and each unit's closure (`M3.5.2.8`) |
| 12 | `2026-10-24` | 10 | 7 | not acceptable, read against `M3.5.2.8`'s code; every finding answered, and the method changed a fifth time: the census reads only what it can check it reads as rustc does — an alphabet it checks, every path followed with the module's own file or refused, no narrowing a build script can undo — the rest refused, its reading held against rustc by a generated corpus (`M3.5.2.9`) |
| 13 | `2026-10-24` | 5 | 6 | not acceptable, read against `M3.5.2.9`'s code, the count falling; every finding answered, round 12's method carried into the three channels it had not reached — a macro's metavariable, a file's head, an inline module's path — the generated corpus extended to each (`M3.5.2.10`) |
| 14 | `2026-10-24` | 4 | 5 | not acceptable, read against `M3.5.2.10`'s code, the count falling; every finding answered, each a spelling within or beside a named channel — an inline module in a block, a keyword-named macro, a doc test's injected attribute, a non-member's feature — read as rustc reads it or refused, the generated legs extended (`M3.5.2.11`) |
