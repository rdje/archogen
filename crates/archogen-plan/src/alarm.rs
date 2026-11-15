//! Format `/2`'s alarms, timer and epoch lines, as procedures (`docs/decisions/decision_runtime-plan-format-2.md` §2 to
//! §4; leaf `M4.1.7.6.2`): an alarm's time base, what it owes, whether a task writing it owes the application's word,
//! how its device's compare is read, how a rate is written, and which epoch line a plan writes.
//! `tests/format2_model.rs` holds each to its transcription over every small input it enumerates.

use std::num::NonZeroU128;

/// Where a reader of an alarm's time is bound, as §3's `BASE` reads it off the plan's bindings.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Reader {
    /// At the alarm's own device.
    AtTheDevice,
    /// At an `epoch-extender` over the alarm's own device.
    ExtendedOverIt,
    /// On another device.
    OnAnother,
    /// At an `epoch-extender` over another device.
    ExtendedOverAnother,
}

/// §3, `BASE`: an alarm's time base.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Base {
    /// `counter`: the time base is the counter itself.
    Counter,
    /// `epoch`: every reader of its time reads that counter's own extended time.
    Epoch,
}

impl Base {
    /// Its text on an `(alarm …)` line.
    #[must_use]
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Counter => "counter",
            Self::Epoch => "epoch",
        }
    }
}

/// §3, `BASE`: a wake's is `counter`; any other alarm's is `epoch` where it has a reader of time and each is bound at an
/// `epoch-extender` over the alarm's own device, and `counter` otherwise (RP5-1): a value written to its compare needs no
/// `R`, no `room` and no `RA-H7` statement only where it was read as that counter's own extended time.
#[must_use]
pub fn base(wake: bool, readers: &[Reader]) -> Base {
    if !wake
        && !readers.is_empty()
        && readers
            .iter()
            .all(|reader| *reader == Reader::ExtendedOverIt)
    {
        Base::Epoch
    } else {
        Base::Counter
    }
}

/// §3, `RA-H7`: whether a task that reaches an alarm's node owes the application's statement of it, its
/// `deadlines-within-reach`: an alarm on the counter itself that is no wake, a wake being written by its own service
/// alone, which no task signs for (RP4-3).
#[must_use]
pub fn asks_application(wake: bool, on_the_counter: bool) -> bool {
    !wake && on_the_counter
}

/// §3: which of its five facts an alarm owes, by what the checker asks of it, in the order §3 names them: `floor` and
/// `service-cost` of every alarm, `step` of a wake, `reach-ahead` of an expiry whose time base is the counter itself,
/// `lag` of one on a target, a simulated alarm having none (RP13-5).
#[must_use]
pub fn owes(wake: bool, counter_itself: bool, simulated: bool) -> Vec<&'static str> {
    let mut owed = vec!["floor"];
    if wake {
        owed.push("step");
    }
    if !wake && counter_itself {
        owed.push("reach-ahead");
    }
    if !simulated {
        owed.push("lag");
    }
    owed.push("service-cost");
    owed
}

/// §2: how a device's compare is read, of its `compare-window` and its `compare-magnitude`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Reading<W> {
    /// By its window.
    Window(W),
    /// As whole numbers, `(compare-magnitude true)` and no window.
    WholeNumbers,
    /// Nothing stated.
    Unstated,
    /// `(compare-magnitude true)` beside a window: `invalid-description` (RP12-2).
    Refused,
}

/// §2: a compare is read by its window, or as whole numbers where it states `(compare-magnitude true)` and none;
/// `(compare-magnitude false)` says nothing, so beside a window the device reads by its window, and only
/// `(compare-magnitude true)` beside one is refused (RP12-2).
#[must_use]
pub fn compare_reading<W>(window: Option<W>, magnitude: Option<bool>) -> Reading<W> {
    match (window, magnitude) {
        (Some(_), Some(true)) => Reading::Refused,
        (None, Some(true)) => Reading::WholeNumbers,
        (Some(window), _) => Reading::Window(window),
        (None, _) => Reading::Unstated,
    }
}

/// §3, `R`: a rate in hertz, exact, a whole number or `N/D` in lowest terms, as `decision_runtime-plan.md` §3 writes a
/// duration: `(tick-rate 3/2)` of 1.5 Hz (RP4-7).
#[must_use]
pub fn rate(numerator: u128, denominator: NonZeroU128) -> String {
    let denominator = denominator.get();
    let (mut a, mut b) = (numerator, denominator);
    while b != 0 {
        (a, b) = (b, a % b);
    }
    match denominator / a {
        1 => (numerator / a).to_string(),
        over => format!("{}/{over}", numerator / a),
    }
}

/// §4: the epoch's line, of a hosted plan, or of a target's with a release timer or none and an alarm or none, `lead`
/// the port's `epoch-lead` as a `FIGURE` writes it, or `owed` (RP35-1): `(epoch simulator)` hosted; `(epoch after-read
/// (lead FIGURE))` under a timer; `(epoch none (lead FIGURE))` with an alarm and no timer; and `(epoch none)` with
/// neither, where nothing holds a lead.
#[must_use]
pub fn epoch_line(hosted: bool, timer: bool, alarm: bool, lead: &str) -> String {
    match (hosted, timer, alarm) {
        (true, _, _) => "(epoch simulator)".to_owned(),
        (false, true, _) => format!("(epoch after-read (lead {lead}))"),
        (false, false, true) => format!("(epoch none (lead {lead}))"),
        (false, false, false) => "(epoch none)".to_owned(),
    }
}
