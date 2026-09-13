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
        },
        ProfileDecision {
            concern: "Scheduling",
            decision: "static unique task priorities; preemption at the target's supported interrupt points; bounded kernel critical sections",
        },
        ProfileDecision {
            concern: "Workload",
            decision: "finite static task set; periodic or sporadic releases with declared minimum separation; constrained deadlines; bounded release jitter",
        },
        ProfileDecision {
            concern: "Task execution",
            decision: "separate statically allocated task stacks; bounded jobs; no self-suspension within a job",
        },
        ProfileDecision {
            concern: "Resources",
            decision: "static task and kernel objects; no runtime heap allocation; no application mutexes",
        },
        ProfileDecision {
            concern: "Communication",
            decision: "no general IPC; task-to-task queues require a later profile amendment with bounded semantics",
        },
        ProfileDecision {
            concern: "Interrupts",
            decision: "timer interrupt plus explicitly modeled bounded sources; unknown or uncontrolled interrupt load is unsupported for timing assurance",
        },
        ProfileDecision {
            concern: "Time",
            decision: "fixed clock configuration; explicit counter modulus, conversion rules, and interrupt delivery bounds",
        },
        ProfileDecision {
            concern: "Isolation",
            decision: "trusted application components in one address space; no claim of isolation from malicious tasks",
        },
        ProfileDecision {
            concern: "Fault response",
            decision: "defined overrun, unexpected-trap, stack-guard, and assertion failure policy; bounded diagnostic handling",
        },
        ProfileDecision {
            concern: "Devices",
            decision: "timer and minimal observable output first; no filesystem, network stack, DMA driver, or general virtio dependency",
        },
        ProfileDecision {
            concern: "Runtime",
            decision: "Rust no_std core with narrow architecture and MMIO boundaries",
        },
        ProfileDecision {
            concern: "Host support",
            decision: "Linux and macOS development first; Windows after the primary pipeline works",
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
