//! Format `/2`'s interrupt controller and routes (`docs/decisions/decision_runtime-plan-format-2.md` §3 and §6; leaf
//! `M4.1.7.6.2`): the order routes are ranked in, which reach the controller, the priority each rank takes, and the
//! `(enabled …)` line. `tests/format2_model.rs` holds each to its transcription over every small input it enumerates.

/// §3: routes ranked as the release alarm's §3 ranks them, by their value, what states no value last, ties broken by
/// the interrupt's text as an item writes it, `(alarm NODE)` or `(source NAME)`, never by `NODE`'s (RP10-2). Each item is
/// its value, where one is stated, and its text; the result is the items' texts in rank order.
#[must_use]
pub fn ranked<V: Ord, I: Ord>(mut items: Vec<(Option<V>, I)>) -> Vec<I> {
    items.sort_by(|a, b| (a.0.is_none(), &a.0, &a.1).cmp(&(b.0.is_none(), &b.0, &b.1)));
    items.into_iter().map(|(_, item)| item).collect()
}

/// §6: which routes reach the controller is each one's `external`, the realization's statement: a route whose file states
/// `no` is outside the composition and refuses the system, and one owed counts as reaching it (RP22-1). `Ok` counts the
/// routes that reach it, every one; `Err` names the first stated `no`, by its place.
///
/// # Errors
///
/// The place of the first route whose `external` is stated `no`.
pub fn reaching(external: &[Option<bool>]) -> Result<u64, usize> {
    match external.iter().position(|stated| *stated == Some(false)) {
        Some(at) => Err(at),
        None => Ok(u64::try_from(external.len()).unwrap_or(u64::MAX)),
    }
}

/// §6, "Ranks onto levels": "Rank 1 takes the highest level, and each rank after it the next", "`PRIORITY` is
/// `controller-levels` less the number of such routes ahead of it", and "Where the routes outnumber the levels the system
/// is refused". Nothing where the routes outnumber the levels or `rank` is no rank of `routes`.
#[must_use]
pub fn priority(levels: u64, routes: u64, rank: u64) -> Option<u64> {
    (routes <= levels && (1..=routes).contains(&rank)).then(|| levels - (rank - 1))
}

/// §3: the `(enabled …)` line's items: `timer`, then each route by rank, and no `timer` where the plan writes `(timer
/// none)`, the release alarm's §1 and model §8 rule 8 enabling no release timer a system has none of (RP11-1).
#[must_use]
pub fn enabled<'a>(timer: bool, routes: &[&'a str]) -> Vec<&'a str> {
    let timer: &[&str] = if timer { &["timer"] } else { &[] };
    timer.iter().chain(routes).copied().collect()
}
