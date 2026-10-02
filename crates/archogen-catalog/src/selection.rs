//! §12's selection: which records an analysis draws on, and from whom each name comes (`M2.7.3.5.2`).
//!
//! An analysis for profile `P` on target `X` reads facts from records whose `profiles` include `P` and whose
//! `targets` admit `X`, and costs whose `target` is `X` in records whose `profiles` include `P`. A claim with no
//! target reads facts only from records with `(targets any)`, and no cost. Each name comes from exactly one record
//! under a selection, or the catalog is refused (`catalog-conflict`). A code fact grouped with a cost comes from the
//! record that supplies that cost, its group's anchor, so one review sees the fact and the code together.

use std::collections::{BTreeMap, BTreeSet};

use crate::hash::Catalog;
use crate::record::{Content, Cost, FacetKind, Fact, Record, Targets};
use crate::refusal::{Code, Refusal};
use crate::statement;

/// An analysis's selection: a profile, and a target or none.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Selection<'a> {
    /// The profile's id.
    pub profile: &'a str,
    /// The target's stem, or `None` for a claim with no target.
    pub target: Option<&'a str>,
}

/// What a lookup found: the record that supplies the name, and the fact or cost.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Supplied<'r> {
    /// A behavioral or timing fact.
    Fact(&'r Fact),
    /// A timing cost.
    Cost(&'r Cost),
}

/// A lookup's answer.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Found<'r> {
    /// The id of the one record that supplies the name.
    pub id: &'r str,
    /// What it supplies.
    pub value: Supplied<'r>,
}

/// The groups of §12, each anchored on a cost: its anchor, and whether a name belongs to it.
fn group_of(name: &str) -> Option<String> {
    const SWITCH: [&str; 8] = [
        "eager-switching",
        "interrupts-do-not-nest",
        "services-preempt-every-task",
        "pending-taken-and-transitions-unmasked",
        "pending-taken-after-unmask",
        "no-empty-claim",
        "one-claim-per-trap",
        "starts-by-transition",
    ];
    const TIMER: [&str; 8] = [
        "timer-event-driven",
        "compare-rounds-up",
        "due-check-matches-compare",
        "no-early-release",
        "raised-only-when-due",
        "only-timer-releases-timer-tasks",
        "reprograms-only-in-service",
        "compare-rounding",
    ];
    const COMPLETION: [&str; 6] = [
        "no-suspension-primitive",
        "no-scheduler-lock-primitive",
        "preemptive-everywhere",
        "sections-mask-every-interrupt",
        "releases-never-latched",
        "primitives-out-of-line",
    ];
    const SERVICE: [&str; 5] = [
        "no-application-code.",
        "acknowledge-at-entry.",
        "defers-nothing.",
        "one-request-per-arrival.",
        "external.",
    ];
    if SWITCH.contains(&name)
        || statement::FACTS.contains(&name)
        || statement::COSTS.contains(&name)
    {
        return Some("switch".to_owned());
    }
    if TIMER.contains(&name) {
        return Some("timer-service".to_owned());
    }
    // `api.<p>` and `masked.<p>`, `masked.completion` among them.
    if COMPLETION.contains(&name)
        || ["api.", "masked."]
            .iter()
            .any(|p| name.strip_prefix(p).is_some_and(|rest| !rest.is_empty()))
    {
        return Some("completion".to_owned());
    }
    SERVICE
        .iter()
        .find_map(|p| name.strip_prefix(p).filter(|source| !source.is_empty()))
        .map(|source| format!("service.{source}"))
}

/// Whether `record` is drawn on for facts under `selection`.
fn facts_drawn(record: &Record, selection: Selection<'_>) -> bool {
    record
        .contract
        .profiles
        .iter()
        .any(|p| p == selection.profile)
        && match (&record.contract.targets, selection.target) {
            (Targets::Any, _) => true,
            (Targets::Named(named), Some(target)) => named.iter().any(|t| t == target),
            (Targets::Named(_), None) => false,
        }
}

/// Every fact and cost `record` supplies under `selection`, by `(facet, name)`.
fn supplied<'r>(
    record: &'r Record,
    selection: Selection<'_>,
) -> Vec<(FacetKind, &'r str, Supplied<'r>)> {
    let mut out = Vec::new();
    if facts_drawn(record, selection) {
        if let Content::Present(model) = &record.behavior_model.content {
            for fact in &model.facts {
                out.push((
                    FacetKind::BehaviorModel,
                    fact.name.as_str(),
                    Supplied::Fact(fact),
                ));
            }
        }
        if let Content::Present(model) = &record.timing_model.content {
            for fact in &model.facts {
                out.push((
                    FacetKind::TimingModel,
                    fact.name.as_str(),
                    Supplied::Fact(fact),
                ));
            }
        }
    }
    if let (Some(target), true) = (
        selection.target,
        record
            .contract
            .profiles
            .iter()
            .any(|p| p == selection.profile),
    ) {
        if let Content::Present(model) = &record.timing_model.content {
            for cost in model.costs.iter().filter(|c| c.target == target) {
                out.push((
                    FacetKind::TimingModel,
                    cost.name.as_str(),
                    Supplied::Cost(cost),
                ));
            }
        }
    }
    out
}

impl Catalog {
    /// Every supplier of every name under `selection`.
    fn suppliers(&self, selection: Selection<'_>) -> BTreeMap<(FacetKind, &str), Vec<Found<'_>>> {
        let mut out: BTreeMap<(FacetKind, &str), Vec<Found<'_>>> = BTreeMap::new();
        for (id, record) in &self.records {
            for (facet, name, value) in supplied(record, selection) {
                out.entry((facet, name))
                    .or_default()
                    .push(Found { id, value });
            }
        }
        out
    }

    /// The one supplier of `(facet, name)` under `selection`, ignoring groups.
    fn one<'r>(
        &'r self,
        selection: Selection<'_>,
        facet: FacetKind,
        name: &str,
    ) -> Result<Option<Found<'r>>, Refusal> {
        let found: Vec<Found<'r>> = self
            .records
            .iter()
            .flat_map(|(id, record)| {
                supplied(record, selection)
                    .into_iter()
                    .filter(|(f, n, _)| *f == facet && *n == name)
                    .map(move |(_, _, value)| Found { id, value })
            })
            .collect();
        match found.as_slice() {
            [] => Ok(None),
            [one] => Ok(Some(*one)),
            [a, b, ..] => Err(conflict(selection, facet, name, a.id, b.id)),
        }
    }

    /// A lookup by `(facet, name)` under `selection` (§12). A name in a group is read only from the record its
    /// anchor's lookup returned, so a fact is never combined with a cost from another record; where no record
    /// supplies the anchor, or the claim has no target, the group's names are not read.
    ///
    /// # Errors
    ///
    /// `catalog-conflict` when two records supply the name, or its anchor, under the selection.
    pub fn lookup<'r>(
        &'r self,
        selection: Selection<'_>,
        facet: FacetKind,
        name: &str,
    ) -> Result<Option<Found<'r>>, Refusal> {
        let Some(anchor) = group_of(name) else {
            return self.one(selection, facet, name);
        };
        // Every anchor is a cost, and a claim with no target reads no cost, so its lookup finds none.
        let Some(by) = self.one(selection, FacetKind::TimingModel, &anchor)? else {
            return Ok(None);
        };
        Ok(self
            .one(selection, facet, name)?
            .filter(|found| found.id == by.id))
    }

    /// §12's checks over every selection the catalog names: each name supplied by one record, for every profile
    /// and target and for a claim with no target (`catalog-conflict`); and under every target selection where a
    /// group's anchor is supplied, each name of the group that some record supplies supplied by the anchor's record
    /// (`catalog-field`).
    ///
    /// # Errors
    ///
    /// `catalog-conflict` or `catalog-field` as above; `catalog-field` for a target file without its pair.
    pub fn check_selections(&self) -> Result<(), Refusal> {
        let profiles: BTreeSet<&str> = self
            .records
            .values()
            .flat_map(|r| r.contract.profiles.iter().map(String::as_str))
            .collect();
        let named = self.named_targets()?;
        let targets: Vec<Option<&str>> = named
            .iter()
            .map(|t| Some(t.as_str()))
            .chain([None])
            .collect();
        for profile in &profiles {
            for target in &targets {
                let selection = Selection {
                    profile,
                    target: *target,
                };
                let suppliers = self.suppliers(selection);
                for ((facet, name), found) in &suppliers {
                    if let [a, b, ..] = found.as_slice() {
                        return Err(conflict(selection, *facet, name, a.id, b.id));
                    }
                }
                // With no target no cost is supplied, so no anchor is, and no group is checked.
                for ((_, name), found) in &suppliers {
                    let Some(anchor) = group_of(name) else {
                        continue;
                    };
                    let Some([by]) = suppliers
                        .get(&(FacetKind::TimingModel, anchor.as_str()))
                        .map(Vec::as_slice)
                    else {
                        continue;
                    };
                    if let Some(other) = found.iter().find(|f| f.id != by.id) {
                        let path = self.records.get(other.id).map_or("", |r| r.path.as_str());
                        return Err(Refusal::new(
                            Code::Field,
                            path,
                            &format!("fact[{name}]"),
                            None,
                            format!(
                                "under {}, `{name}` is in the `{anchor}` group, so it comes from `{}`, which supplies \
                                 `{anchor}`, not from `{}`",
                                describe(selection),
                                by.id,
                                other.id
                            ),
                        ));
                    }
                }
                self.statement_under(selection, &suppliers)?;
            }
        }
        Ok(())
    }

    /// §14.4 under one selection: where a record supplies `switch`, it depends on exactly one check-passing
    /// convention, and no record in the selection on another (`catalog-conflict`); and it supplies every fact of
    /// the table that has no read condition or whose condition holds, `one-claim-per-trap`, and both costs
    /// (`catalog-field`). Under a selection with no `switch`, nothing is checked.
    fn statement_under(
        &self,
        selection: Selection<'_>,
        suppliers: &BTreeMap<(FacetKind, &str), Vec<Found<'_>>>,
    ) -> Result<(), Refusal> {
        let Some([by]) = suppliers
            .get(&(FacetKind::TimingModel, "switch"))
            .map(Vec::as_slice)
        else {
            return Ok(());
        };
        let Some(port) = self.records.get(by.id) else {
            return Ok(());
        };
        let conventions = |record: &Record| -> Vec<String> {
            record
                .contract
                .depends
                .iter()
                .map(|d| d.id.clone())
                .filter(|id| statement::is_convention(id))
                .collect()
        };
        let chosen = conventions(port);
        if chosen.len() != 1 {
            return Err(Refusal::new(
                Code::Conflict,
                &port.path,
                "depends",
                None,
                format!(
                    "under {}, `{}` supplies `switch` and depends on {} check-passing conventions; it depends on exactly one (§14.4)",
                    describe(selection),
                    port.id,
                    chosen.len()
                ),
            ));
        }
        for (id, record) in &self.records {
            if !facts_drawn(record, selection) {
                continue;
            }
            if let Some(other) = conventions(record).iter().find(|c| **c != chosen[0]) {
                return Err(Refusal::new(
                    Code::Conflict,
                    &record.path,
                    "depends",
                    None,
                    format!(
                        "under {}, `{id}` depends on `{other}`, and the port, `{}`, on `{}`: one convention per selection (§14.4)",
                        describe(selection),
                        port.id,
                        chosen[0]
                    ),
                ));
            }
        }
        let facts = match &port.behavior_model.content {
            Content::Present(m) => m.facts.as_slice(),
            Content::None(_) => &[],
        };
        // The group rule, checked first, leaves the port the only supplier of every name it owes.
        let missing = |facet: FacetKind, name: &str| !suppliers.contains_key(&(facet, name));
        let absent = statement::required(facts)
            .into_iter()
            .map(|name| (FacetKind::BehaviorModel, name))
            .chain(statement::COSTS.map(|name| (FacetKind::TimingModel, name)))
            .find(|(facet, name)| missing(*facet, name));
        if let Some((_, name)) = absent {
            return Err(Refusal::new(
                Code::Field,
                &port.path,
                name,
                None,
                format!(
                    "under {}, `{}` supplies `switch`, so it states `{name}`, `unknown` if need be (§14.4)",
                    describe(selection),
                    port.id
                ),
            ));
        }
        Ok(())
    }
}

/// A selection in words.
fn describe(selection: Selection<'_>) -> String {
    match selection.target {
        Some(target) => format!("profile `{}` on target `{target}`", selection.profile),
        None => format!("profile `{}` with no target", selection.profile),
    }
}

/// Two records supplying one name under one selection.
fn conflict(selection: Selection<'_>, facet: FacetKind, name: &str, a: &str, b: &str) -> Refusal {
    Refusal::new(
        Code::Conflict,
        "catalog",
        &format!("{} {name}", facet.as_str()),
        None,
        format!(
            "under {}, `{a}` and `{b}` both supply `{name}`: a source conflict is investigated, not averaged",
            describe(selection)
        ),
    )
}
