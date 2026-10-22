//! Joint constraints over one complete assignment (`docs/semantics/model.md` §8, leaf `M3.3.2`): F08 and F09 of
//! `ROADMAP.md` §13.1, each refused at every assignment beside controls that hold, and the design's rules one by one.
//!
//! The assignments are enumerated here, every provider the relation satisfies each requirement at, every choice:
//! the search that does this for `archogen check` is `M3.4`'s, so this is a test's exhaustive stand-in for it, on
//! descriptions small enough to list. Expectations are written out literally — providers, requesters, counts and byte
//! ranges — never computed by the code under test.

use eadl_front::{read, Form, SourceMap};
use eadl_model::check::shipped_registry;
use eadl_resolve::joint::{
    requesters, Because, Binding, Finding, Initialization, Interrupt, Joint, Kind, Node,
    Obligation, Report, Requester, System,
};
use eadl_resolve::offer::{self, Provider, State};
use eadl_resolve::relation::{judge, Verdict};
use eadl_resolve::requirement::Requirement;
use eadl_resolve::vocabulary::Vocabulary;

fn root() -> std::path::PathBuf {
    std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(std::path::Path::parent)
        .expect("crates/<name>/ is two levels below the root")
        .to_path_buf()
}

fn vocabulary() -> Vocabulary {
    let kinds: Vec<(String, String)> = [
        "docs/semantics/kinds/core.eadl",
        "docs/semantics/kinds/os-rt.eadl",
    ]
    .iter()
    .map(|r| {
        (
            (*r).to_string(),
            std::fs::read_to_string(root().join(r)).expect("a kind module"),
        )
    })
    .collect();
    let registry = shipped_registry(&mut SourceMap::new(), &kinds).expect("the kinds load");
    Vocabulary::shipped(&mut SourceMap::new(), &registry).expect("the shipped vocabulary reads")
}

fn forms(text: &str) -> Vec<Form> {
    let mut sources = SourceMap::new();
    let id = sources.add("t.eadl", text).expect("small");
    let (doc, diags) = read(&sources, id);
    assert!(!diags.has_errors(), "{text}");
    doc.forms
        .into_iter()
        .filter(|f| f.head() != Some("eadl-version"))
        .collect()
}

fn fixture(name: &str) -> String {
    std::fs::read_to_string(root().join("crates/eadl-resolve/tests/joint").join(name))
        .expect("a fixture")
}

/// A description read: its providers and its requesters.
struct Read {
    decls: Vec<Form>,
    providers: Vec<Provider>,
    requesters: Vec<Requester>,
}

impl Read {
    fn of(text: &str, v: &Vocabulary) -> Self {
        let decls = forms(text);
        let is_provider = |f: &Form| matches!(f.head(), Some("defblock" | "defplatform"));
        let providers = decls
            .iter()
            .filter(|f| is_provider(f))
            .map(|f| offer::read(f, v).unwrap_or_else(|r| panic!("{r:?}")))
            .collect();
        let requesters = decls
            .iter()
            .filter(|f| !is_provider(f))
            .flat_map(|f| requesters(f, v).unwrap_or_else(|r| panic!("{r:?}")))
            .collect();
        Self {
            decls,
            providers,
            requesters,
        }
    }

    fn joint<'v>(&self, v: &'v Vocabulary) -> Result<Joint<'v>, Vec<Finding>> {
        let pairs: Vec<(Provider, &Form)> = self
            .providers
            .iter()
            .map(|p| (p.clone(), self.decl(&p.name)))
            .collect();
        Joint::new(&pairs, &self.requesters, v)
    }

    /// Each system the description declares, as the joint rules read it.
    fn systems(&self) -> Vec<System> {
        self.decls
            .iter()
            .filter(|f| f.head() == Some("defsystem"))
            .map(|f| System::read(f, &self.decls))
            .collect()
    }

    fn decl(&self, name: &str) -> &Form {
        self.decls
            .iter()
            .find(|f| f.items().get(1).and_then(Form::as_symbol) == Some(name))
            .expect("declared")
    }

    /// Every complete assignment: each requirement on a guarantee bound to each provider the relation satisfies it at.
    fn assignments(&self, v: &Vocabulary) -> Vec<Vec<Binding>> {
        self.assignments_of(v, |_| true)
    }

    /// Every complete assignment of the requesters `keep` admits — one system's, for instance.
    fn assignments_of(&self, v: &Vocabulary, keep: impl Fn(&str) -> bool) -> Vec<Vec<Binding>> {
        let mut all: Vec<Vec<Binding>> = vec![Vec::new()];
        for r in self.requesters.iter().filter(|r| keep(&r.name)) {
            for req in &r.requirements {
                if matches!(req, Requirement::Statement { .. }) {
                    continue;
                }
                let at: Vec<&Provider> = self
                    .providers
                    .iter()
                    .filter(|p| judge(p, req, v) == Verdict::Satisfied)
                    .collect();
                assert!(
                    !at.is_empty(),
                    "`{}`'s `{}` is met nowhere",
                    r.name,
                    req.fact()
                );
                all = all
                    .into_iter()
                    .flat_map(|chosen| {
                        at.iter().map(move |p| {
                            let mut next = chosen.clone();
                            next.push(Binding {
                                requester: r.name.clone(),
                                requirement: req.clone(),
                                provider: p.name.clone(),
                            });
                            next
                        })
                    })
                    .collect();
            }
        }
        all
    }
}

/// What a finding names, without its spans: the literal a test writes.
fn named(f: &Finding) -> String {
    let names = |rs: &[(String, eadl_front::Span)]| {
        rs.iter()
            .map(|(n, _)| n.as_str())
            .collect::<Vec<_>>()
            .join(" ")
    };
    match f {
        Finding::OverChannels {
            provider,
            channels,
            requesters,
        } => format!(
            "{} over {provider} ({channels}): {}",
            f.code(),
            names(requesters)
        ),
        Finding::ChannelsUnknown {
            provider,
            requesters,
        } => format!("{} channels of {provider}: {}", f.code(), names(requesters)),
        Finding::Overlap { first, second } => format!(
            "{} {} {}@{:#x}+{:?} / {} {}@{:#x}+{:?}",
            f.code(),
            first.0,
            first.1.name,
            first.1.base,
            first.1.bytes,
            second.0,
            second.1.name,
            second.1.base,
            second.1.bytes
        ),
        Finding::RegionUnsized {
            provider,
            region,
            against,
        } => format!(
            "{} size of {provider} {} against {against}",
            f.code(),
            region.name
        ),
        Finding::ChannelsContradicted { provider, .. } => {
            format!("{} channels of {provider}", f.code())
        }
        Finding::RegionMalformed {
            provider, detail, ..
        } => format!("{} {provider}: {detail}", f.code()),
        Finding::RegionPastArithmetic { provider, .. } => {
            format!("{} region of {provider}", f.code())
        }
        Finding::Unconnected {
            requester,
            provider,
            system,
            ..
        } => format!(
            "{} unconnected {requester} -> {provider} outside {system}",
            f.code()
        ),
        Finding::LevelUnsupported {
            requester,
            stated,
            level,
            ..
        } => format!(
            "{} {requester} states {} at {level}",
            f.code(),
            stated.join(" ")
        ),
        Finding::OutOfLevel {
            requester,
            provider,
            level,
            offered,
            ..
        } => format!(
            "{} out of level {requester} -> {provider} at {level}: {}",
            f.code(),
            offered.join(" ")
        ),
        Finding::MapContradiction {
            system,
            memory,
            other,
        } => format!(
            "{} map {system}: {} {} in {} {}",
            f.code(),
            other.0,
            other.1.name,
            memory.0,
            memory.1.name
        ),
        Finding::MemoryAllHoles { system, memory } => {
            format!(
                "{} all holes {system}: {} {}",
                f.code(),
                memory.0,
                memory.1.name
            )
        }
        Finding::PlacementUnsized {
            system,
            sizeless,
            against,
        } => format!(
            "{} size of {} {} against {against} in {system}",
            f.code(),
            sizeless.0,
            sizeless.1.name
        ),
        Finding::PlacementIllegal {
            system,
            region,
            why,
            ..
        } => {
            format!("{} placement {system} in {region}: {why}", f.code())
        }
        Finding::LevelUnstated {
            requester,
            provider,
            absent,
            ..
        } => format!(
            "{} level {} {requester} -> {provider}",
            f.code(),
            if *absent { "absent" } else { "unstated" }
        ),
    }
}

fn report_of(r: &Report) -> (Vec<String>, Vec<String>) {
    (
        r.conflicts.iter().map(named).collect(),
        r.unknowns.iter().map(named).collect(),
    )
}

/// The reports of every assignment of `text`, judged as its one system's when it declares one (leaf `M3.3.3`), as the
/// checker will judge every assignment.
fn every(text: &str, v: &Vocabulary) -> Vec<Report> {
    let read = Read::of(text, v);
    let mut joint = read
        .joint(v)
        .unwrap_or_else(|f| panic!("{:?}", f.iter().map(named).collect::<Vec<_>>()));
    if let [system] = read.systems().as_slice() {
        joint = joint.for_system(system.clone());
    }
    read.assignments(v).iter().map(|a| joint.check(a)).collect()
}

#[test]
fn f08_two_requests_on_one_single_channel_comparator_are_refused_at_every_assignment() {
    let v = vocabulary();
    let reports = every(&fixture("f08-two-exclusive-requests.eadl"), &v);
    assert_eq!(reports.len(), 1, "one comparator, one assignment");
    for r in &reports {
        assert_eq!(
            report_of(r),
            (
                vec![
                    "infeasible-configuration over timer.compare (1): alarm.engine alarm.watchdog"
                        .to_string()
                ],
                vec![]
            )
        );
        // Leaf `M3.3.3`: both alarms rely on the comparator's interrupt, one route for the plan to enable.
        assert_eq!(
            r.obligations,
            [timer_route(
                "two-alarms",
                "timer.compare",
                &["alarm.engine", "alarm.watchdog"]
            )]
        );
    }
}

/// A timer's route obligation, written out.
fn timer_route(system: &str, provider: &str, requesters: &[&str]) -> Obligation {
    Obligation::Route {
        system: system.to_string(),
        provider: provider.to_string(),
        interrupt: Interrupt::Timer,
        requesters: requesters.iter().map(|r| (*r).to_string()).collect(),
        level: None,
    }
}

#[test]
fn f08_controls_two_channels_one_requester_or_an_undescribed_count() {
    let v = vocabulary();
    let text = fixture("f08-two-exclusive-requests.eadl");
    // Two channels: both requests fit.
    for r in every(&text.replace("(channels 1 tick)", "(channels 2 tick)"), &v) {
        assert!(r.holds(), "{:?}", report_of(&r));
    }
    // One requester: offering the function at all serves one, no count needed.
    let one = text.replace(
        "(defservice alarm.watchdog\n  (requires (needs absolute-deadline)))",
        "(defservice alarm.watchdog\n  (requires (needs core-count)))",
    );
    assert_ne!(one, text);
    for r in every(&one, &v) {
        assert!(r.holds(), "{:?}", report_of(&r));
    }
    // No count written, two requesters: unknown, never accepted.
    for r in every(&text.replace(" (channels 1 tick)", ""), &v) {
        assert_eq!(
            report_of(&r),
            (
                vec![],
                vec![
                    "missing-fact channels of timer.compare: alarm.engine alarm.watchdog"
                        .to_string()
                ]
            )
        );
    }
}

#[test]
fn f08_a_multiplexer_is_one_requester_beneath_and_serves_both_above() {
    // The sharing realization `M3.2` will select, held synthetically: a provider with more channels, itself one
    // requester of the comparator's one channel.
    let v = vocabulary();
    // Leaf `M3.3.3`: the multiplexer is a block, so it is judged connected: the platform composes it.
    let text = fixture("f08-two-exclusive-requests.eadl").replace(
        "(requires (uses timer.compare))",
        "(requires (uses timer.compare alarm.mux))",
    ) + "\n(defblock alarm.mux (offers (absolute-deadline true) (channels 4 tick)))\n";
    let read = Read::of(&text, &v);
    let [system] = read.systems().try_into().expect("one system");
    let joint = read.joint(&v).expect("reads").for_system(system);
    let deadline = read.requesters[0].requirements[0].clone();
    assert_eq!(deadline.fact(), "absolute-deadline");
    let bind = |requester: &str, provider: &str| Binding {
        requester: requester.to_string(),
        requirement: deadline.clone(),
        provider: provider.to_string(),
    };
    let through_mux = [
        bind("alarm.engine", "alarm.mux"),
        bind("alarm.watchdog", "alarm.mux"),
        bind("alarm.mux", "timer.compare"),
    ];
    assert!(joint.check(&through_mux).holds());
    let direct = [
        bind("alarm.engine", "timer.compare"),
        bind("alarm.watchdog", "timer.compare"),
    ];
    assert_eq!(
        report_of(&joint.check(&direct)).0,
        ["infeasible-configuration over timer.compare (1): alarm.engine alarm.watchdog"]
    );
}

#[test]
fn two_tasks_of_one_system_are_two_requesters() {
    // Review D1: the relation reads a system and its tasks as one side, and that must not hide two requests.
    let v = vocabulary();
    // Leaf `M3.3.3`: the system gains the platform its comparator sits on, as a binding must be connected.
    let text = "(defblock timer.compare (offers (absolute-deadline true) (channels 1 tick)))
(defplatform soc (offers (core-count 1 tick)) (requires (uses timer.compare)))
(defsystem s
  (task a (period 10 ms) (deadline 10 ms) (deadline-from release) (priority 1) (needs absolute-deadline))
  (task b (period 20 ms) (deadline 20 ms) (deadline-from release) (priority 2) (needs absolute-deadline))
  (platform (uses soc)))";
    let read = Read::of(text, &v);
    let kinds: Vec<(&str, Kind)> = read
        .requesters
        .iter()
        .map(|r| (r.name.as_str(), r.kind))
        .collect();
    assert_eq!(
        kinds,
        [("s", Kind::Other), ("s/a", Kind::Task), ("s/b", Kind::Task)]
    );
    for r in every(text, &v) {
        assert_eq!(
            report_of(&r).0,
            ["infeasible-configuration over timer.compare (1): s/a s/b"]
        );
    }
}

#[test]
fn a_service_two_tasks_use_is_one_requester_and_owes_the_multiplexing() {
    let v = vocabulary();
    // Leaf `M3.3.3`: the system gains the platform its comparator sits on, and the alarm its route.
    let text = "(defblock timer.compare (offers (absolute-deadline true) (channels 1 tick)))
(defplatform soc (offers (core-count 1 tick)) (requires (uses timer.compare)))
(defservice time.deadline (requires (needs absolute-deadline)))
(defsystem s
  (task a (period 10 ms) (deadline 10 ms) (deadline-from release) (priority 1) (uses time.deadline))
  (task b (period 20 ms) (deadline 20 ms) (deadline-from release) (priority 2) (uses time.deadline))
  (platform (uses soc)))";
    let reports = every(text, &v);
    assert_eq!(reports.len(), 1);
    assert!(reports[0].holds());
    assert_eq!(
        reports[0].obligations,
        [
            Obligation::Multiplex {
                service: "time.deadline".to_string(),
                users: vec!["s/a".to_string(), "s/b".to_string()],
                provider: "timer.compare".to_string(),
            },
            timer_route("s", "timer.compare", &["time.deadline"]),
        ]
    );
}

#[test]
fn only_the_value_true_of_a_consumed_fact_takes_a_channel() {
    // Review D2: requiring the function's absence programs nothing, so it takes no channel.
    let v = vocabulary();
    let text = "(defblock timer.none (offers (relative-delay false)))
(defservice a (requires (relative-delay false)))
(defservice b (requires (relative-delay false)))";
    for r in every(text, &v) {
        assert!(r.holds(), "{:?}", report_of(&r));
    }
    // A shared fact never does, however many read it.
    let read = "(defblock timer.counter (offers (observation-coherent true)))
(defservice a (requires (needs observation-coherent)))
(defservice b (requires (needs observation-coherent)))";
    for r in every(read, &v) {
        assert!(r.holds(), "{:?}", report_of(&r));
    }
}

#[test]
fn a_consumed_function_offered_with_no_channel_is_refused_whether_bound_or_not() {
    let v = vocabulary();
    for channels in ["(absent channels)", "(offers (channels 0 tick))"] {
        let text = format!("(defblock timer.compare (offers (absolute-deadline true)) {channels})");
        let read = Read::of(&text, &v);
        let refused = read.joint(&v).expect_err("refused at the provider");
        assert_eq!(
            refused.iter().map(named).collect::<Vec<_>>(),
            ["invalid-description channels of timer.compare"],
            "{channels}"
        );
    }
    // Absent channels beside the function's absence is no contradiction.
    let text = "(defblock timer.none (offers (relative-delay false)) (absent channels))";
    assert!(Read::of(text, &v).joint(&v).is_ok());
}

#[test]
fn f09_direct_and_firmware_control_of_one_comparator_are_refused_at_every_assignment() {
    let v = vocabulary();
    let reports = every(&fixture("f09-conflicting-ownership.eadl"), &v);
    assert_eq!(reports.len(), 1);
    assert_eq!(
        report_of(&reports[0]),
        (
            vec![
                "infeasible-configuration timer.direct clint.mtimecmp@0x2004000+Some(8) / timer.firmware \
                 sbi.timer-window@0x2000000+Some(65536)"
                    .to_string()
            ],
            vec![]
        )
    );
}

#[test]
fn f09_controls_an_unused_provider_owns_nothing_and_disjoint_ranges_coexist() {
    let v = vocabulary();
    let text = fixture("f09-conflicting-ownership.eadl");
    // Nothing requires the absolute deadline: the firmware block is unused, and costs nothing.
    let unused = text.replace(
        "(defservice time.deadline\n  (requires (needs absolute-deadline)))",
        "(defservice time.deadline\n  (requires (needs core-count)))",
    );
    assert_ne!(unused, text);
    for r in every(&unused, &v) {
        assert!(r.holds(), "{:?}", report_of(&r));
    }
    // The firmware's window moved off the comparator: both bound, no overlap.
    let apart = text.replace(
        "(base 0x0200_0000) (size 64 KiB)",
        "(base 0x0300_0000) (size 64 KiB)",
    );
    assert_ne!(apart, text);
    for r in every(&apart, &v) {
        assert!(r.holds(), "{:?}", report_of(&r));
    }
    // Adjacent ranges share no byte: [0x0200_0000, 0x0200_4000) ends where the comparator begins.
    let adjacent = text.replace(
        "(base 0x0200_0000) (size 64 KiB)",
        "(base 0x0200_0000) (size 16 KiB)",
    );
    for r in every(&adjacent, &v) {
        assert!(r.holds(), "{:?}", report_of(&r));
    }
}

#[test]
fn a_provider_and_one_it_composes_may_share_bytes() {
    // Review remark 4: a platform's window enclosing its own block's registers is not a conflict.
    let v = vocabulary();
    let text = "(defblock timer.direct (offers (relative-delay true) (region clint (base 0x0200_0000) (size 64 KiB))))
(defplatform soc.window (offers (core-count 1 tick) (region mmio (base 0x0200_0000) (size 1 MiB)))
  (requires (uses timer.direct)))
(defservice time.delay (requires (needs relative-delay)))
(defpolicy cores (requires (core-count 1 tick)))";
    for r in every(text, &v) {
        assert!(r.holds(), "{:?}", report_of(&r));
    }
    let uncomposed = text.replace("\n  (requires (uses timer.direct))", "");
    assert_ne!(uncomposed, text);
    for r in every(&uncomposed, &v) {
        assert_eq!(
            report_of(&r).0,
            [
                "infeasible-configuration soc.window mmio@0x2000000+Some(1048576) / timer.direct \
              clint@0x2000000+Some(65536)"
            ]
        );
    }
}

#[test]
fn a_region_without_a_size_is_unknown_only_where_it_could_reach_another() {
    let v = vocabulary();
    let text = |base: &str| {
        format!(
            "(defblock low (offers (relative-delay true) (region low.regs (base {base}))))
(defblock high (offers (absolute-deadline true) (region high.regs (base 0x2000) (size 16 byte))))
(defservice a (requires (needs relative-delay)))
(defservice b (requires (needs absolute-deadline)))"
        )
    };
    // Below the other region: its unknown size could reach it.
    for r in every(&text("0x1000"), &v) {
        assert_eq!(
            report_of(&r).1,
            ["missing-fact size of low low.regs against high"]
        );
        assert!(r.conflicts.is_empty());
    }
    // Above it: nothing at or above its base is another's.
    for r in every(&text("0x3000"), &v) {
        assert!(r.holds(), "{:?}", report_of(&r));
    }
    // At a byte the other owns: a region owns at least its base, so the overlap is known.
    for r in every(&text("0x2008"), &v) {
        assert_eq!(
            report_of(&r).0,
            ["infeasible-configuration high high.regs@0x2000+Some(16) / low low.regs@0x2008+None"]
        );
    }
}

#[test]
fn a_region_that_names_no_byte_range_is_refused() {
    let v = vocabulary();
    let cases = [
        (
            "(region r (size 8 byte))",
            "invalid-description p: region `r` has no base",
        ),
        (
            "(region r (base -1))",
            "invalid-description p: region `r`'s base is a non-negative address",
        ),
        (
            "(region r (base 0x10) (base 0x20))",
            "invalid-description p: region `r` has two bases",
        ),
        (
            "(region r (base 0x10) (size 3 bit))",
            "invalid-description p: region `r`'s size is a positive whole number of bytes",
        ),
        (
            "(region r (base 0x10) (size 4 ms))",
            "invalid-description p: region `r`'s size is a positive whole number of bytes",
        ),
        (
            "(region r (base 0x10) (size 0 byte))",
            "invalid-description p: region `r`'s size is a positive whole number of bytes",
        ),
        (
            "(region (base 0x10))",
            "invalid-description p: a region is named first",
        ),
    ];
    for (region, expected) in cases {
        let text = format!("(defblock p (offers (relative-delay true) {region}))");
        let refused = Read::of(&text, &v).joint(&v).expect_err(region);
        assert_eq!(
            refused.iter().map(named).collect::<Vec<_>>(),
            [expected],
            "{region}"
        );
    }
    // Other sub-clauses are another rule's: `executable` and `reserved` are read past.
    let text = "(defblock p (offers (relative-delay true) (region r (base 0x10) (size 4 byte) (executable true))))";
    assert!(Read::of(text, &v).joint(&v).is_ok());
}

#[test]
fn every_finding_s_diagnostic_carries_its_code() {
    let v = vocabulary();
    let mut seen = Vec::new();
    for text in [
        fixture("f08-two-exclusive-requests.eadl"),
        fixture("f09-conflicting-ownership.eadl"),
        fixture("f08-two-exclusive-requests.eadl").replace(" (channels 1 tick)", ""),
    ] {
        for r in every(&text, &v) {
            for f in r.conflicts.iter().chain(&r.unknowns) {
                assert_eq!(f.diagnostic().code, f.code());
                seen.push(f.code());
            }
        }
    }
    let refused = Read::of(
        "(defblock p (offers (absolute-deadline true)) (absent channels))",
        &v,
    )
    .joint(&v)
    .expect_err("refused");
    assert_eq!(refused[0].diagnostic().code, "invalid-description");
    seen.sort_unstable();
    seen.dedup();
    assert_eq!(seen, ["infeasible-configuration", "missing-fact"]);
}

// Connectivity and routes (leaf `M3.3.3`, model §8 rules 7 and 8): `SR-H15`'s last clause in the checker's form, and the
// interrupts an assignment relies on.

/// `docs/semantics/modules/hw.bus.eadl`'s edges, flattened: a platform naming a block through `needs`, one refining
/// another, and a block no platform composes.
const BUS: &str = "(defplatform bus.main (uses bus.timer) (offers (bus-width 32 bit)))
(defplatform bus.fast (refines bus.main) (needs bus.clock) (offers (bus-width 32 bit)))
(defblock bus.timer (offers (counter-width 32 bit)))
(defblock bus.clock (offers (clock-rate 10 MHz)))
(defblock stray.clock (offers (clock-rate 10 MHz)))
";

/// A system of one task, which writes `task`, and the rest of the system's own clauses, `rest`.
fn one_task_system(task: &str, rest: &str) -> String {
    format!("(defsystem s\n  (task t (period 10 ms) (deadline 10 ms) (deadline-from release) (priority 1){task}){rest})")
}

fn hardware(text: &str, v: &Vocabulary) -> Vec<String> {
    let [system] = Read::of(text, v).systems().try_into().expect("one system");
    system.hardware.into_iter().collect()
}

/// Every report of `text`'s assignments as literals, sorted, so a test lists what it expects whatever the order.
fn every_named(text: &str, v: &Vocabulary) -> Vec<(Vec<String>, Vec<String>)> {
    let mut seen: Vec<_> = every(text, v).iter().map(report_of).collect();
    seen.sort();
    seen
}

#[test]
fn a_system_s_hardware_follows_uses_and_needs_and_never_refines() {
    // Review D1: a platform's `needs` is an edge, `refines` is none, and a system's own `uses` of hardware is one.
    let v = vocabulary();
    let with = |platform: &str, own: &str| {
        format!(
            "{BUS}{}",
            one_task_system("", &format!("{own}\n  (platform (uses {platform}))"))
        )
    };
    assert_eq!(
        hardware(&with("bus.fast", ""), &v),
        ["bus.clock", "bus.fast"]
    );
    assert_eq!(
        hardware(&with("bus.main", ""), &v),
        ["bus.main", "bus.timer"]
    );
    assert_eq!(
        hardware(&with("bus.fast", "\n  (requires (uses bus.timer))"), &v),
        ["bus.clock", "bus.fast", "bus.timer"]
    );
    assert!(hardware(&format!("{BUS}{}", one_task_system("", "")), &v).is_empty());
}

#[test]
fn a_block_outside_the_system_s_hardware_is_refused_beside_one_inside() {
    let v = vocabulary();
    let text = format!(
        "{BUS}{}",
        one_task_system(" (needs clock-rate)", "\n  (platform (uses bus.fast))")
    );
    assert_eq!(
        every_named(&text, &v),
        [
            (vec![], vec![]),
            (
                vec![
                    "infeasible-configuration unconnected s/t -> stray.clock outside s".to_string()
                ],
                vec![]
            ),
        ]
    );
}

#[test]
fn a_system_with_no_hardware_binds_no_block() {
    let v = vocabulary();
    let text = format!("{BUS}{}", one_task_system(" (needs clock-rate)", ""));
    assert_eq!(
        every_named(&text, &v),
        [
            (
                vec!["infeasible-configuration unconnected s/t -> bus.clock outside s".to_string()],
                vec![]
            ),
            (
                vec![
                    "infeasible-configuration unconnected s/t -> stray.clock outside s".to_string()
                ],
                vec![]
            ),
        ]
    );
}

#[test]
fn two_systems_are_judged_apart_each_against_its_own_board() {
    // Review D3: an assignment is one system's, so two boards with a timer at one address never meet.
    let v = vocabulary();
    let text = "(defblock timer.a (offers (counter-width 32 bit) (region clint.a (base 0x0200_0000) (size 64 KiB))))
(defblock timer.b (offers (counter-width 32 bit) (region clint.b (base 0x0200_0000) (size 64 KiB))))
(defplatform board.a (offers (core-count 1 tick)) (requires (uses timer.a)))
(defplatform board.b (offers (core-count 1 tick)) (requires (uses timer.b)))
(defsystem sa (task t (period 10 ms) (deadline 10 ms) (deadline-from release) (priority 1) (needs counter-width))
  (platform (uses board.a)))
(defsystem sb (task t (period 10 ms) (deadline 10 ms) (deadline-from release) (priority 1) (needs counter-width))
  (platform (uses board.b)))";
    let read = Read::of(text, &v);
    let joint = read.joint(&v).expect("reads");
    let systems = read.systems();
    assert_eq!(systems.len(), 2);
    for (system, other) in systems.iter().zip(["timer.b", "timer.a"]) {
        let judge = joint.clone().for_system(system.clone());
        let mut seen: Vec<_> = read
            .assignments_of(&v, |r| system.serves(r))
            .iter()
            .map(|a| report_of(&judge.check(a)))
            .collect();
        seen.sort();
        let refused = format!(
            "infeasible-configuration unconnected {0}/t -> {other} outside {0}",
            system.name
        );
        assert_eq!(seen, [(vec![], vec![]), (vec![refused], vec![])]);
    }
    // Judged as one assignment, the two boards' timers would overlap: why an assignment is one system's.
    let need = |name: &str| {
        read.requesters
            .iter()
            .find(|r| r.name == name)
            .expect("a requester")
            .requirements[0]
            .clone()
    };
    let both = [
        Binding {
            requester: "sa/t".to_string(),
            requirement: need("sa/t"),
            provider: "timer.a".to_string(),
        },
        Binding {
            requester: "sb/t".to_string(),
            requirement: need("sb/t"),
            provider: "timer.b".to_string(),
        },
    ];
    assert_eq!(
        report_of(&joint.check(&both)).0,
        ["infeasible-configuration timer.a clint.a@0x2000000+Some(65536) / timer.b clint.b@0x2000000+Some(65536)"]
    );
}

#[test]
fn a_block_may_rely_on_a_sibling_its_platform_composes() {
    let v = vocabulary();
    let text = format!(
        "(defblock x (offers (counter-width 32 bit)) (requires (needs clock-rate)))
(defblock y (offers (clock-rate 10 MHz)))
(defblock stray (offers (clock-rate 10 MHz)))
(defplatform soc (offers (core-count 1 tick)) (requires (uses x y)))
{}",
        one_task_system("", "\n  (platform (uses soc))")
    );
    let read = Read::of(&text, &v);
    let [system] = read.systems().try_into().expect("one system");
    let joint = read.joint(&v).expect("reads").for_system(system);
    let [x] = requesters(read.decl("x"), &v)
        .expect("the block reads")
        .try_into()
        .expect("one requester");
    let bind = |provider: &str| {
        [Binding {
            requester: x.name.clone(),
            requirement: x.requirements[0].clone(),
            provider: provider.to_string(),
        }]
    };
    assert!(joint.check(&bind("y")).holds());
    assert_eq!(
        report_of(&joint.check(&bind("stray"))).0,
        ["infeasible-configuration unconnected x -> stray outside s"]
    );
}

#[test]
fn a_requester_no_system_reaches_is_judged_for_channels_alone() {
    // Review R11: connectivity and routes say nothing of it; the channel count still does.
    let v = vocabulary();
    let text = format!(
        "(defblock stray (offers (absolute-deadline true) (channels 1 tick)))
(defplatform soc (offers (core-count 1 tick)))
(defservice orphan (requires (needs absolute-deadline)))
(defservice orphan.two (requires (needs absolute-deadline)))
{}",
        one_task_system("", "\n  (platform (uses soc))")
    );
    let reports = every(&text, &v);
    assert_eq!(reports.len(), 1);
    assert_eq!(
        report_of(&reports[0]),
        (
            vec!["infeasible-configuration over stray (1): orphan orphan.two".to_string()],
            vec![]
        )
    );
    assert!(reports[0].obligations.is_empty());
}

#[test]
fn each_source_and_each_timer_an_assignment_relies_on_is_a_route_obligation() {
    let v = vocabulary();
    // `docs/semantics/cases/positive-sporadic-release.eadl`'s source, platform and system, its service's
    // `bounded-arrival` left out: no block offers it, the realization does (`M3.2`, `M3.7`).
    let sporadic = |offers: &str, needs: &str| {
        format!(
            "(defblock event.line (offers {offers}))
(defplatform soc.p (offers (core-count 1 tick)) (requires (uses event.line)))
(defservice event.release (requires (needs {needs})))
(defsystem s
  (task handler (min-separation 20 ms) (deadline 15 ms) (deadline-from release) (jitter 1 ms)
                (priority 1) (uses event.release))
  (requires (uses event.release))
  (platform (uses soc.p)))"
        )
    };
    let source = |text: &str| {
        let reports = every(text, &v);
        assert_eq!(reports.len(), 1);
        assert!(reports[0].holds(), "{:?}", report_of(&reports[0]));
        match reports[0].obligations.as_slice() {
            [Obligation::Route {
                system,
                provider,
                interrupt: Interrupt::Source { separation },
                requesters,
                level: None,
            }] => {
                assert_eq!(
                    (system.as_str(), provider.as_str(), requesters.as_slice()),
                    ("s", "event.line", &["event.release".to_string()][..])
                );
                separation.clone()
            }
            other => panic!("one source's route, not {other:?}"),
        }
    };
    // As the case writes it: the separation offered without a value.
    assert_eq!(
        source(&sporadic(
            "interrupt-source min-arrival-separation",
            "interrupt-source min-arrival-separation"
        )),
        Some(State::Unvalued)
    );
    // A source that states no separation of its own: the analysis must find one elsewhere, or refuse.
    assert_eq!(
        source(&sporadic("interrupt-source", "interrupt-source")),
        None
    );
    // F09's two timers: one route each.
    let reports = every(&fixture("f09-conflicting-ownership.eadl"), &v);
    assert_eq!(reports.len(), 1);
    assert_eq!(
        reports[0].obligations,
        [
            timer_route("delays-and-deadlines", "timer.direct", &["time.delay"]),
            timer_route("delays-and-deadlines", "timer.firmware", &["time.deadline"]),
        ]
    );
}

#[test]
fn an_unconnected_binding_s_diagnostic_names_both_ends_and_the_repair() {
    let v = vocabulary();
    let text = format!("{BUS}{}", one_task_system(" (needs clock-rate)", ""));
    let report = every(&text, &v)
        .into_iter()
        .find(|r| !r.conflicts.is_empty())
        .expect("a conflict");
    let d = report.conflicts[0].diagnostic();
    assert_eq!(d.code, "infeasible-configuration");
    assert!(
        d.message.contains("not part of `s`'s hardware"),
        "{}",
        d.message
    );
    assert!(d.repair.contains("compose"), "{}", d.repair);
    assert_eq!(
        d.secondary.len(),
        1,
        "the provider's declaration, outside the hardware"
    );
}

// Privilege (leaf `M3.3.4.1`, model §8 rule 9): `SR-H11` and `SR-H12` over one system's callers, given its level.

/// The reports of every assignment of `text`'s one system, judged at `level`.
fn every_at(text: &str, level: &str, v: &Vocabulary) -> Vec<(Vec<String>, Vec<String>)> {
    let read = Read::of(text, v);
    let [system] = read.systems().try_into().expect("one system");
    let joint = read
        .joint(v)
        .expect("reads")
        .for_system(system.at(level, v).expect("a level of the domain"));
    let mut seen: Vec<_> = read
        .assignments(v)
        .iter()
        .map(|a| report_of(&joint.check(a)))
        .collect();
    seen.sort();
    seen
}

/// Two timers, one reachable at `machine` and one at `supervisor`, on a platform reachable at `machine`, and a task
/// needing a counter.
const LEVELS: &str = "(defblock timer.m (offers (counter-width 32 bit) (reachable-at-privilege machine)))
(defblock timer.s (offers (counter-width 32 bit) (reachable-at-privilege supervisor)))
(defplatform soc (offers (core-count 1 tick) (reachable-at-privilege machine)) (requires (uses timer.m timer.s)))
(defsystem s
  (task t (period 10 ms) (deadline 10 ms) (deadline-from release) (priority 1) (needs counter-width))
  (platform (uses soc)))";

#[test]
fn a_caller_is_bound_only_to_a_provider_reachable_at_its_system_s_level() {
    let v = vocabulary();
    assert_eq!(
        every_at(LEVELS, "machine", &v),
        [
            (vec![], vec![]),
            (
                vec![
                    "infeasible-configuration out of level s/t -> timer.s at machine: supervisor"
                        .to_string()
                ],
                vec![]
            ),
        ]
    );
    // Review D10: with no level given, the rule says nothing, so the earlier tests' controls keep holding.
    for r in every(LEVELS, &v) {
        assert!(r.holds(), "{:?}", report_of(&r));
    }
}

#[test]
fn a_provider_that_states_no_level_is_refused_for_a_caller() {
    // `SR-H12`: undescribed or bare is unknown, declared absent a conflict — the codes the relation gives a requirer
    // that states a level (`SR-H18`). At `archogen check`, model §2 rule 1 would refuse an absent level beside an offered
    // one first, description-wide (review R12); these rules are judged on their own here.
    let v = vocabulary();
    let with = |timer: &str| {
        format!(
            "(defblock timer {timer})
(defplatform soc (offers (core-count 1 tick) (reachable-at-privilege machine)) (requires (uses timer)))
(defsystem s
  (task t (period 10 ms) (deadline 10 ms) (deadline-from release) (priority 1) (needs counter-width))
  (platform (uses soc)))"
        )
    };
    let one = |timer: &str| {
        let seen = every_at(&with(timer), "machine", &v);
        assert_eq!(seen.len(), 1);
        seen[0].clone()
    };
    assert_eq!(
        one("(offers (counter-width 32 bit))"),
        (
            vec![],
            vec!["missing-fact level unstated s/t -> timer".to_string()]
        )
    );
    assert_eq!(
        one("(offers (counter-width 32 bit) reachable-at-privilege)"),
        (
            vec![],
            vec!["missing-fact level unstated s/t -> timer".to_string()]
        )
    );
    assert_eq!(
        one("(offers (counter-width 32 bit)) (absent reachable-at-privilege)"),
        (
            vec!["infeasible-configuration level absent s/t -> timer".to_string()],
            vec![]
        )
    );
    assert_eq!(
        one("(offers (counter-width 32 bit) (reachable-at-privilege machine supervisor))"),
        (vec![], vec![])
    );
}

#[test]
fn a_caller_stating_another_level_is_unsupported_and_judged_first() {
    // Review D3: `docs/semantics/boundary/accept/access-authority.eadl`'s service calls from `supervisor`; under this
    // profile its system runs callers at `machine` alone, so whatever the provider offers, no context at `supervisor`
    // exists, and nothing else is said of that caller.
    let v = vocabulary();
    let with = |offered: &str| {
        format!(
            "(defblock timer (offers (counter-width 32 bit) (reachable-at-privilege {offered})))
(defplatform soc (offers (core-count 1 tick) (reachable-at-privilege machine)) (requires (uses timer)))
(defservice time.deadline (requires (reachable-at-privilege supervisor) (needs counter-width)))
(defsystem s
  (task t (period 10 ms) (deadline 10 ms) (deadline-from release) (priority 1) (uses time.deadline))
  (platform (uses soc)))"
        )
    };
    let refused = (
        vec!["unsupported-profile time.deadline states supervisor at machine".to_string()],
        vec![],
    );
    assert_eq!(
        every_at(&with("supervisor machine"), "machine", &v),
        std::slice::from_ref(&refused)
    );
    assert_eq!(every_at(&with("supervisor"), "machine", &v), [refused]);
    // Stating the system's own level asks nothing more of it: met at the timer or at the platform, both hold.
    assert_eq!(
        every_at(
            &with("supervisor machine").replace(
                "(reachable-at-privilege supervisor) (needs",
                "(reachable-at-privilege machine) (needs"
            ),
            "machine",
            &v
        ),
        [(vec![], vec![]), (vec![], vec![])]
    );
}

#[test]
fn hardware_needing_hardware_has_no_level() {
    // Review D4: a block relying on a sibling's fact is no caller; the sibling states no level and the binding holds.
    let v = vocabulary();
    let text = format!(
        "(defblock x (offers (counter-width 32 bit) (reachable-at-privilege machine)) (requires (needs clock-rate)))
(defblock y (offers (clock-rate 10 MHz)))
(defplatform soc (offers (core-count 1 tick) (reachable-at-privilege machine)) (requires (uses x y)))
{}",
        one_task_system("", "\n  (platform (uses soc))")
    );
    let read = Read::of(&text, &v);
    let [system] = read.systems().try_into().expect("one system");
    let joint = read
        .joint(&v)
        .expect("reads")
        .for_system(system.at("machine", &v).expect("a level"));
    let [x] = requesters(read.decl("x"), &v)
        .expect("the block reads")
        .try_into()
        .expect("one requester");
    let to_y = [Binding {
        requester: x.name.clone(),
        requirement: x.requirements[0].clone(),
        provider: "y".to_string(),
    }];
    assert!(joint.check(&to_y).holds());
}

#[test]
fn a_level_outside_the_domain_is_refused_where_it_is_given() {
    let v = vocabulary();
    let read = Read::of(LEVELS, &v);
    let [system] = read.systems().try_into().expect("one system");
    let refused = system.clone().at("kernel", &v).expect_err("no such level");
    assert!(refused.contains("`kernel` is no level"), "{refused}");
    assert_eq!(
        system.at("machine", &v).expect("a level").level.as_deref(),
        Some("machine")
    );
}

#[test]
fn a_route_carries_the_level_its_interrupt_is_delivered_at() {
    // Review D9: `M3.3.3` handed this here; a source routed to the core is delivered at the system's level.
    let v = vocabulary();
    let text = "(defblock event.line (offers interrupt-source (reachable-at-privilege machine)))
(defplatform soc.p (offers (core-count 1 tick) (reachable-at-privilege machine)) (requires (uses event.line)))
(defservice event.release (requires (needs interrupt-source)))
(defsystem s
  (task handler (min-separation 20 ms) (deadline 15 ms) (deadline-from release) (jitter 1 ms)
                (priority 1) (uses event.release))
  (platform (uses soc.p)))";
    let read = Read::of(text, &v);
    let [system] = read.systems().try_into().expect("one system");
    let joint = read
        .joint(&v)
        .expect("reads")
        .for_system(system.at("machine", &v).expect("a level"));
    let [assignment] = read.assignments(&v).try_into().expect("one assignment");
    let report = joint.check(&assignment);
    assert!(report.holds(), "{:?}", report_of(&report));
    assert_eq!(
        report.obligations,
        [Obligation::Route {
            system: "s".to_string(),
            provider: "event.line".to_string(),
            interrupt: Interrupt::Source { separation: None },
            requesters: vec!["event.release".to_string()],
            level: Some("machine".to_string()),
        }]
    );
}

#[test]
fn every_privilege_finding_s_diagnostic_carries_its_code() {
    let v = vocabulary();
    let read = Read::of(LEVELS, &v);
    let [system] = read.systems().try_into().expect("one system");
    let joint = read
        .joint(&v)
        .expect("reads")
        .for_system(system.at("machine", &v).expect("a level"));
    let mut codes = Vec::new();
    for a in read.assignments(&v) {
        for f in joint.check(&a).conflicts {
            let d = f.diagnostic();
            assert_eq!(d.code, f.code());
            assert!(
                d.message.contains("reachable from `supervisor` alone"),
                "{}",
                d.message
            );
            codes.push(d.code);
        }
    }
    assert_eq!(codes, ["infeasible-configuration"]);
}

#[test]
fn the_profile_s_level_is_one_a_system_can_be_given() {
    // Leaf `M3.3.4.2`: the level is the profile's Privilege row, a level of `reachable-at-privilege`'s domain, so the
    // search can give it to every system it resolves.
    let v = vocabulary();
    let read = Read::of(LEVELS, &v);
    let [system] = read.systems().try_into().expect("one system");
    let level = eadl_model::RT_STATIC_UP_V1.caller_level;
    assert_eq!(level, "machine");
    let at = system
        .at(level, &v)
        .expect("the profile's level is the vocabulary's");
    assert_eq!(at.level.as_deref(), Some("machine"));
}

// Placement (leaf `M3.3.5.1`, model §8 rule 10): one system's memory map, judged once.

/// A platform with 128 MiB of executable RAM at `0x8000_0000`, a device and a firmware block written by `devices`, and
/// a system on it; `extra` adds declarations.
fn memory_map(devices: &str) -> String {
    format!(
        "{devices}
(defplatform soc (offers (core-count 1 tick) (region ram (base 0x8000_0000) (size 128 MiB) (executable true)))
  (requires (uses dev fw)))
{}",
        one_task_system("", "\n  (platform (uses soc))")
    )
}

/// The map of `text`'s one system, as literals: refusals, unknowns, and the placement obligations.
fn map_of(text: &str, v: &Vocabulary) -> (Vec<String>, Vec<String>, Vec<Obligation>) {
    let read = Read::of(text, v);
    let [system] = read.systems().try_into().expect("one system");
    let map = read
        .joint(v)
        .unwrap_or_else(|f| panic!("{:?}", f.iter().map(named).collect::<Vec<_>>()))
        .map(&system);
    (
        map.refused.iter().map(named).collect(),
        map.unknowns.iter().map(named).collect(),
        map.obligations,
    )
}

fn placement(region: &str, holes: &[(&str, &str)]) -> Obligation {
    Obligation::Placement {
        system: "s".to_string(),
        provider: "soc".to_string(),
        region: region.to_string(),
        holes: holes
            .iter()
            .map(|(o, r)| ((*o).to_string(), (*r).to_string()))
            .collect(),
    }
}

const DEV_OUTSIDE: &str = "(defblock dev (offers (observable-output true) (region dev.regs (base 0x1000_0000) (size 4 KiB))))";
const FW_HOLE: &str = "(defblock fw (offers (core-count 1 tick) (region fw.range (base 0x8000_0000) (size 2 MiB) (reserved true))))";

#[test]
fn a_reserved_range_in_memory_is_a_hole_the_plan_avoids() {
    // Review D1: firmware below an S-mode image, a carve-out — `addressable-region.eadl`'s "constrains placement".
    let v = vocabulary();
    assert_eq!(
        map_of(&memory_map(&format!("{DEV_OUTSIDE}\n{FW_HOLE}")), &v),
        (
            vec![],
            vec![],
            vec![placement("ram", &[("fw", "fw.range")])]
        )
    );
}

#[test]
fn a_device_inside_memory_is_a_contradictory_map_once_per_system() {
    // Review D3: two claims on the same bytes, judged over the hardware, bound or not and composed or not.
    let v = vocabulary();
    let inside = "(defblock dev (offers (observable-output true) (region dev.regs (base 0x8100_0000) (size 4 KiB))))";
    let (refused, unknowns, _) = map_of(&memory_map(&format!("{inside}\n{FW_HOLE}")), &v);
    assert_eq!(
        refused,
        ["invalid-description map s: dev dev.regs in soc ram"]
    );
    assert!(unknowns.is_empty());
    // Straddling memory's end is overlapping it too (review D4).
    let straddle = "(defblock dev (offers (observable-output true) (region dev.regs (base 0x87FF_F000) (size 8 KiB))))";
    assert_eq!(
        map_of(&memory_map(&format!("{straddle}\n{FW_HOLE}")), &v).0,
        ["invalid-description map s: dev dev.regs in soc ram"]
    );
}

#[test]
fn a_region_both_executable_and_reserved_is_no_placement_memory() {
    // Review D2.
    let v = vocabulary();
    let fw = "(defblock fw (offers (core-count 1 tick) (region fw.range (base 0x9000_0000) (size 2 MiB) (executable true) (reserved true))))";
    let (refused, unknowns, obligations) = map_of(&memory_map(&format!("{DEV_OUTSIDE}\n{fw}")), &v);
    assert!(refused.is_empty() && unknowns.is_empty());
    assert_eq!(
        obligations,
        [placement("ram", &[])],
        "only `ram` is placement memory"
    );
}

#[test]
fn a_size_the_map_needs_is_unknown_either_way() {
    // Review D4: rule 5 mirrored — memory unsized below a device, and a device unsized below memory.
    let v = vocabulary();
    let sizeless_ram = memory_map(&format!("{DEV_OUTSIDE}\n{FW_HOLE}")).replace(
        "(region ram (base 0x8000_0000) (size 128 MiB) (executable true))",
        "(region ram (base 0x0800_0000) (executable true))",
    );
    assert_eq!(
        map_of(&sizeless_ram, &v).1,
        [
            "missing-fact size of soc ram against dev in s",
            "missing-fact size of soc ram against fw in s"
        ]
    );
    let sizeless_dev =
        "(defblock dev (offers (observable-output true) (region dev.regs (base 0x7FFF_F000))))";
    assert_eq!(
        map_of(&memory_map(&format!("{sizeless_dev}\n{FW_HOLE}")), &v).1,
        ["missing-fact size of dev dev.regs against soc in s"]
    );
}

#[test]
fn two_placement_regions_on_the_same_bytes_contradict_and_memory_all_holes_is_refused() {
    let v = vocabulary();
    let ram2 = "(defblock fw (offers (core-count 1 tick) (region ram2 (base 0x8400_0000) (size 128 MiB) (executable true))))";
    assert_eq!(
        map_of(&memory_map(&format!("{DEV_OUTSIDE}\n{ram2}")), &v).0,
        // Neither is privileged: the map walks the hardware in name order, `fw` before `soc`.
        ["invalid-description map s: soc ram in fw ram2"]
    );
    let all = "(defblock fw (offers (core-count 1 tick) (region fw.range (base 0x8000_0000) (size 128 MiB) (reserved true))))";
    assert_eq!(
        map_of(&memory_map(&format!("{DEV_OUTSIDE}\n{all}")), &v).0,
        ["invalid-description all holes s: soc ram"]
    );
}

#[test]
fn a_region_s_sub_forms_follow_one_grammar() {
    // Review R7: each sub-form at most once, a flag `true` or `false`, nothing else — refused where providers are read.
    let v = vocabulary();
    for (written, refused) in [
        (
            "(executable yes)",
            "region `r`'s `executable` is `true` or `false`",
        ),
        (
            "(reserved true) (reserved true)",
            "region `r` has two `reserved`s",
        ),
        (
            "(alias other)",
            "region `r` has a sub-form `alias` no rule reads",
        ),
    ] {
        let text = format!("(defblock b (offers (region r (base 0x1000) (size 4 KiB) {written})))");
        let found = Read::of(&text, &v).joint(&v).expect_err(written);
        assert_eq!(
            found.iter().map(named).collect::<Vec<_>>(),
            [format!("invalid-description b: {refused}")]
        );
    }
}

#[test]
fn a_placement_given_to_the_plan_is_placement_memory_of_the_system() {
    // Review R9: M3 chooses the region, `M4.1` the offsets.
    let v = vocabulary();
    let text = memory_map(&format!("{DEV_OUTSIDE}\n{FW_HOLE}"));
    let read = Read::of(&text, &v);
    let [system] = read.systems().try_into().expect("one system");
    let joint = read.joint(&v).expect("reads");
    let check = |region: &str| {
        report_of(
            &joint
                .clone()
                .for_system(system.clone().placed_in(region))
                .check(&[]),
        )
        .0
    };
    assert!(check("ram").is_empty());
    assert_eq!(check("dev.regs"), ["infeasible-configuration placement s in dev.regs: it is not executable, unreserved memory"]);
    assert_eq!(check("fw.range"), ["infeasible-configuration placement s in fw.range: it is not executable, unreserved memory"]);
    assert_eq!(check("nowhere"), ["infeasible-configuration placement s in nowhere: it is no region of the system's hardware"]);
}

#[test]
fn the_target_s_map_holds_with_its_ram_as_placement_memory() {
    let v = vocabulary();
    let target =
        std::fs::read_to_string(root().join("targets/riscv-virt-up.eadl")).expect("the target");
    let text = format!(
        "{target}\n{}",
        one_task_system("", "\n  (platform (uses target.riscv-virt-up))")
    );
    let (refused, unknowns, obligations) = map_of(&text, &v);
    assert!(
        refused.is_empty() && unknowns.is_empty(),
        "{refused:?} {unknowns:?}"
    );
    assert_eq!(
        obligations,
        [Obligation::Placement {
            system: "s".to_string(),
            provider: "target.riscv-virt-up".to_string(),
            region: "target.ram".to_string(),
            holes: vec![],
        }]
    );
}

#[test]
fn ownership_no_longer_reads_placement_memory() {
    // Review R6: a bound device inside bound memory is the map's contradiction, not a second ownership report.
    let v = vocabulary();
    let text = "(defblock dev (offers (observable-output true) (region dev.regs (base 0x8100_0000) (size 4 KiB))))
(defblock mem (offers (core-count 1 tick) (region ram (base 0x8000_0000) (size 128 MiB) (executable true))))
(defservice out (requires (needs observable-output core-count)))";
    for r in every(text, &v) {
        assert!(r.conflicts.is_empty(), "{:?}", report_of(&r));
    }
}

#[test]
fn a_required_ordering_is_unsupported_under_this_profile() {
    // Leaf `M3.3.5.2`: `ordering` has no entry in `/1` — `rt-static-up-v1` excludes `dma`, whose obligations name
    // ordering — so a requirement on it in a system's closure is `unsupported-profile`, the relation's reading of a
    // constraint on an undeclared fact (record §1.1). `docs/semantics/boundary/accept/required-ordering-guarantee.eadl`
    // requests nothing, so no closure judges it and its frozen verdict stays.
    let v = vocabulary();
    let platform = forms(
        "(defplatform soc.bus (requires (ordering (before (write device.control)) (after (write memory.buffer)))))",
    );
    let refused = eadl_resolve::requirement::read_side(&platform[0], &v)
        .expect_err("not judged under this profile");
    assert_eq!(
        eadl_resolve::requirement::code(&refused).slug(),
        "unsupported-profile",
        "{refused:?}"
    );
}

// The initialization graph (leaf `M3.3.7`, model §8 rule 12): derived for `M4.1`'s plan, refusing nothing.

/// The graph of `text`'s one system for its one assignment, with `extra` requesters — blocks or platforms that require
/// something — read too, since a provider's own requirements are bound like anyone's.
fn graph(text: &str, extra: &[&str], v: &Vocabulary) -> Initialization {
    let read = Read::of(text, v);
    let [system] = read.systems().try_into().expect("one system");
    let mut all = read.requesters.clone();
    for name in extra {
        all.extend(requesters(read.decl(name), v).expect("reads"));
    }
    let pairs: Vec<(Provider, &Form)> = read
        .providers
        .iter()
        .map(|p| (p.clone(), read.decl(&p.name)))
        .collect();
    let joint = Joint::new(&pairs, &all, v).expect("reads");
    let with_extra = Read {
        decls: read.decls.clone(),
        providers: read.providers.clone(),
        requesters: all,
    };
    let [assignment] = with_extra
        .assignments(v)
        .try_into()
        .expect("one assignment");
    joint.initialization(&system, &assignment)
}

fn edge(from: &str, to: &str, because: Because) -> (String, String, Because) {
    (from.to_string(), to.to_string(), because)
}

fn bound(fact: &str) -> Because {
    Because::Binding(fact.to_string())
}

#[test]
fn a_system_s_initialization_graph_runs_from_providers_and_used_services_to_their_users() {
    let v = vocabulary();
    let text = "(defblock timer.counter (offers (counter-width 32 bit)))
(defplatform soc (offers (core-count 1 tick)) (requires (uses timer.counter)))
(defservice time.monotonic (requires (needs counter-width)))
(defservice time.periodic-release (requires (needs time.monotonic)))
(defpolicy sched (requires (needs core-count)))
(defsystem s
  (task t (period 10 ms) (deadline 10 ms) (deadline-from release) (priority 1) (uses time.periodic-release))
  (requires (uses time.monotonic time.periodic-release sched))
  (platform (uses soc)))";
    let g = graph(text, &[], &v);
    assert_eq!(
        g.nodes,
        [
            ("sched".to_string(), Node::Other),
            ("soc".to_string(), Node::Provider),
            ("time.monotonic".to_string(), Node::Service),
            ("time.periodic-release".to_string(), Node::Service),
            ("timer.counter".to_string(), Node::Provider),
        ],
        "the system's own level and its task are no nodes"
    );
    assert_eq!(
        g.edges,
        [
            edge("soc", "sched", bound("core-count")),
            edge("time.monotonic", "time.periodic-release", Because::Named),
            edge("timer.counter", "time.monotonic", bound("counter-width")),
        ]
    );
    assert!(g.components.is_empty());
}

#[test]
fn a_composite_and_its_part_binding_each_other_make_no_edge() {
    // Review D1: a well-formed board whose platform needs its timer's counter and whose timer needs the bus.
    let v = vocabulary();
    let text = "(defblock timer (offers (counter-width 32 bit)) (requires (needs bus-width)))
(defplatform soc (offers (bus-width 32 bit) (core-count 1 tick))
  (requires (uses timer) (needs counter-width)))
(defsystem s
  (task t (period 10 ms) (deadline 10 ms) (deadline-from release) (priority 1) (needs counter-width))
  (platform (uses soc)))";
    let g = graph(text, &["timer", "soc"], &v);
    assert_eq!(
        g.nodes,
        [
            ("soc".to_string(), Node::Provider),
            ("timer".to_string(), Node::Provider)
        ]
    );
    assert!(g.edges.is_empty(), "{:?}", g.edges);
    assert!(g.components.is_empty());
}

#[test]
fn two_services_using_each_other_are_one_component_not_a_refusal() {
    // Review D3: a log that timestamps and a clock that logs its wrap each initialise without calling the other.
    let v = vocabulary();
    let text = "(defblock timer.counter (offers (counter-width 32 bit) (observable-output true)))
(defplatform soc (offers (core-count 1 tick)) (requires (uses timer.counter)))
(defservice log.write (requires (needs observable-output) (uses time.monotonic)))
(defservice time.monotonic (requires (needs counter-width) (uses log.write)))
(defsystem s
  (task t (period 10 ms) (deadline 10 ms) (deadline-from release) (priority 1) (uses log.write))
  (platform (uses soc)))";
    let g = graph(text, &[], &v);
    assert_eq!(
        g.components,
        [vec!["log.write".to_string(), "time.monotonic".to_string()]]
    );
    assert!(g
        .edges
        .contains(&edge("log.write", "time.monotonic", Because::Named)));
    assert!(g
        .edges
        .contains(&edge("time.monotonic", "log.write", Because::Named)));
}

#[test]
fn a_provider_s_own_bindings_count_as_an_adapter_s_will() {
    // Review D2: a multiplexer — `M3.2`'s adapter, held here by a block that no platform composes, as an adapter is composed
    // by none — bound for a served service, itself bound to the comparator beneath it: the system reaches it only through
    // that binding, and the comparator precedes it.
    let v = vocabulary();
    let text = "(defblock timer.compare (offers (absolute-deadline true) (channels 1 tick)))
(defblock alarm.mux (offers (relative-delay true) (channels 4 tick)) (requires (needs absolute-deadline)))
(defplatform soc (offers (core-count 1 tick)) (requires (uses timer.compare)))
(defservice time.delay (requires (needs relative-delay)))
(defservice orphan (requires (needs core-count)))
(defsystem s
  (task t (period 10 ms) (deadline 10 ms) (deadline-from release) (priority 1) (uses time.delay))
  (platform (uses soc)))";
    let g = graph(text, &["alarm.mux"], &v);
    assert_eq!(
        g.edges,
        [
            edge("alarm.mux", "time.delay", bound("relative-delay")),
            edge("timer.compare", "alarm.mux", bound("absolute-deadline")),
        ]
    );
    assert!(
        !g.nodes.iter().any(|(n, _)| n == "orphan"),
        "a requester the system does not serve is no node: {:?}",
        g.nodes
    );
}
