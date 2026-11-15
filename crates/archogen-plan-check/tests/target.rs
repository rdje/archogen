//! The checker's reading of a target and of a system put on it (leaf `M4.1.7.5.4.2`; `DC-H2`), each expected value
//! derived from `docs/decisions/decision_device-correspondence.md` alone — "What a target's file is", rule 1 and the
//! table "What is refused before any search" — and never from a run: the shipped target reads as one target, the S0
//! heartbeat is put on it, each sentence of what a target's file is refuses by its code, and each row of the table
//! refuses by its code, with the cases the record says are not refused beside them.

use std::collections::BTreeSet;
use std::path::Path;

use archogen_outcome::Status;
use archogen_plan_check::check::{plain, Verdict};
use archogen_plan_check::requesters::{declarations, hardware};
use archogen_plan_check::target::{on_target, OnTarget, Target};
use eadl_front::module::{elaborate, MemoryModules};
use eadl_front::{Form, Program, SourceMap};

/// A file of the repository, read from its root.
fn repository(file: &str) -> String {
    std::fs::read_to_string(
        Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../..")
            .join(file),
    )
    .unwrap_or_else(|e| panic!("{file} reads: {e}"))
}

/// `text` read as a target's file, `target.eadl`, added to `sources`.
fn read_target(sources: &mut SourceMap, text: &str) -> Result<Target, Verdict> {
    let id = sources.add("target.eadl", text).expect("the target fits");
    Target::read(sources, id)
}

/// `text` read as a plain description, `system.eadl`, added to `sources`: one root instance, as the checker reads a
/// description with no `defmodule` (`check::plain`).
fn read_description(sources: &mut SourceMap, text: &str) -> Program {
    let id = sources
        .add("system.eadl", text)
        .expect("the description fits");
    let (document, diagnostics) = eadl_front::read(sources, id);
    assert!(!diagnostics.has_errors(), "{}", diagnostics.render(sources));
    plain(document.declarations().cloned().collect())
}

/// The system `system` of the plain description `description` put on the target `target`, which must read.
fn put(description: &str, target: &str, system: &str) -> Result<OnTarget, Verdict> {
    let mut sources = SourceMap::new();
    let program = read_description(&mut sources, description);
    let target = read_target(&mut sources, target).expect("the target reads");
    on_target(&sources, &program, system, &target)
}

/// A refusal's status and the code of each of its findings.
fn refused<T: std::fmt::Debug>(result: Result<T, Verdict>) -> (Status, Vec<String>) {
    let verdict = result.expect_err("refused");
    let codes = verdict.findings.iter().map(|f| f.code.clone()).collect();
    (verdict.status, codes)
}

/// One refusal of `status` with one finding, `code`.
fn one(status: Status, code: &str) -> (Status, Vec<String>) {
    (status, vec![code.to_owned()])
}

fn names(names: &[&str]) -> BTreeSet<String> {
    names.iter().map(|n| (*n).to_owned()).collect()
}

/// One form, read from `text`.
fn form(text: &str) -> Form {
    let mut sources = SourceMap::new();
    let id = sources.add("expected", text).expect("fits");
    let (mut document, diagnostics) = eadl_front::read(&sources, id);
    assert!(!diagnostics.has_errors(), "{text}");
    document.forms.remove(0)
}

/// The one `platform` clause of the system as it reads on the target.
fn clause(on: &OnTarget) -> Form {
    let read = declarations(&on.program);
    let (_, system) = read[&on.system];
    let clauses: Vec<&Form> = system
        .items()
        .iter()
        .filter(|item| item.head() == Some("platform"))
        .collect();
    assert_eq!(clauses.len(), 1, "{}", system.to_canonical());
    clauses[0].clone()
}

/// Every declaration read on the target, by its qualified name.
fn read_there(on: &OnTarget) -> BTreeSet<String> {
    declarations(&on.program).into_keys().collect()
}

/// A target of one platform composing two blocks: what each target case below changes.
const TARGET: &str = "(eadl-version eadl/1)
(defblock t.timer (offers (tick-rate 10 MHz) (counter-modulus (pow2 64))))
(defblock t.console (offers (observable-output true)))
(defplatform t.soc (offers (core-count 1 tick)) (requires (uses t.timer t.console)))
";

/// Its hardware: every block and platform its file declares.
fn machine() -> BTreeSet<String> {
    names(&["t.soc", "t.timer", "t.console"])
}

/// A description of the S0 heartbeat's shape with `system` as its system and `more` beside its declarations: what
/// each description case below changes.
fn described(more: &str, system: &str) -> String {
    format!(
        "(defblock console.uart (offers (observable-output true)))
(defplatform host.playground (offers (core-count 1 tick)) (requires (uses console.uart)))
(defservice console.write (requires (needs observable-output)))
{more}
{system}"
    )
}

/// The heartbeat itself: a task, a service, and its platform clause on the description's platform.
const HEARTBEAT: &str = "(defsystem heartbeat
  (task beat (period 10 ms) (deadline 10 ms) (priority 1) (uses console.write))
  (requires (uses console.write))
  (platform (uses host.playground)))";

// ── What a target's file is (`Target::read`) ────────────────────────────────────────────────────────────────────

#[test]
fn the_shipped_target_reads_as_one_target_its_language_version_aside() {
    let mut sources = SourceMap::new();
    let target = read_target(&mut sources, &repository("targets/riscv-virt-up.eadl"))
        .expect("`targets/riscv-virt-up.eadl` reads so, unchanged");
    assert_eq!(target.platform, "target.riscv-virt-up");
    assert_eq!(target.declarations.len(), 3);
    assert_eq!(
        target.hardware(),
        names(&["target.riscv-virt-up", "target.timer", "target.console"])
    );
    assert!(target.declares("target.console") && !target.declares("console.uart"));
    assert_eq!(
        target.declaration("target.timer").and_then(Form::head),
        Some("defblock")
    );
}

#[test]
fn a_target_file_that_does_not_read_is_refused_by_the_reader_s_own_codes() {
    let (status, codes) = refused(read_target(&mut SourceMap::new(), "(defblock t.timer"));
    assert_eq!(status, Status::InvalidDescription);
    assert!(
        codes.contains(&"read-unclosed-list".to_owned()),
        "{codes:?}"
    );
}

#[test]
fn a_target_declares_hardware_alone() {
    let target = format!("{TARGET}(defservice t.write (requires (needs observable-output)))");
    assert_eq!(
        refused(read_target(&mut SourceMap::new(), &target)),
        one(Status::InvalidDescription, "invalid-description")
    );
}

#[test]
fn each_declaration_of_a_target_has_a_name_of_its_own() {
    let target = TARGET.replace(
        "(defblock t.console (offers (observable-output true)))",
        "(defblock t.console (offers (observable-output true)))\n(defblock (offers (observable-output true)))",
    );
    assert_eq!(
        refused(read_target(&mut SourceMap::new(), &target)),
        one(Status::InvalidDescription, "schema-missing-name")
    );
}

#[test]
fn a_target_declares_each_name_once() {
    let target = format!("{TARGET}(defblock t.console (offers (observable-output true)))");
    let verdict = read_target(&mut SourceMap::new(), &target).expect_err("refused");
    assert_eq!(verdict.status, Status::InvalidDescription);
    let codes: Vec<&str> = verdict.findings.iter().map(|f| f.code.as_str()).collect();
    assert_eq!(codes, ["schema-duplicate-name"]);
    assert_eq!(
        verdict.findings[0].secondary.len(),
        1,
        "both sites named: {verdict:?}"
    );
}

#[test]
fn a_target_has_one_platform_nothing_else_in_it_composes() {
    // No platform at all.
    let blocks = "(defblock t.timer (offers (tick-rate 10 MHz)))";
    assert_eq!(
        refused(read_target(&mut SourceMap::new(), blocks)),
        one(Status::InvalidDescription, "invalid-description")
    );
    // Two that nothing composes.
    let two = format!("{TARGET}(defplatform t.other (offers (core-count 1 tick)))");
    assert_eq!(
        refused(read_target(&mut SourceMap::new(), &two)),
        one(Status::InvalidDescription, "invalid-description")
    );
    // The only platform composed by another declaration of the file.
    let composed = TARGET.replace(
        "(defblock t.console (offers (observable-output true)))",
        "(defblock t.console (offers (observable-output true)) (uses t.soc))",
    );
    assert_eq!(
        refused(read_target(&mut SourceMap::new(), &composed)),
        one(Status::InvalidDescription, "invalid-description")
    );
}

#[test]
fn the_target_s_platform_composes_every_other_declaration_by_uses() {
    // A block nothing composes.
    let stray = format!("{TARGET}(defblock t.spare (offers (observable-output true)))");
    assert_eq!(
        refused(read_target(&mut SourceMap::new(), &stray)),
        one(Status::InvalidDescription, "invalid-description")
    );
    // A block the platform only `needs`: composition is by `uses`, never `needs`.
    let needed = TARGET.replace(
        "(requires (uses t.timer t.console))",
        "(requires (uses t.timer) (needs t.console))",
    );
    assert_eq!(
        refused(read_target(&mut SourceMap::new(), &needed)),
        one(Status::InvalidDescription, "invalid-description")
    );
    // Composed at any depth: the platform composes a block that composes another.
    let deep = TARGET
        .replace("(uses t.timer t.console)", "(uses t.timer)")
        .replace(
            "(defblock t.timer (offers (tick-rate 10 MHz) (counter-modulus (pow2 64))))",
            "(defblock t.timer (offers (tick-rate 10 MHz) (counter-modulus (pow2 64))) (uses t.console))",
        );
    let target = read_target(&mut SourceMap::new(), &deep).expect("composed at depth");
    assert_eq!(target.platform, "t.soc");
}

#[test]
fn a_target_s_uses_or_needs_holding_a_list_is_refused() {
    let uses = TARGET.replace(
        "(requires (uses t.timer t.console))",
        "(requires (uses t.timer t.console (t.console)))",
    );
    assert_eq!(
        refused(read_target(&mut SourceMap::new(), &uses)),
        one(Status::InvalidDescription, "invalid-description")
    );
    let needs = TARGET.replace(
        "(defblock t.console (offers (observable-output true)))",
        "(defblock t.console (offers (observable-output true)) (needs (tick-rate (at-least 1 MHz))))",
    );
    assert_eq!(
        refused(read_target(&mut SourceMap::new(), &needs)),
        one(Status::InvalidDescription, "invalid-description")
    );
}

#[test]
fn a_target_s_hardware_refining_what_leads_back_to_it_is_refused() {
    // A platform the target's platform composes, refining that platform.
    let looped = TARGET.replace(
        "(requires (uses t.timer t.console))",
        "(requires (uses t.timer t.console t.abstract)))\n(defplatform t.abstract (refines t.soc)",
    );
    assert_eq!(
        refused(read_target(&mut SourceMap::new(), &looped)),
        one(Status::InvalidDescription, "invalid-description")
    );
    // A platform refining itself.
    let itself = TARGET.replace(
        "(defplatform t.soc (offers",
        "(defplatform t.soc (refines t.soc) (offers",
    );
    assert_eq!(
        refused(read_target(&mut SourceMap::new(), &itself)),
        one(Status::InvalidDescription, "invalid-description")
    );
    // A refinement that leads nowhere back is no loop: the platform refines a platform it composes, which names it not.
    let refined = TARGET.replace(
        "(defplatform t.soc (offers (core-count 1 tick)) (requires (uses t.timer t.console)))",
        "(defplatform t.soc (refines t.abstract) (offers (core-count 1 tick)) (requires (uses t.timer t.console t.abstract)))
(defplatform t.abstract (offers (core-count 1 tick)))",
    );
    let target = read_target(&mut SourceMap::new(), &refined).expect("no loop");
    assert_eq!(target.platform, "t.soc");
}

// ── A system on a target (`on_target`), rule 1 ──────────────────────────────────────────────────────────────────

#[test]
fn the_s0_heartbeat_on_the_shipped_target_has_the_target_s_platform_in_its_clause() {
    let mut sources = SourceMap::new();
    let program = read_description(
        &mut sources,
        &repository("examples/s0-heartbeat/system.eadl"),
    );
    let target = read_target(&mut sources, &repository("targets/riscv-virt-up.eadl"))
        .expect("the shipped target reads");
    let on = on_target(&sources, &program, "heartbeat", &target).expect("put on the target");
    let shipped = names(&["target.riscv-virt-up", "target.timer", "target.console"]);
    assert_eq!(on.system, "heartbeat");
    assert_eq!(on.platform, "target.riscv-virt-up");
    assert_eq!(on.hardware, shipped);
    // The description's hardware is gone from what is read, and the target's declarations are added.
    assert_eq!(
        read_there(&on),
        names(&[
            "console.write",
            "heartbeat",
            "target.riscv-virt-up",
            "target.timer",
            "target.console",
        ])
    );
    let clause = clause(&on);
    assert!(
        clause.structurally_eq(&form("(platform (uses target.riscv-virt-up))")),
        "{}",
        clause.to_canonical()
    );
    // Read as any system (model §8 rule 7): its hardware is what that platform composes.
    assert_eq!(
        hardware(&on.program, &declarations(&on.program), "heartbeat"),
        shipped
    );
}

#[test]
fn all_else_a_clause_holds_stays_and_only_its_own_uses_hardware_is_replaced() {
    let system = "(defsystem heartbeat
  (requires (uses console.write))
  (platform (uses host.playground console.write) (requires (core-count (at-least 1 tick)))))";
    let on = put(&described("", system), TARGET, "heartbeat").expect("put on the target");
    let clause = clause(&on);
    assert!(
        clause.structurally_eq(&form(
            "(platform (uses t.soc) (uses console.write) (requires (core-count (at-least 1 tick))))"
        )),
        "{}",
        clause.to_canonical()
    );
    assert_eq!(
        hardware(&on.program, &declarations(&on.program), "heartbeat"),
        machine()
    );
}

#[test]
fn a_clause_is_given_where_the_system_wrote_none() {
    let system = "(defsystem heartbeat (requires (uses console.write)))";
    let on = put(&described("", system), TARGET, "heartbeat").expect("put on the target");
    assert!(clause(&on).structurally_eq(&form("(platform (uses t.soc))")));
    assert_eq!(on.hardware, machine());
    assert_eq!(
        hardware(&on.program, &declarations(&on.program), "heartbeat"),
        machine()
    );
}

#[test]
fn another_system_is_gone_from_what_is_read_but_one_it_names() {
    let more = "(defsystem named (requires (uses console.write)))
(defsystem unnamed (requires (uses console.write)))";
    let system = "(defsystem heartbeat (requires (uses console.write named)) (platform (uses host.playground)))";
    let on = put(&described(more, system), TARGET, "heartbeat").expect("put on the target");
    assert_eq!(
        read_there(&on),
        names(&[
            "console.write",
            "heartbeat",
            "named",
            "t.soc",
            "t.timer",
            "t.console"
        ])
    );
}

#[test]
fn another_system_it_names_stands_on_the_target_as_the_system_does() {
    // The table's third row, `its` the namer's: a system the system names stands on the target as the system does (rule
    // 1), its own clause's hardware replaced by the target's platform, so the hardware that clause's own `uses` names is
    // no name outside it.
    let more =
        "(defsystem logger (requires (uses console.write)) (platform (uses host.playground)))";
    let system = HEARTBEAT.replace(
        "(requires (uses console.write))",
        "(requires (uses console.write logger))",
    );
    let on = put(&described(more, &system), TARGET, "heartbeat").expect("put on the target");
    let read = declarations(&on.program);
    assert_eq!(hardware(&on.program, &read, "heartbeat"), machine());
    let (_, logger) = read["logger"];
    let clauses: Vec<&Form> = logger
        .items()
        .iter()
        .filter(|item| item.head() == Some("platform"))
        .collect();
    assert_eq!(clauses.len(), 1, "{}", logger.to_canonical());
    assert!(
        clauses[0].structurally_eq(&form("(platform (uses t.soc))")),
        "{}",
        logger.to_canonical()
    );
    assert_eq!(hardware(&on.program, &read, "logger"), machine());
}

#[test]
fn a_system_an_imported_module_declares_reads_the_target_s_platform_too() {
    let modules = MemoryModules::new()
        .with(
            "app",
            "(defmodule app (version 1 0) (import parts)
  (defservice probe (requires (needs observable-output))))",
        )
        .with(
            "parts",
            "(defmodule parts (version 1 0) (export sys)
  (defblock console.uart (offers (observable-output true)))
  (defplatform soc (offers (core-count 1 tick)) (requires (uses console.uart)))
  (defservice console.write (requires (needs observable-output)))
  (defsystem logger (requires (uses console.write)) (platform (uses soc)))
  (defsystem sys (requires (uses console.write logger)) (platform (uses soc))))",
        );
    let mut sources = SourceMap::new();
    let (program, diagnostics) = elaborate(&mut sources, &modules, "app");
    assert!(
        !diagnostics.has_errors(),
        "{}",
        diagnostics.render(&sources)
    );
    let target = read_target(&mut sources, TARGET).expect("the target reads");
    let on = on_target(&sources, &program, "parts.sys", &target).expect("put on the target");
    assert_eq!(
        read_there(&on),
        names(&[
            "probe",
            "parts.console.write",
            "parts.logger",
            "parts.sys",
            "t.soc",
            "t.timer",
            "t.console"
        ])
    );
    // The system, and another system it names, each standing on the target's platform from the imported instance.
    for system in ["parts.sys", "parts.logger"] {
        assert_eq!(
            hardware(&on.program, &declarations(&on.program), system),
            machine(),
            "{system}"
        );
    }
}

#[test]
fn a_name_the_description_does_not_declare_as_a_system_is_the_caller_s_failure() {
    assert_eq!(
        refused(put(&described("", HEARTBEAT), TARGET, "nowhere")).0,
        Status::ToolFailure
    );
    assert_eq!(
        refused(put(&described("", HEARTBEAT), TARGET, "console.write")).0,
        Status::ToolFailure
    );
}

// ── What is refused before any search, row by row ───────────────────────────────────────────────────────────────

#[test]
fn a_name_both_the_description_and_the_target_declare_is_refused() {
    let description = described("", HEARTBEAT).replace("console.uart", "t.console");
    let verdict = put(&description, TARGET, "heartbeat").expect_err("refused");
    assert_eq!(verdict.status, Status::InvalidDescription);
    let codes: Vec<&str> = verdict.findings.iter().map(|f| f.code.as_str()).collect();
    assert_eq!(codes, ["schema-duplicate-name"]);
    assert_eq!(verdict.findings[0].secondary.len(), 1, "{verdict:?}");
}

#[test]
fn a_target_naming_what_only_the_description_declares_is_refused() {
    // By `needs`: the target alone reads, a name it does not declare being no declaration of its own.
    let needs = TARGET.replace(
        "(requires (uses t.timer t.console))",
        "(requires (uses t.timer t.console) (needs console.uart))",
    );
    read_target(&mut SourceMap::new(), &needs).expect("the target alone reads");
    assert_eq!(
        refused(put(&described("", HEARTBEAT), &needs, "heartbeat")),
        one(Status::InvalidDescription, "invalid-description")
    );
    // By `refines`.
    let refines = TARGET.replace(
        "(defplatform t.soc (offers",
        "(defplatform t.soc (refines host.playground) (offers",
    );
    read_target(&mut SourceMap::new(), &refines).expect("the target alone reads");
    assert_eq!(
        refused(put(&described("", HEARTBEAT), &refines, "heartbeat")),
        one(Status::InvalidDescription, "invalid-description")
    );
}

#[test]
fn a_description_naming_what_only_the_target_declares_is_refused() {
    let cases = [
        (
            "its hardware, by `needs`",
            described("", HEARTBEAT).replace(
                "(defblock console.uart (offers (observable-output true)))",
                "(defblock console.uart (offers (observable-output true)) (needs t.timer))",
            ),
        ),
        (
            "the system's clause",
            described(
                "",
                &HEARTBEAT.replace("(uses host.playground)", "(uses t.soc)"),
            ),
        ),
        (
            "a service the system does not reach",
            described(
                "(defservice spare.write (requires (needs observable-output)) (uses t.console))",
                HEARTBEAT,
            ),
        ),
        (
            "a refinement",
            described("(defplatform spare.soc (refines t.soc))", HEARTBEAT),
        ),
    ];
    for (case, description) in cases {
        assert_eq!(
            refused(put(&description, TARGET, "heartbeat")),
            one(Status::InfeasibleConfiguration, "infeasible-configuration"),
            "{case}"
        );
    }
}

#[test]
fn what_asks_naming_a_block_or_platform_outside_the_clause_s_own_uses_is_refused() {
    let cases = [
        (
            "a task",
            described(
                "",
                &HEARTBEAT.replace(
                    "(priority 1) (uses console.write)",
                    "(priority 1) (uses console.write console.uart)",
                ),
            ),
        ),
        (
            "the system's own level",
            described(
                "",
                &HEARTBEAT.replace(
                    "(requires (uses console.write))",
                    "(requires (uses console.write) (needs console.uart))",
                ),
            ),
        ),
        (
            "its clause, outside its own `uses`",
            described(
                "",
                &HEARTBEAT.replace(
                    "(platform (uses host.playground))",
                    "(platform (uses host.playground) (needs console.uart))",
                ),
            ),
        ),
        (
            "a service it reaches",
            described("", HEARTBEAT).replace(
                "(defservice console.write (requires (needs observable-output)))",
                "(defservice console.write (requires (needs observable-output)) (uses console.uart))",
            ),
        ),
        (
            "a policy it reaches through a service",
            described(
                "(defpolicy order (requires (needs observable-output)) (needs host.playground))",
                HEARTBEAT,
            )
            .replace(
                "(defservice console.write (requires (needs observable-output)))",
                "(defservice console.write (requires (needs observable-output)) (uses order))",
            ),
        ),
        (
            "another system it names, at that system's own level, outside its clause",
            described(
                "(defsystem other (requires (uses console.write) (needs console.uart)) (platform (uses host.playground)))",
                &HEARTBEAT.replace(
                    "(requires (uses console.write))",
                    "(requires (uses console.write other))",
                ),
            ),
        ),
        (
            "another system it names, in its clause but outside the clause's own `uses`",
            described(
                "(defsystem other (requires (uses console.write)) (platform (uses host.playground) (needs console.uart)))",
                &HEARTBEAT.replace(
                    "(requires (uses console.write))",
                    "(requires (uses console.write other))",
                ),
            ),
        ),
    ];
    for (case, description) in cases {
        assert_eq!(
            refused(put(&description, TARGET, "heartbeat")),
            one(Status::InfeasibleConfiguration, "infeasible-configuration"),
            "{case}"
        );
    }
}

#[test]
fn what_the_system_does_not_reach_neither_asks_nor_answers() {
    let cases = [
        (
            "a service it does not reach",
            described(
                "(defservice spare.write (requires (needs observable-output)) (uses console.uart))",
                HEARTBEAT,
            ),
        ),
        (
            "another system's task, its tasks being its own",
            described(
                "(defsystem other (task tick (period 10 ms) (deadline 10 ms) (priority 1) (uses console.uart)))",
                &HEARTBEAT.replace(
                    "(requires (uses console.write))",
                    "(requires (uses console.write other))",
                ),
            ),
        ),
        (
            "a service reached only through hardware the target replaces",
            described(
                "(defservice probe (requires (needs observable-output)) (uses console.uart))",
                HEARTBEAT,
            )
            .replace(
                "(defblock console.uart (offers (observable-output true)))",
                "(defblock console.uart (offers (observable-output true)) (needs probe))",
            ),
        ),
    ];
    for (case, description) in cases {
        put(&description, TARGET, "heartbeat")
            .unwrap_or_else(|verdict| panic!("{case}: {verdict:?}"));
    }
}

/// The heartbeat whose platform clause names `clause`, beside `more`.
fn standing(more: &str, clause: &str) -> String {
    let system =
        format!("(defsystem heartbeat (requires (uses console.write)) (platform (uses {clause})))");
    format!(
        "(defblock console.uart (offers (observable-output true)))
(defservice console.write (requires (needs observable-output)))
{more}
{system}"
    )
}

#[test]
fn an_absence_of_what_the_clause_stands_on_or_up_its_chain_that_the_target_offers_is_refused() {
    let cases = [
        (
            "the platform the clause stands on",
            standing(
                "(defplatform soc (offers (core-count 1 tick)) (absent tick-rate) (requires (uses console.uart)))",
                "soc",
            ),
        ),
        (
            "a block the clause stands on",
            standing("", "console.uart").replace(
                "(defblock console.uart (offers (observable-output true)))",
                "(defblock console.uart (offers (observable-output true)) (absent tick-rate))",
            ),
        ),
        (
            "one up the chain it refines",
            standing(
                "(defplatform soc.abstract (absent tick-rate))
(defplatform soc (refines soc.abstract) (offers (core-count 1 tick)) (requires (uses console.uart)))",
                "soc",
            ),
        ),
        (
            "two up the chain",
            standing(
                "(defplatform soc.base (absent tick-rate))
(defplatform soc.abstract (refines soc.base))
(defplatform soc (refines soc.abstract) (offers (core-count 1 tick)) (requires (uses console.uart)))",
                "soc",
            ),
        ),
        (
            "a clause naming the platform and what it refines",
            standing(
                "(defplatform soc.abstract (absent tick-rate))
(defplatform soc (refines soc.abstract) (offers (core-count 1 tick)) (requires (uses console.uart)))",
                "soc soc.abstract",
            ),
        ),
        (
            "a cycle of composition, named by one member",
            standing(
                "(defplatform soc (offers (core-count 1 tick)) (requires (uses console.uart mid)))
(defplatform mid (absent tick-rate) (requires (uses soc)))",
                "soc",
            ),
        ),
        (
            "the same cycle, named by the other: no name moves the verdict",
            standing(
                "(defplatform soc (offers (core-count 1 tick)) (requires (uses console.uart mid)))
(defplatform mid (absent tick-rate) (requires (uses soc)))",
                "mid",
            ),
        ),
    ];
    for (case, description) in cases {
        assert_eq!(
            refused(put(&description, TARGET, "heartbeat")),
            one(Status::InfeasibleConfiguration, "refinement-violated"),
            "{case}"
        );
    }
}

#[test]
fn an_absence_that_does_not_speak_for_the_machine_is_not_carried() {
    let cases = [
        (
            "a composed block's",
            standing(
                "(defplatform soc (offers (core-count 1 tick)) (requires (uses console.uart)))",
                "soc",
            )
            .replace(
                "(defblock console.uart (offers (observable-output true)))",
                "(defblock console.uart (offers (observable-output true)) (absent tick-rate))",
            ),
        ),
        (
            "one of two the clause stands on",
            standing(
                "(defplatform soc (offers (core-count 1 tick)) (absent tick-rate) (requires (uses console.uart)))
(defplatform spare.soc (absent tick-rate))",
                "soc spare.soc",
            ),
        ),
        (
            "a platform beside a block it only `needs`, which the system has whichever way the clause is written",
            standing(
                "(defblock spare.timer (offers (counter-width 32 bit)))
(defplatform soc (offers (core-count 1 tick)) (absent tick-rate) (requires (uses console.uart) (needs spare.timer)))",
                "soc",
            ),
        ),
        (
            "a fact the target does not state: silence is not an offer",
            standing(
                "(defplatform soc (offers (core-count 1 tick)) (absent interrupt-source) (requires (uses console.uart)))",
                "soc",
            ),
        ),
        (
            "a derived fact's, meeting only its own name",
            standing(
                "(defplatform soc (offers (core-count 1 tick)) (absent unambiguous-horizon) (requires (uses console.uart)))",
                "soc",
            ),
        ),
        (
            "past what is no hardware: the chain stops there",
            standing(
                "(defservice abstract.write (requires (needs observable-output)) (absent tick-rate))
(defplatform soc (refines abstract.write) (offers (core-count 1 tick)) (requires (uses console.uart)))",
                "soc",
            ),
        ),
    ];
    for (case, description) in cases {
        put(&description, TARGET, "heartbeat")
            .unwrap_or_else(|verdict| panic!("{case}: {verdict:?}"));
    }
}

#[test]
fn two_platform_clauses_on_the_system_are_refused() {
    let system = HEARTBEAT.replace(
        "(platform (uses host.playground))",
        "(platform (uses host.playground)) (platform (uses host.playground))",
    );
    assert_eq!(
        refused(put(&described("", &system), TARGET, "heartbeat")),
        one(Status::InvalidDescription, "schema-cardinality")
    );
}

#[test]
fn hardware_the_system_has_refining_what_leads_back_to_it_is_refused() {
    // Through what they compose and refine: the platform refines one that composes it.
    let looped = described(
        "(defplatform soc.abstract (requires (uses host.playground)))",
        HEARTBEAT,
    )
    .replace(
        "(defplatform host.playground (offers",
        "(defplatform host.playground (refines soc.abstract) (offers",
    );
    assert_eq!(
        refused(put(&looped, TARGET, "heartbeat")),
        one(Status::InvalidDescription, "invalid-description")
    );
    // A platform refining itself.
    let itself = described("", HEARTBEAT).replace(
        "(defplatform host.playground (offers",
        "(defplatform host.playground (refines host.playground) (offers",
    );
    assert_eq!(
        refused(put(&itself, TARGET, "heartbeat")),
        one(Status::InvalidDescription, "invalid-description")
    );
    // A loop no hardware of the system holds is no machine of the system's.
    let elsewhere = described(
        "(defplatform x (refines y))\n(defplatform y (requires (uses x)))",
        HEARTBEAT,
    );
    put(&elsewhere, TARGET, "heartbeat").expect("no loop of the system's");
}

#[test]
fn every_refusal_is_said_under_the_most_severe_status() {
    // A name both declare (`invalid-description`) beside a task naming a block (`infeasible-configuration`).
    let description = described(
        "(defblock t.console (offers (observable-output true)))",
        &HEARTBEAT.replace(
            "(priority 1) (uses console.write)",
            "(priority 1) (uses console.write console.uart)",
        ),
    );
    let (status, mut codes) = refused(put(&description, TARGET, "heartbeat"));
    codes.sort();
    assert_eq!(status, Status::InvalidDescription);
    assert_eq!(codes, ["infeasible-configuration", "schema-duplicate-name"]);
}
