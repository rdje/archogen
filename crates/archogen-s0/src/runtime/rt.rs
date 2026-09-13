//! The S0 hosted-playground runtime.
//!
//! ⭐ **This file is emitted into the generated crate byte for byte**, and it is also compiled
//! and unit-tested here, inside the engine. `ROADMAP.md` §1.1 admits exactly this: "Reviewed
//! reusable Rust and assembly are legitimate catalog inputs. Generation may emit bindings,
//! specialized code, tables, layouts, and copied or linked components." What generation must not
//! do is invent behavior, so the *behavior* lives here where it can be reviewed and tested, and
//! only the **table** is specialized per description.
//!
//! A test asserts the emitted copy is identical to this source, so "reviewed" and "shipped"
//! cannot drift apart.
//!
//! # What it does
//!
//! One hyperperiod of modeled time, and one event per task release. It is not a scheduler: a
//! release is a workload fact that follows from the description, whereas when a task actually
//! *runs* is a scheduling result — and S0 establishes none.

/// One task of the generated system, as the plan fixed it.
pub struct Task {
    /// The task's declared name.
    pub name: &'static str,
    /// Its release period, in whole milliseconds of modeled time.
    pub period_ms: u64,
    /// Its priority rank. Ascending is highest-first.
    pub priority: i64,
}

/// Where an observation goes. The generated system holds one, supplied by the service the
/// description asked for.
pub trait Output {
    /// Emit one line of observation.
    fn line(&mut self, text: &str);
}

/// Run the system for one hyperperiod of modeled time, returning the number of releases.
///
/// # Panics
///
/// On a zero period, which a valid plan cannot contain. A generated program that cannot honor
/// its plan must stop rather than print a plausible trace: a silently dropped task would produce
/// an observation that looks exactly like a correct one for a smaller system.
pub fn run(system: &str, tasks: &[Task], horizon_ms: u64, out: &mut dyn Output) -> usize {
    out.line(&format!("system {system}"));

    let mut releases: Vec<(u64, i64, &'static str)> = Vec::new();
    for task in tasks {
        assert!(
            task.period_ms > 0,
            "generated plan is invalid: task `{}` has a zero period",
            task.name
        );
        let mut at = 0_u64;
        while at < horizon_ms {
            releases.push((at, task.priority, task.name));
            at += task.period_ms;
        }
    }

    // Ascending time, then ascending priority rank. §3.1 requires unique priorities, so this is
    // a total order and no further tie-break is reachable.
    releases.sort_by_key(|&(at, priority, _)| (at, priority));

    for &(at, _, name) in &releases {
        out.line(&format!("release {at} ms {name}"));
    }
    out.line(&format!(
        "summary hyperperiod {horizon_ms} ms releases {}",
        releases.len()
    ));
    releases.len()
}
