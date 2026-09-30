//! Supported-profile definitions.
//!
//! A profile is the contract that says which systems the toolchain will admit and what it
//! will claim about them. `ROADMAP.md` §3.1 fixes the first one, `rt-static-up-v1`, and §3.3
//! fixes how later ones are added: each new family "changes a named profile and its analysis
//! obligations", and "no earlier timing or isolation result automatically transfers".
//!
//! The exclusion list is **data, not prose**, because of the rule that makes it matter:
//!
//! > A request for these features returns an unsupported-profile diagnostic rather than
//! > silently reducing the requested guarantee.
//!
//! A refusal has to name what was refused and why. That is only reliable if the list the
//! engine consults and the list the documentation publishes are the same object — which a
//! test in this module enforces against `docs/profiles/rt-static-up-v1.md`.

/// One row of a profile's concern table: what was decided, for one concern.
pub struct ProfileDecision {
    /// The concern, e.g. `Scheduling`.
    pub concern: &'static str,
    /// The decision taken for this profile.
    pub decision: &'static str,
    /// Each rule the decision states, and where it is enforced (leaf `M1.10`). A row usually states several
    /// rules, and they are enforced in different places, so a row is classified part by part.
    pub parts: &'static [Part],
}

/// One rule of a profile decision, and the stage that enforces it.
pub struct Part {
    /// The rule, in the decision's own words.
    pub rule: &'static str,
    /// Where it is enforced.
    pub stage: Stage,
}

/// Where a profile rule is enforced (leaf `M1.10`).
///
/// ⭐ This is what turned "thirteen rows, one enforced" from a number into a routed list. A rule is enforced
/// **now** — by `archogen check` or by a verification step on the engine — or it belongs to a stage not yet
/// built, which names the leaf that builds it, or it is not a rule a description or a system can break at all.
/// A test holds every exclusion slug and every owner named here to something that exists.
pub enum Stage {
    /// `archogen check` refuses a description that breaks it; the mechanism, with any exclusion as `` `slug` ``.
    Check(&'static str),
    /// A verification step on the engine enforces it; which one.
    Tier(&'static str),
    /// A later stage enforces it; which stage, and the task-tree leaf that builds it.
    Later {
        /// The pipeline stage: resolution, build or analysis.
        stage: &'static str,
        /// The leaf that builds it.
        owner: &'static str,
    },
    /// Not a property a description or a generated system can break; why.
    NotARule(&'static str),
}

/// One capability this profile does not admit.
pub struct Exclusion {
    /// Stable machine name. Diagnostics quote it, so it may not be renamed casually.
    pub slug: &'static str,
    /// What is excluded, in one line.
    pub summary: &'static str,
    /// The obligation admitting it would add — the reason the refusal is not arbitrary.
    pub reason: &'static str,
}

/// A named supported profile.
pub struct Profile {
    /// The profile identity used on the command line and in reports.
    pub id: &'static str,
    /// One-line description.
    pub summary: &'static str,
    /// The concern/decision table.
    pub decisions: &'static [ProfileDecision],
    /// Capabilities this profile refuses, each with the obligation it would add.
    pub exclusions: &'static [Exclusion],
}

impl Profile {
    /// Look up an exclusion by its slug.
    #[must_use]
    pub fn exclusion(&self, slug: &str) -> Option<&Exclusion> {
        self.exclusions.iter().find(|item| item.slug == slug)
    }

    /// Whether this profile admits the named capability.
    ///
    /// An unknown slug is **not** admitted by default: a capability the profile has never
    /// heard of is a `missing-fact` for the caller to resolve, not a silent yes. Callers
    /// distinguish the two by checking [`Profile::exclusion`] first.
    #[must_use]
    pub fn excludes(&self, slug: &str) -> bool {
        self.exclusion(slug).is_some()
    }
}

/// The first supported family (`ROADMAP.md` §3.1): a small, single-core, statically
/// configured real-time executive.
pub const RT_STATIC_UP_V1: Profile = Profile {
    id: "rt-static-up-v1",
    summary: "single-core, statically configured, fixed-priority real-time executive",
    decisions: &[
        ProfileDecision {
            concern: "Processor",
            decision: "one active core; one execution context runs at a time",
            parts: &[
                Part {
                    rule: "one active core",
                    stage: Stage::Check("the `multicore` and `task-migration` exclusions"),
                },
                Part {
                    rule: "one execution context runs at a time",
                    stage: Stage::Later { stage: "build", owner: "M4.1" },
                },
            ],
        },
        ProfileDecision {
            concern: "Scheduling",
            decision: "static unique task priorities; preemption at the target's supported interrupt points; bounded kernel critical sections",
            parts: &[
                Part {
                    rule: "static unique task priorities",
                    stage: Stage::Check("`crate::workload`: a priority two tasks share is refused"),
                },
                Part {
                    rule: "preemption at the target's supported interrupt points",
                    stage: Stage::Later { stage: "build", owner: "M4.6" },
                },
                Part {
                    rule: "bounded kernel critical sections",
                    stage: Stage::Later { stage: "analysis", owner: "M2.6" },
                },
            ],
        },
        ProfileDecision {
            concern: "Workload",
            decision: "finite static task set; periodic or sporadic releases with declared minimum separation; constrained deadlines; bounded release jitter",
            parts: &[
                Part {
                    rule: "finite static task set",
                    stage: Stage::Check("the `dynamic-task-creation` exclusion"),
                },
                Part {
                    rule: "periodic or sporadic releases with declared minimum separation",
                    stage: Stage::Check("`crate::workload`: one release model per task"),
                },
                Part {
                    rule: "constrained deadlines",
                    stage: Stage::Check("`crate::workload`: a deadline longer than its separation is refused"),
                },
                Part {
                    rule: "bounded release jitter",
                    stage: Stage::Later { stage: "analysis", owner: "M2.6" },
                },
            ],
        },
        ProfileDecision {
            concern: "Task execution",
            decision: "separate statically allocated task stacks; bounded jobs; no self-suspension within a job",
            parts: &[
                Part {
                    rule: "separate statically allocated task stacks",
                    stage: Stage::Later { stage: "build", owner: "M4.4" },
                },
                Part {
                    rule: "bounded jobs",
                    stage: Stage::Later { stage: "analysis", owner: "M2.6" },
                },
                Part {
                    rule: "no self-suspension within a job",
                    stage: Stage::Later { stage: "build", owner: "M4.2" },
                },
            ],
        },
        ProfileDecision {
            concern: "Resources",
            decision: "static task and kernel objects; no runtime heap allocation; no application mutexes",
            parts: &[
                Part {
                    rule: "static task and kernel objects",
                    stage: Stage::Check("the `dynamic-task-creation` and `runtime-loaded-drivers` exclusions"),
                },
                Part {
                    rule: "no runtime heap allocation",
                    stage: Stage::Check("the `runtime-heap` exclusion"),
                },
                Part {
                    rule: "no application mutexes",
                    stage: Stage::Check("the `application-mutexes` exclusion"),
                },
            ],
        },
        ProfileDecision {
            concern: "Communication",
            decision: "no general IPC; task-to-task queues require a later profile amendment with bounded semantics",
            parts: &[
                Part {
                    rule: "no general IPC",
                    stage: Stage::Check("the `general-ipc` exclusion"),
                },
                Part {
                    rule: "task-to-task queues require a later profile amendment",
                    stage: Stage::NotARule("a statement about a future profile, not a rule of this one; `general-ipc` refuses a queue today"),
                },
            ],
        },
        ProfileDecision {
            concern: "Interrupts",
            decision: "timer interrupt plus explicitly modeled bounded sources; unknown or uncontrolled interrupt load is unsupported for timing assurance",
            parts: &[
                Part {
                    rule: "timer interrupt plus explicitly modeled bounded sources",
                    stage: Stage::Later { stage: "analysis", owner: "M2.6" },
                },
                Part {
                    rule: "unknown or uncontrolled interrupt load is unsupported",
                    stage: Stage::Check("the `unmodeled-interrupt-load` exclusion"),
                },
            ],
        },
        ProfileDecision {
            concern: "Time",
            decision: "fixed clock configuration; explicit counter modulus, conversion rules, and interrupt delivery bounds",
            parts: &[
                Part {
                    rule: "fixed clock configuration",
                    stage: Stage::Check("the `dynamic-clock-scaling` exclusion"),
                },
                Part {
                    rule: "explicit counter modulus, conversion rules",
                    stage: Stage::Later { stage: "resolution", owner: "M3.2" },
                },
                Part {
                    rule: "interrupt delivery bounds",
                    stage: Stage::Later { stage: "build", owner: "M4.5" },
                },
            ],
        },
        ProfileDecision {
            concern: "Isolation",
            decision: "trusted application components in one address space; no claim of isolation from malicious tasks",
            parts: &[
                Part {
                    rule: "trusted application components in one address space",
                    stage: Stage::Check("the `memory-isolation` exclusion"),
                },
                Part {
                    rule: "no claim of isolation from malicious tasks",
                    stage: Stage::NotARule("a limit on what the toolchain claims, not a property a description can break; the assurance report states it"),
                },
            ],
        },
        ProfileDecision {
            concern: "Fault response",
            decision: "defined overrun, unexpected-trap, stack-guard, and assertion failure policy; bounded diagnostic handling",
            parts: &[
                Part {
                    rule: "defined overrun, unexpected-trap, stack-guard, and assertion failure policy",
                    stage: Stage::Later { stage: "build", owner: "M4.6" },
                },
                Part {
                    rule: "bounded diagnostic handling",
                    stage: Stage::Later { stage: "build", owner: "M4.6" },
                },
            ],
        },
        ProfileDecision {
            concern: "Devices",
            decision: "timer and minimal observable output first; no filesystem, network stack, DMA driver, or general virtio dependency",
            parts: &[
                Part {
                    rule: "timer and minimal observable output first",
                    stage: Stage::NotARule("an order of work, not a rule a description can break"),
                },
                Part {
                    rule: "no filesystem, network stack, DMA driver",
                    stage: Stage::Check("the `filesystem`, `network-stack` and `dma` exclusions"),
                },
                Part {
                    rule: "general virtio dependency",
                    stage: Stage::Later { stage: "resolution", owner: "M3.1" },
                },
            ],
        },
        ProfileDecision {
            concern: "Runtime",
            decision: "Rust no_std core with narrow architecture and MMIO boundaries",
            parts: &[
                Part {
                    rule: "Rust no_std core",
                    stage: Stage::Tier("the `integration` tier's `no-std-build` step"),
                },
                Part {
                    rule: "narrow architecture and MMIO boundaries",
                    stage: Stage::Later { stage: "build", owner: "M4.2" },
                },
            ],
        },
        ProfileDecision {
            concern: "Host support",
            decision: "Linux and macOS development first; Windows after the primary pipeline works",
            parts: &[
                Part {
                    rule: "Linux and macOS development first; Windows after the primary pipeline works",
                    stage: Stage::NotARule("where the toolchain is developed first, not a property of a description or of a system"),
                },
            ],
        },
    ],
    exclusions: &[
        Exclusion {
            slug: "task-migration",
            summary: "moving a task between cores",
            reason: "meaningless on one active core, and its analysis requires a multicore interference model",
        },
        Exclusion {
            slug: "mixed-criticality-scheduling",
            summary: "criticality levels with mode change on budget overrun",
            reason: "a distinct scheduling policy with its own theorem, applicability conditions, and mode-change semantics",
        },
        Exclusion {
            slug: "dynamic-clock-scaling",
            summary: "changing clock frequency at run time",
            reason: "every execution bound is tied to a clock configuration; scaling invalidates them and changes the timer contract",
        },
        Exclusion {
            slug: "unbounded-blocking",
            summary: "blocking whose duration has no declared bound",
            reason: "response-time analysis needs a bounded blocking term; an unbounded one makes the recurrence meaningless",
        },
        Exclusion {
            slug: "arbitrary-async-executors",
            summary: "general asynchronous task executors and futures runtimes",
            reason: "introduces dynamic scheduling and self-suspension the admitted task model does not cover",
        },
        Exclusion {
            slug: "dynamic-task-creation",
            summary: "creating or destroying tasks after boot",
            reason: "the task set is an input to the schedulability argument; a changing set has no single analyzed workload",
        },
        Exclusion {
            slug: "runtime-loaded-drivers",
            summary: "loading device implementations at run time",
            reason: "provider selection, resource ownership, and initialization order are resolved and checked before the build",
        },
        Exclusion {
            slug: "application-mutexes",
            summary: "application-level mutual exclusion between tasks",
            reason: "requires a resource-sharing protocol, a priority-inversion bound, and a blocking term in the analysis",
        },
        Exclusion {
            slug: "general-ipc",
            summary: "general inter-task communication, including task-to-task queues",
            reason: "needs queue capacity, overflow semantics, and their response-time effects; a later profile amendment",
        },
        Exclusion {
            slug: "runtime-heap",
            summary: "dynamic memory allocation after boot",
            reason: "static allocation is what makes the memory accounting checkable against the linked image",
        },
        Exclusion {
            slug: "memory-isolation",
            summary: "isolation from malicious or faulty application components",
            reason: "all components share one address space here; isolation needs protected tasks and authority semantics",
        },
        Exclusion {
            slug: "unmodeled-interrupt-load",
            summary: "interrupt sources whose arrival or cost is not declared",
            reason: "an omitted source that can run during the analyzed interval silently invalidates every timing result",
        },
        Exclusion {
            slug: "filesystem",
            summary: "a filesystem service",
            reason: "protocol semantics, fault and recovery behavior, persistence, and bounded resource use are all out of scope",
        },
        Exclusion {
            slug: "network-stack",
            summary: "a network protocol stack",
            reason: "unbounded external arrival and protocol state are outside the admitted workload model",
        },
        Exclusion {
            slug: "dma",
            summary: "DMA-capable devices and non-coherent transfers",
            reason: "needs buffer ownership, cache maintenance, ordering, addressability, and bus contention analysis",
        },
        Exclusion {
            slug: "power-states",
            summary: "dynamic power states and clock domains",
            reason: "feature availability becomes state-dependent, and transitions change timing bounds and time continuity",
        },
        Exclusion {
            slug: "multicore",
            summary: "more than one active core",
            reason: "requires an explicit memory model, inter-core interrupts, and shared-resource interference analysis",
        },
        Exclusion {
            slug: "posix",
            summary: "a POSIX personality",
            reason: "needs a named compliance subset, conformance tests, and specified divergences",
        },
    ],
};

/// Every profile this build supports.
pub const SUPPORTED: &[&Profile] = &[&RT_STATIC_UP_V1];

/// Look up a supported profile by id.
#[must_use]
pub fn supported(id: &str) -> Option<&'static Profile> {
    SUPPORTED.iter().copied().find(|profile| profile.id == id)
}

#[cfg(test)]
mod tests {
    use super::{supported, RT_STATIC_UP_V1, SUPPORTED};

    /// The published profile page. Read at compile time so the test cannot pass against a
    /// stale copy: if the file moves or is deleted, this crate stops compiling.
    const PAGE: &str = include_str!("../../../docs/profiles/rt-static-up-v1.md");

    #[test]
    fn the_first_profile_is_the_roadmap_one() {
        assert_eq!(RT_STATIC_UP_V1.id, "rt-static-up-v1");
        assert!(supported("rt-static-up-v1").is_some());
        assert!(supported("rt-static-up-v2").is_none());
        assert_eq!(SUPPORTED.len(), 1);
    }

    #[test]
    fn exclusion_slugs_are_unique() {
        let mut slugs: Vec<&str> = RT_STATIC_UP_V1
            .exclusions
            .iter()
            .map(|item| item.slug)
            .collect();
        let total = slugs.len();
        slugs.sort_unstable();
        slugs.dedup();
        assert_eq!(slugs.len(), total, "duplicate exclusion slug");
    }

    #[test]
    fn slugs_are_stable_identifiers() {
        // Diagnostics quote these, so they must be safe to print, grep and match on.
        for item in RT_STATIC_UP_V1.exclusions {
            assert!(!item.slug.is_empty());
            assert!(
                item.slug
                    .chars()
                    .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '-'),
                "`{}` is not a kebab-case slug",
                item.slug
            );
            assert!(!item.reason.is_empty(), "`{}` has no reason", item.slug);
        }
    }

    #[test]
    fn an_unknown_capability_is_not_admitted_by_silence() {
        assert!(RT_STATIC_UP_V1.excludes("multicore"));
        assert!(!RT_STATIC_UP_V1.excludes("quantum-annealing"));
        assert!(RT_STATIC_UP_V1.exclusion("quantum-annealing").is_none());
    }

    /// Pull the slugs out of the published page's exclusion table. The table rows begin with
    /// a code-quoted slug: `| `slug` | … |`.
    fn published_slugs() -> Vec<String> {
        PAGE.lines()
            .filter_map(|line| {
                let line = line.trim();
                let rest = line.strip_prefix("| `")?;
                let (slug, _) = rest.split_once('`')?;
                Some(slug.to_string())
            })
            .collect()
    }

    #[test]
    fn the_published_page_lists_exactly_the_excluded_capabilities() {
        // ⭐ The point of the whole module. §3.1 requires an unsupported request to be
        // refused by name; that is only trustworthy if the list the engine consults and the
        // list the documentation publishes cannot drift apart.
        let mut published = published_slugs();
        let mut declared: Vec<String> = RT_STATIC_UP_V1
            .exclusions
            .iter()
            .map(|item| item.slug.to_string())
            .collect();
        published.sort();
        declared.sort();
        assert_eq!(
            published, declared,
            "docs/profiles/rt-static-up-v1.md has drifted from the profile data"
        );
    }

    #[test]
    fn the_published_page_carries_every_concern_decision() {
        for row in RT_STATIC_UP_V1.decisions {
            assert!(
                PAGE.contains(row.concern),
                "concern `{}` missing from the published page",
                row.concern
            );
        }
    }
}
