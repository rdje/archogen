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
    requesters, Binding, Finding, Interrupt, Joint, Kind, Obligation, Report, Requester, System,
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
