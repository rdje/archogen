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
    requesters, Binding, Finding, Joint, Kind, Obligation, Report, Requester,
};
use eadl_resolve::offer::{self, Provider};
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

    fn decl(&self, name: &str) -> &Form {
        self.decls
            .iter()
            .find(|f| f.items().get(1).and_then(Form::as_symbol) == Some(name))
            .expect("declared")
    }

    /// Every complete assignment: each requirement on a guarantee bound to each provider the relation satisfies it at.
    fn assignments(&self, v: &Vocabulary) -> Vec<Vec<Binding>> {
        let mut all: Vec<Vec<Binding>> = vec![Vec::new()];
        for r in &self.requesters {
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
    }
}

fn report_of(r: &Report) -> (Vec<String>, Vec<String>) {
    (
        r.conflicts.iter().map(named).collect(),
        r.unknowns.iter().map(named).collect(),
    )
}

/// The reports of every assignment of `text`.
fn every(text: &str, v: &Vocabulary) -> Vec<Report> {
    let read = Read::of(text, v);
    let joint = read
        .joint(v)
        .unwrap_or_else(|f| panic!("{:?}", f.iter().map(named).collect::<Vec<_>>()));
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
        assert!(r.obligations.is_empty());
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
    let text = fixture("f08-two-exclusive-requests.eadl")
        + "\n(defblock alarm.mux (offers (absolute-deadline true) (channels 4 tick)))\n";
    let read = Read::of(&text, &v);
    let joint = read.joint(&v).expect("reads");
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
    let text = "(defblock timer.compare (offers (absolute-deadline true) (channels 1 tick)))
(defsystem s
  (task a (period 10 ms) (deadline 10 ms) (deadline-from release) (priority 1) (needs absolute-deadline))
  (task b (period 20 ms) (deadline 20 ms) (deadline-from release) (priority 2) (needs absolute-deadline)))";
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
    let text = "(defblock timer.compare (offers (absolute-deadline true) (channels 1 tick)))
(defservice time.deadline (requires (needs absolute-deadline)))
(defsystem s
  (task a (period 10 ms) (deadline 10 ms) (deadline-from release) (priority 1) (uses time.deadline))
  (task b (period 20 ms) (deadline 20 ms) (deadline-from release) (priority 2) (uses time.deadline)))";
    let reports = every(text, &v);
    assert_eq!(reports.len(), 1);
    assert!(reports[0].holds());
    assert_eq!(
        reports[0].obligations,
        [Obligation::Multiplex {
            service: "time.deadline".to_string(),
            users: vec!["s/a".to_string(), "s/b".to_string()],
            provider: "timer.compare".to_string(),
        }]
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
