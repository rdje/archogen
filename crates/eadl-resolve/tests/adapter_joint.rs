//! An adapter's instance and the joint rules over one (leaf `M3.2.3.1`, `docs/specs/adapters/decision_engine-adapters.md`
//! §1.5, §3.1, §4): its offers computed from the provider it programs, and the rules judging an assignment that binds
//! one — written here by hand, as the search will propose it in `M3.2.3.2`. Expectations are literal: providers,
//! values, codes.

use eadl_front::{read, Form, SourceMap};
use eadl_model::check::shipped_registry;
use eadl_model::profile::RT_STATIC_UP_V1;
use eadl_resolve::adapter::{instance_name, Catalog, Instance};
use eadl_resolve::joint::{
    requesters, Binding, Finding, Interrupt, Joint, Obligation, Requester, System,
};
use eadl_resolve::offer::{self, Provider, State};
use eadl_resolve::relation::{judge, Verdict};
use eadl_resolve::requirement::Requirement;
use eadl_resolve::value::Value;
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

fn catalog(v: &Vocabulary) -> Catalog {
    Catalog::shipped(&mut SourceMap::new(), v).expect("the shipped catalog reads")
}

/// A description read: its declarations, providers and requesters.
struct Read {
    decls: Vec<Form>,
    providers: Vec<Provider>,
    requesters: Vec<Requester>,
}

impl Read {
    fn of(text: &str, v: &Vocabulary) -> Self {
        let mut sources = SourceMap::new();
        let id = sources.add("t.eadl", text).expect("small");
        let (doc, diags) = read(&sources, id);
        assert!(!diags.has_errors(), "{text}");
        let decls: Vec<Form> = doc
            .forms
            .into_iter()
            .filter(|f| f.head() != Some("eadl-version"))
            .collect();
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

    fn provider(&self, name: &str) -> &Provider {
        self.providers
            .iter()
            .find(|p| p.name == name)
            .expect("a provider")
    }

    fn requester(&self, name: &str) -> &Requester {
        self.requesters
            .iter()
            .find(|r| r.name == name)
            .expect("a requester")
    }

    /// The joint rules over this description and `instances`, for its system `s` at the profile's level.
    fn joint<'v>(&self, v: &'v Vocabulary, instances: &[Instance]) -> Joint<'v> {
        let decl = |name: &str| {
            self.decls
                .iter()
                .find(|f| f.items().get(1).and_then(Form::as_symbol) == Some(name))
                .expect("declared")
        };
        let pairs: Vec<(Provider, &Form)> = self
            .providers
            .iter()
            .map(|p| (p.clone(), decl(&p.name)))
            .collect();
        let system = System::read(decl("s"), &self.decls)
            .at(RT_STATIC_UP_V1.caller_level, v)
            .expect("the profile's level");
        Joint::new(&pairs, &self.requesters, v)
            .expect("the providers read")
            .with_instances(instances)
            .for_system(system)
    }
}

/// Every binding of `requester`'s requirements to `provider`.
fn bind(requester: &Requester, provider: &str) -> Vec<Binding> {
    requester
        .requirements
        .iter()
        .filter(|q| !matches!(q, Requirement::Statement { .. }))
        .map(|q| Binding {
            requester: requester.name.clone(),
            requirement: q.clone(),
            provider: provider.to_string(),
        })
        .collect()
}

/// An instance's `beneath` bindings to its provider, as its kind requires them.
fn beneath(catalog: &Catalog, instance: &Instance) -> Vec<Binding> {
    let kind = catalog.kind(&instance.kind).expect("its kind");
    kind.beneath
        .iter()
        .map(|q| Binding {
            requester: instance.provider.name.clone(),
            requirement: q.clone(),
            provider: instance.over.clone(),
        })
        .collect()
}

/// A timer `relative-delay` programs, of `channels` channels, and the `deadline` services the system's tasks use.
fn delays(channels: u32, services: &str, tasks: &str, uses: &str) -> String {
    format!(
        "(eadl-version eadl/1)
(defblock timer.d (offers (relative-delay true) (counter-modulus 4294967296) (tick-rate 1 MHz) (observation-coherent true) (channels {channels} tick) (reachable-at-privilege machine)))
(defplatform soc (offers (core-count 1 tick) (reachable-at-privilege machine)) (requires (uses timer.d)))
{services}
(defsystem s {tasks} (requires (uses {uses})) (platform (uses soc)))
"
    )
}

const DEADLINE: &str = "(defservice time.deadline (requires (absolute-deadline (supported-horizon (at-least 10 s)) (delivery-bound (at-most 50 us))) (delivery-bound (at-most 40 us))))";

#[test]
fn an_instance_offers_what_its_kind_computes_from_its_provider() {
    let v = vocabulary();
    let c = catalog(&v);
    let d = Read::of(
        &delays(
            1,
            DEADLINE,
            "(task t (period 10 ms) (deadline 10 ms) (deadline-from release) (priority 1) (uses time.deadline))",
            "time.deadline",
        ),
        &v,
    );
    let deadline = d.requester("time.deadline");
    let kind = c.kind("deadline-over-delay").expect("the kind");
    let instance = kind.instance(
        d.provider("timer.d"),
        Some((&deadline.name, &deadline.requirements)),
        &v,
    );
    assert_eq!(
        instance.provider.name,
        "(adapter deadline-over-delay timer.d time.deadline)"
    );
    assert_eq!(instance.over, "timer.d");
    // The horizon passed through from the counter's, derived there: (2^32 − 1) / 1 MHz, 4294.967295 s, held exact.
    let horizon = format!("{:?}", instance.provider.state("supported-horizon"));
    assert!(
        horizon.contains("numerator: 858993459, denominator: 200000")
            && horizon.contains("name: \"s\""),
        "{horizon}"
    );
    // The obliged bound at the tightest the requester writes, a nested and a flat one: 40 us.
    let delivery = format!("{:?}", instance.provider.state("delivery-bound"));
    assert!(delivery.contains("40"), "{delivery}");
    assert_eq!(
        instance.provider.state("absolute-deadline"),
        Some(&State::Valued(vec![Value::Bool(true)]))
    );
    assert_eq!(
        instance.provider.state("channels"),
        Some(&State::Valued(vec![Value::Count(1)]))
    );
    // Every requirement the service writes is met at the instance, by the relation as at any provider.
    for q in &deadline.requirements {
        assert_eq!(
            judge(&instance.provider, q, &v),
            Verdict::Satisfied,
            "{}",
            q.fact()
        );
    }
    // The extender over a 16-bit modular counter: 2^64 counts at its rate, the horizon derived at the instance.
    let e = Read::of(
        "(eadl-version eadl/1)
(defblock timer.short (offers (relative-delay true) (counter-modulus 65536) (tick-rate 10 MHz) (wrap-behavior modular) (observation-coherent true) (channels 1 tick) (reachable-at-privilege machine)))
(defservice time.monotonic (requires (unambiguous-horizon (at-least 60 s)) (needs counter-width tick-rate observation-coherent)))
",
        &v,
    );
    let extender =
        c.kind("epoch-extender")
            .expect("the kind")
            .instance(e.provider("timer.short"), None, &v);
    assert_eq!(
        extender.provider.name,
        "(adapter epoch-extender timer.short)"
    );
    for q in &e.requester("time.monotonic").requirements {
        assert_eq!(
            judge(&extender.provider, q, &v),
            Verdict::Satisfied,
            "{}",
            q.fact()
        );
        // The counter alone meets the presence of its own facts, and not the horizon.
        if q.fact() == "unambiguous-horizon" {
            assert_eq!(judge(e.provider("timer.short"), q, &v), Verdict::Refused);
        }
    }
}

#[test]
fn an_assignment_through_an_instance_holds_its_routes_level_and_order() {
    let v = vocabulary();
    let c = catalog(&v);
    let d = Read::of(
        &delays(
            1,
            DEADLINE,
            "(task t (period 10 ms) (deadline 10 ms) (deadline-from release) (priority 1) (uses time.deadline))",
            "time.deadline",
        ),
        &v,
    );
    let deadline = d.requester("time.deadline");
    let instance = c.kind("deadline-over-delay").expect("the kind").instance(
        d.provider("timer.d"),
        Some((&deadline.name, &deadline.requirements)),
        &v,
    );
    let joint = d.joint(&v, std::slice::from_ref(&instance));
    let mut assignment = bind(deadline, &instance.provider.name);
    assignment.extend(beneath(&c, &instance));
    let report = joint.check(&assignment);
    assert!(report.holds(), "{report:?}");
    // One timer route, at the device, carrying the service the instance serves; none at the instance.
    let routes: Vec<(String, Vec<String>)> = report
        .obligations
        .iter()
        .filter_map(|o| match o {
            Obligation::Route {
                provider,
                interrupt: Interrupt::Timer,
                requesters,
                ..
            } => Some((provider.clone(), requesters.clone())),
            _ => None,
        })
        .collect();
    assert_eq!(
        routes,
        [("timer.d".to_string(), vec!["time.deadline".to_string()])]
    );
    // The device is initialised before the instance, and the instance before the service it serves.
    let system = System::read(
        d.decls
            .iter()
            .find(|f| f.head() == Some("defsystem"))
            .expect("the system"),
        &d.decls,
    );
    let order = joint.initialization(&system, &assignment);
    let edges: Vec<(String, String)> = order
        .edges
        .iter()
        .map(|(from, to, _)| (from.clone(), to.clone()))
        .collect();
    assert!(
        edges.contains(&("timer.d".to_string(), instance.provider.name.clone())),
        "{edges:?}"
    );
    assert!(
        edges.contains(&(instance.provider.name.clone(), "time.deadline".to_string())),
        "{edges:?}"
    );
}

#[test]
fn two_requesters_each_take_their_own_instance_s_channel_beneath() {
    let v = vocabulary();
    let c = catalog(&v);
    let services = "(defservice alarm.a (requires (needs absolute-deadline)))
(defservice alarm.b (requires (needs absolute-deadline)))";
    let tasks = "(task a (period 10 ms) (deadline 10 ms) (deadline-from release) (priority 1) (uses alarm.a))
  (task b (period 20 ms) (deadline 20 ms) (deadline-from release) (priority 2) (uses alarm.b))";
    for (channels, holds) in [(1, false), (2, true)] {
        let d = Read::of(&delays(channels, services, tasks, "alarm.a alarm.b"), &v);
        let kind = c.kind("deadline-over-delay").expect("the kind");
        let instances: Vec<Instance> = ["alarm.a", "alarm.b"]
            .iter()
            .map(|name| {
                let r = d.requester(name);
                kind.instance(d.provider("timer.d"), Some((&r.name, &r.requirements)), &v)
            })
            .collect();
        let joint = d.joint(&v, &instances);
        let mut assignment = Vec::new();
        for (name, instance) in ["alarm.a", "alarm.b"].iter().zip(&instances) {
            assignment.extend(bind(d.requester(name), &instance.provider.name));
            assignment.extend(beneath(&c, instance));
        }
        let report = joint.check(&assignment);
        assert_eq!(report.holds(), holds, "{channels} channel(s): {report:?}");
        if !holds {
            assert!(
                report.conflicts.iter().any(|f| matches!(
                    f,
                    Finding::OverChannels { provider, .. } if provider == "timer.d"
                )),
                "{report:?}"
            );
        }
    }
}

#[test]
fn an_instance_takes_its_requesters_system_and_its_device_must_be_of_it() {
    let v = vocabulary();
    let c = catalog(&v);
    // `timer.d` is declared, and no platform of the system composes it.
    let text = "(eadl-version eadl/1)
(defblock timer.d (offers (relative-delay true) (counter-modulus 4294967296) (tick-rate 1 MHz) (observation-coherent true) (channels 1 tick) (reachable-at-privilege machine)))
(defblock console.uart (offers (observable-output true) (reachable-at-privilege machine)))
(defplatform soc (offers (core-count 1 tick) (reachable-at-privilege machine)) (requires (uses console.uart)))
(defservice alarm (requires (needs absolute-deadline)))
(defsystem s (task t (period 10 ms) (deadline 10 ms) (deadline-from release) (priority 1) (uses alarm)) (requires (uses alarm)) (platform (uses soc)))
";
    let d = Read::of(text, &v);
    let alarm = d.requester("alarm");
    let instance = c.kind("deadline-over-delay").expect("the kind").instance(
        d.provider("timer.d"),
        Some((&alarm.name, &alarm.requirements)),
        &v,
    );
    let joint = d.joint(&v, std::slice::from_ref(&instance));
    let mut assignment = bind(alarm, &instance.provider.name);
    assignment.extend(beneath(&c, &instance));
    let report = joint.check(&assignment);
    let unconnected: std::collections::BTreeSet<(String, String)> = report
        .conflicts
        .iter()
        .filter_map(|f| match f {
            Finding::Unconnected {
                requester,
                provider,
                ..
            } => Some((requester.clone(), provider.clone())),
            _ => None,
        })
        .collect();
    // The instance itself is no hardware and is not refused for it; the device it programs is, at each of its
    // bindings beneath, as connectivity reports any requester's.
    assert_eq!(
        unconnected.into_iter().collect::<Vec<_>>(),
        [(instance.provider.name.clone(), "timer.d".to_string())],
        "{report:?}"
    );
}

#[test]
fn a_deadline_and_the_time_it_is_read_in_are_one_device() {
    // Rule 14: `time.deadline` needs `time.monotonic`; the deadline programmed at one timer and the time read at
    // another is a conflict, at one timer none.
    let v = vocabulary();
    let c = catalog(&v);
    let text = |monotonic_at: &str| {
        format!(
            "(eadl-version eadl/1)
(defblock timer.a (offers (relative-delay true) (counter-modulus 4294967296) (tick-rate 1 MHz) (observation-coherent true) (channels 1 tick) (reachable-at-privilege machine)))
(defblock timer.b (offers (counter-modulus 4294967296) (tick-rate 1 MHz) (observation-coherent true) (reachable-at-privilege machine)))
(defplatform soc (offers (core-count 1 tick) (reachable-at-privilege machine)) (requires (uses timer.a timer.b)))
(defservice time.monotonic (requires (unambiguous-horizon (at-least 60 s)) (needs tick-rate observation-coherent)))
(defservice time.deadline (requires (needs time.monotonic absolute-deadline)))
(defsystem s (task t (period 10 ms) (deadline 10 ms) (deadline-from release) (priority 1) (uses time.deadline)) (requires (uses time.monotonic time.deadline)) (platform (uses soc)))
; {monotonic_at}
"
        )
    };
    for (monotonic_at, conflict) in [("timer.b", true), ("timer.a", false)] {
        let d = Read::of(&text(monotonic_at), &v);
        let deadline = d.requester("time.deadline");
        let instance = c.kind("deadline-over-delay").expect("the kind").instance(
            d.provider("timer.a"),
            Some((&deadline.name, &deadline.requirements)),
            &v,
        );
        let joint = d.joint(&v, std::slice::from_ref(&instance));
        let mut assignment = bind(deadline, &instance.provider.name);
        assignment.extend(beneath(&c, &instance));
        assignment.extend(bind(d.requester("time.monotonic"), monotonic_at));
        let report = joint.check(&assignment);
        let found: Vec<(String, String, String)> = report
            .conflicts
            .iter()
            .filter_map(|f| match f {
                Finding::TwoTimeBases {
                    requester,
                    device,
                    read_device,
                    ..
                } => Some((requester.clone(), device.clone(), read_device.clone())),
                _ => None,
            })
            .collect();
        if conflict {
            assert_eq!(
                found,
                [(
                    "time.deadline".to_string(),
                    "timer.a".to_string(),
                    "timer.b".to_string()
                )],
                "{report:?}"
            );
            assert_eq!(report.conflicts[0].code(), "infeasible-configuration");
        } else {
            assert!(found.is_empty(), "{report:?}");
        }
    }
    assert_eq!(
        instance_name("epoch-extender", "timer.a", None),
        "(adapter epoch-extender timer.a)"
    );
}
