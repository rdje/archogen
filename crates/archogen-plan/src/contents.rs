//! `archogen-runtime-plan/1` read back (`docs/decisions/decision_runtime-plan.md` §3; leaf `M4.1.4.2`): what a consumer of
//! a plan reads, typed, from the one canonical text.
//!
//! ⭐ **The text is the plan, and this reads exactly it.** Each line is one form, its tokens parted by a single space, and
//! the forms come in §3's order: a line is taken only if writing back what was read gives that line's bytes, so a plan has
//! one spelling, and a value no writer writes, a duration not reduced or an integer with a leading zero, is refused rather
//! than normalised. Every plan the lowering writes goes through here before it is returned, so the corpus and the bounded
//! checker hold this reader to its writer on every system they generate.
//!
//! What a reader must look inside is typed — a task's timing and its trigger, a route's interrupt and separation, a
//! context — and a name, a requester or a node stays the text §3 writes it as, since a consumer compares those whole.

use eadl_model::rational::Rational;

/// One form of a line: an atom, a quoted text, or a list.
#[derive(Debug, Clone, PartialEq, Eq)]
enum Tree {
    Atom(String),
    Quoted(String),
    List(Vec<Tree>),
}

impl Tree {
    /// The form as §3 writes it.
    fn text(&self) -> String {
        match self {
            Self::Atom(atom) => atom.clone(),
            Self::Quoted(text) => format!("\"{text}\""),
            Self::List(items) => {
                let inner: Vec<String> = items.iter().map(Self::text).collect();
                format!("({})", inner.join(" "))
            }
        }
    }

    /// Its head, for a list opening with an atom.
    fn head(&self) -> Option<&str> {
        match self {
            Self::List(items) => match items.first() {
                Some(Self::Atom(head)) => Some(head),
                _ => None,
            },
            _ => None,
        }
    }

    /// A list's items after its head, for the list `head` opens.
    fn under(&self, head: &str) -> Result<&[Self], String> {
        match self {
            Self::List(items) if self.head() == Some(head) => Ok(&items[1..]),
            _ => Err(format!("expected `({head} …)`, found `{}`", self.text())),
        }
    }

    fn atom(&self) -> Result<&str, String> {
        match self {
            Self::Atom(atom) => Ok(atom),
            _ => Err(format!("expected a name, found `{}`", self.text())),
        }
    }
}

/// One line's form: its tokens read, with nothing left over. The caller holds the line's bytes to what this read.
fn parse(line: &str) -> Result<Tree, String> {
    let bytes = line.as_bytes();
    let mut at = 0;
    let tree = parse_form(line, bytes, &mut at)?;
    if at == bytes.len() {
        Ok(tree)
    } else {
        Err(format!("text after the form, at byte {at}"))
    }
}

fn parse_form(line: &str, bytes: &[u8], at: &mut usize) -> Result<Tree, String> {
    match bytes.get(*at) {
        Some(b'(') => {
            *at += 1;
            let mut items = Vec::new();
            loop {
                match bytes.get(*at) {
                    Some(b')') => {
                        *at += 1;
                        return Ok(Tree::List(items));
                    }
                    None => return Err("a list is not closed".to_owned()),
                    Some(b' ') => {
                        *at += 1;
                    }
                    Some(_) => items.push(parse_form(line, bytes, at)?),
                }
            }
        }
        Some(b'"') => {
            let start = *at + 1;
            let end = line[start..]
                .find('"')
                .ok_or_else(|| "a text is not closed".to_owned())?;
            *at = start + end + 1;
            Ok(Tree::Quoted(line[start..start + end].to_owned()))
        }
        Some(_) => {
            let start = *at;
            while bytes
                .get(*at)
                .is_some_and(|b| !matches!(b, b'(' | b')' | b'"' | b' '))
            {
                *at += 1;
            }
            Ok(Tree::Atom(line[start..*at].to_owned()))
        }
        None => Err("a form was expected".to_owned()),
    }
}

/// An interrupt, as §2 writes one.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Item {
    /// The release timer.
    Timer,
    /// A timer route, by its provider.
    TimerRoute(String),
    /// A source, by its provider.
    Source(String),
}

fn item(tree: &Tree) -> Result<Item, String> {
    match tree {
        Tree::Atom(atom) if atom == "timer" => Ok(Item::Timer),
        _ => match (tree.head(), tree) {
            (Some("timer-route"), Tree::List(items)) if items.len() == 2 => {
                Ok(Item::TimerRoute(items[1].atom()?.to_owned()))
            }
            (Some("source"), Tree::List(items)) if items.len() == 2 => {
                Ok(Item::Source(items[1].atom()?.to_owned()))
            }
            _ => Err(format!("no interrupt: `{}`", tree.text())),
        },
    }
}

/// `INT`: a decimal integer with no sign and no leading zero.
fn int(tree: &Tree) -> Result<u64, String> {
    let text = tree.atom()?;
    let value: u64 = text
        .parse()
        .map_err(|_| format!("`{text}` is no decimal integer"))?;
    if value.to_string() == text {
        Ok(value)
    } else {
        Err(format!("`{text}` is not written as §3 writes an integer"))
    }
}

/// §2.2's duration, `N s` or `N/D s`, reduced: the two atoms it is written as.
fn duration(number: &Tree, unit: &Tree) -> Result<Rational, String> {
    if unit.atom()? != "s" {
        return Err(format!("`{}` is no duration's unit", unit.text()));
    }
    let text = number.atom()?;
    let part = |part: &str| -> Result<i128, String> {
        part.parse()
            .map_err(|_| format!("`{text}` is no duration in seconds"))
    };
    let value = match text.split_once('/') {
        Some((numerator, denominator)) => Rational::new(part(numerator)?, part(denominator)?)
            .ok_or_else(|| format!("`{text}` has no value"))?,
        None => Rational::new(part(text)?, 1).ok_or_else(|| format!("`{text}` has no value"))?,
    };
    if written(value) == text {
        Ok(value)
    } else {
        Err(format!("`{text}` is not a reduced duration"))
    }
}

/// A duration's number, as §2.2 writes it.
fn written(value: Rational) -> String {
    if value.denominator() == 1 {
        value.numerator().to_string()
    } else {
        format!("{}/{}", value.numerator(), value.denominator())
    }
}

/// `(HEAD DUR)`'s duration.
fn duration_under(tree: &Tree, head: &str) -> Result<Rational, String> {
    match tree.under(head)? {
        [number, unit] => duration(number, unit),
        _ => Err(format!("`{}` holds no one duration", tree.text())),
    }
}

/// The names `(HEAD NAME …)` holds, each by its text.
fn texts_under(tree: &Tree, head: &str) -> Result<Vec<String>, String> {
    Ok(tree.under(head)?.iter().map(Tree::text).collect())
}

/// `(HEAD VALUE)`'s one value, by its text.
fn one_under(tree: &Tree, head: &str) -> Result<String, String> {
    match tree.under(head)? {
        [value] => Ok(value.text()),
        _ => Err(format!("`{}` holds no one value", tree.text())),
    }
}

/// How a task is released (§2.2).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Release {
    /// Periodically.
    Period(Rational),
    /// At arrivals at least this far apart.
    MinSeparation(Rational),
}

/// What releases a task (§3's `TRIGGER`).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Trigger {
    /// The release timer.
    Timer,
    /// A source, on every arrival.
    Source(String),
    /// What the plan could not infer: the cause, by its text.
    Unknown(String),
}

/// A `task` line.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Task {
    /// Its runtime index (§2.2).
    pub index: u64,
    /// Its stable logical identity, `(task SYSTEM NAME)`.
    pub identity: String,
    /// Its name in its system.
    pub name: String,
    /// Its rank.
    pub rank: u64,
    /// Its release.
    pub release: Release,
    /// Its relative deadline.
    pub deadline: Rational,
    /// Whether the description measures the deadline from the release, `(deadline-from release)`.
    pub deadline_from_release: bool,
    /// Its release jitter.
    pub jitter: Rational,
    /// Its overrun policy, as the description writes it.
    pub overrun: String,
    /// What releases it.
    pub released_by: Trigger,
    /// The nodes it uses, each by its text.
    pub uses: Vec<String>,
}

fn task(items: &[Tree]) -> Result<Task, String> {
    let [index, identity, rank, release, deadline, reference, jitter, overrun, released_by, uses] =
        items
    else {
        return Err("a task line holds ten forms".to_owned());
    };
    let release = match release.head() {
        Some("period") => Release::Period(duration_under(release, "period")?),
        Some("min-separation") => {
            Release::MinSeparation(duration_under(release, "min-separation")?)
        }
        _ => return Err(format!("no release: `{}`", release.text())),
    };
    let (Release::Period(separation) | Release::MinSeparation(separation)) = release;
    if !separation.is_positive() {
        return Err("a release's duration is never zero (§1)".to_owned());
    }
    let name = match identity.under("task")? {
        [_, name] => name.atom()?.to_owned(),
        _ => return Err(format!("no task: `{}`", identity.text())),
    };
    let deadline_from_release = match reference.under("deadline-from")? {
        [] => false,
        [from] if from.atom()? == "release" => true,
        _ => return Err(format!("no reference: `{}`", reference.text())),
    };
    let trigger = match released_by.under("released-by")? {
        [Tree::Atom(timer)] if timer == "timer" => Trigger::Timer,
        [cause] => match cause.head() {
            Some("source") => match cause.under("source")? {
                [provider, every] if every.atom()? == "every" => {
                    Trigger::Source(provider.atom()?.to_owned())
                }
                _ => return Err(format!("no trigger: `{}`", cause.text())),
            },
            Some("unknown") => match cause.under("unknown")? {
                [why] => Trigger::Unknown(why.text()),
                _ => return Err(format!("no trigger: `{}`", cause.text())),
            },
            _ => return Err(format!("no trigger: `{}`", cause.text())),
        },
        _ => return Err(format!("no trigger: `{}`", released_by.text())),
    };
    Ok(Task {
        index: int(index)?,
        identity: identity.text(),
        name,
        rank: match rank.under("rank")? {
            [rank] => int(rank)?,
            _ => return Err(format!("no rank: `{}`", rank.text())),
        },
        release,
        deadline: duration_under(deadline, "deadline")?,
        deadline_from_release,
        jitter: duration_under(jitter, "jitter")?,
        overrun: one_under(overrun, "overrun")?,
        released_by: trigger,
        uses: texts_under(uses, "uses")?,
    })
}

/// A `bind` line, as `archogen-assignment/2` writes it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Bind {
    /// The requester, by its text.
    pub requester: String,
    /// The fact.
    pub fact: String,
    /// The provider, by its text.
    pub provider: String,
}

/// An `instance` line.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Instance {
    /// Its `(adapter …)` text.
    pub text: String,
    /// What it is obliged, each `(obliged FACT …)` by its text.
    pub obliged: Vec<String>,
    /// Its kind's obligations.
    pub obligations: Vec<String>,
    /// Its kind's costs.
    pub costs: Vec<String>,
}

fn instance(items: &[Tree]) -> Result<Instance, String> {
    let [text, obliged @ .., obligations, costs] = items else {
        return Err("an instance line holds its text, its obligations and its costs".to_owned());
    };
    for one in obliged {
        one.under("obliged")?;
    }
    Ok(Instance {
        text: text.text(),
        obliged: obliged.iter().map(Tree::text).collect(),
        obligations: texts_under(obligations, "obligations")?,
        costs: texts_under(costs, "costs")?,
    })
}

/// What a route says of its arrivals (§3's `separation`).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Separation {
    /// A stated minimum.
    Of(Rational),
    /// Stated by nothing.
    Unknown,
    /// Declared absent.
    Absent,
    /// A timer route's, which its requester programs.
    Programmed,
}

/// A `route` line.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Route {
    /// Its rank among the routes.
    pub rank: u64,
    /// Its interrupt.
    pub item: Item,
    /// Its arrivals' separation.
    pub separation: Separation,
    /// The level its interrupt is delivered at.
    pub level: String,
    /// Who relies on it, each by its text.
    pub requesters: Vec<String>,
    /// Each cost on it: the instance's text and the cost's name in the kind.
    pub costs: Vec<(String, String)>,
}

fn route(items: &[Tree]) -> Result<Route, String> {
    let [rank, interrupt, separation, level, deliverable, requesters, costs] = items else {
        return Err("a route line holds seven forms".to_owned());
    };
    let item = item(interrupt)?;
    let separation = match separation.under("separation")? {
        [Tree::Atom(word)] if word == "unknown" => Separation::Unknown,
        [Tree::Atom(word)] if word == "absent" => Separation::Absent,
        [Tree::Atom(word)] if word == "programmed" => Separation::Programmed,
        [number, unit] => Separation::Of(duration(number, unit)?),
        _ => return Err(format!("no separation: `{}`", separation.text())),
    };
    match (&item, separation) {
        (Item::Timer, _) => return Err("the release timer is no route".to_owned()),
        (Item::Source(_), Separation::Programmed) => {
            return Err("a source's separation is never programmed".to_owned())
        }
        _ => {}
    }
    if one_under(deliverable, "deliverable")? != "yes" {
        return Err(format!("no route: `{}`", deliverable.text()));
    }
    let costs = costs
        .under("costs")?
        .iter()
        .map(|cost| match cost {
            Tree::List(pair) => match pair.as_slice() {
                [instance, name] => Ok((instance.text(), name.atom()?.to_owned())),
                _ => Err(format!("no cost: `{}`", cost.text())),
            },
            _ => Err(format!("no cost: `{}`", cost.text())),
        })
        .collect::<Result<_, String>>()?;
    Ok(Route {
        rank: int(rank)?,
        item,
        separation,
        level: one_under(level, "level")?,
        requesters: texts_under(requesters, "requesters")?,
        costs,
    })
}

/// A node's kind (§3's `NODEKIND`).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NodeKind {
    /// A block.
    Block,
    /// A platform.
    Platform,
    /// An adapter's instance.
    Instance,
    /// A service.
    Service,
    /// A policy.
    Policy,
}

/// A context a node or a task runs in (§3's `CONTEXT`).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Context {
    /// A job.
    Job,
    /// An interrupt's routine.
    Interrupt(Item),
    /// Initialisation.
    Initialisation,
}

fn context(tree: &Tree) -> Result<Context, String> {
    match tree {
        Tree::Atom(word) if word == "job" => Ok(Context::Job),
        Tree::Atom(word) if word == "initialisation" => Ok(Context::Initialisation),
        _ => match tree.under("interrupt")? {
            [interrupt] => match item(interrupt)? {
                Item::Timer => Err("the release timer's routine runs no node".to_owned()),
                routed => Ok(Context::Interrupt(routed)),
            },
            _ => Err(format!("no context: `{}`", tree.text())),
        },
    }
}

/// A `depends` line: `from` initialises before `to`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Depends {
    /// The node initialised first, by its text.
    pub from: String,
    /// The node initialised after it, by its text.
    pub to: String,
    /// Why, each `REASON` by its text.
    pub because: Vec<String>,
}

/// An `obligation` line.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ObligationLine {
    /// Its kind.
    pub kind: String,
    /// Its subject, by its text.
    pub subject: String,
    /// The leaves that discharge it.
    pub discharged_by: Vec<String>,
}

/// An `owed` line.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Owed {
    /// Its subject, by its text.
    pub subject: String,
    /// The fact owed.
    pub fact: String,
    /// Who owes it.
    pub by: String,
}

/// What a plan holds, read from its text (§3).
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Contents {
    /// The profile it was lowered under.
    pub profile: String,
    /// Its system.
    pub system: String,
    /// The digest of the elaborated program, 64 hexadecimal digits.
    pub program: String,
    /// The digest of the assignment record, 64 hexadecimal digits.
    pub assignment: String,
    /// The adapter catalog's version.
    pub adapters: String,
    /// Its tasks, in runtime order.
    pub tasks: Vec<Task>,
    /// The record's bindings.
    pub binds: Vec<Bind>,
    /// Its adapters' instances.
    pub instances: Vec<Instance>,
    /// The tasks the simulator's timer releases, each by its identity.
    pub timer_releases: Vec<String>,
    /// Its routes.
    pub routes: Vec<Route>,
    /// The interrupts it enables.
    pub enabled: Vec<Item>,
    /// Its nodes, each by its text, and each's kind.
    pub nodes: Vec<(String, NodeKind)>,
    /// What runs in which contexts, by the subject's text.
    pub contexts: Vec<(String, Vec<Context>)>,
    /// The initialisation graph's edges.
    pub depends: Vec<Depends>,
    /// The initialisation's steps, each the nodes of one component, by their text.
    pub steps: Vec<Vec<String>>,
    /// What the plan's realization must discharge.
    pub obligations: Vec<ObligationLine>,
    /// What each owner owes.
    pub owed: Vec<Owed>,
    /// Whether nothing is owed.
    pub complete: bool,
}

/// `(format …)`'s text and the digests' spelling (§3).
const HEX_DIGITS: usize = 64;

fn hex_under(tree: &Tree, head: &str) -> Result<String, String> {
    match tree.under(head)? {
        [Tree::Quoted(hex)]
            if hex.len() == HEX_DIGITS
                && hex
                    .bytes()
                    .all(|b| b.is_ascii_digit() || (b'a'..=b'f').contains(&b)) =>
        {
            Ok(hex.clone())
        }
        _ => Err(format!("`{}` holds no digest", tree.text())),
    }
}

/// The forms in §3's order, each section's head: a line's head may not fall back to an earlier section.
const ORDER: [&str; 20] = [
    "format",
    "profile",
    "system",
    "program",
    "assignment",
    "adapters",
    "task",
    "bind",
    "instance",
    "timer",
    "route",
    "enabled",
    "interrupt-model",
    "node",
    "context",
    "depends",
    "step",
    "obligation",
    "owed",
    "complete",
];

/// The forms written exactly once; the others are written once for each thing they say (§3).
const ONCE: [&str; 10] = [
    "format",
    "profile",
    "system",
    "program",
    "assignment",
    "adapters",
    "timer",
    "enabled",
    "interrupt-model",
    "complete",
];

/// One line taken into what has been read.
#[allow(clippy::too_many_lines)]
fn take(head: &str, items: &[Tree], form: &Tree, out: &mut Contents) -> Result<(), String> {
    match head {
        "format" => match items {
            [Tree::Quoted(format)] if format == crate::FORMAT => {}
            _ => return Err(format!("not `{}`: `{}`", crate::FORMAT, form.text())),
        },
        "profile" => out.profile = one_under(form, head)?,
        "system" => out.system = one_under(form, head)?,
        "program" => out.program = hex_under(form, head)?,
        "assignment" => out.assignment = hex_under(form, head)?,
        "adapters" => out.adapters = one_under(form, head)?,
        "task" => out.tasks.push(task(items)?),
        "bind" => match items {
            [requester, fact, provider] => out.binds.push(Bind {
                requester: requester.text(),
                fact: fact.atom()?.to_owned(),
                provider: provider.text(),
            }),
            _ => return Err(format!("no binding: `{}`", form.text())),
        },
        "instance" => out.instances.push(instance(items)?),
        "timer" => match items {
            [Tree::Atom(simulator), releases] if simulator == "simulator" => {
                out.timer_releases = texts_under(releases, "releases")?;
            }
            _ => return Err(format!("no timer: `{}`", form.text())),
        },
        "route" => out.routes.push(route(items)?),
        "enabled" => {
            out.enabled = items.iter().map(item).collect::<Result<_, _>>()?;
        }
        "interrupt-model" => {
            if form.text() != "(interrupt-model (nesting no) (masking all))" {
                return Err(format!("no interrupt model: `{}`", form.text()));
            }
        }
        "node" => match items {
            [node, Tree::Atom(kind)] => {
                let kind = match kind.as_str() {
                    "block" => NodeKind::Block,
                    "platform" => NodeKind::Platform,
                    "instance" => NodeKind::Instance,
                    "service" => NodeKind::Service,
                    "policy" => NodeKind::Policy,
                    _ => return Err(format!("no node's kind: `{kind}`")),
                };
                out.nodes.push((node.text(), kind));
            }
            _ => return Err(format!("no node: `{}`", form.text())),
        },
        "context" => match items {
            [subject, contexts @ ..] => out.contexts.push((
                subject.text(),
                contexts.iter().map(context).collect::<Result<_, _>>()?,
            )),
            [] => return Err("a context line names its subject".to_owned()),
        },
        "depends" => match items {
            [from, to, because] => out.depends.push(Depends {
                from: from.text(),
                to: to.text(),
                because: texts_under(because, "because")?,
            }),
            _ => return Err(format!("no edge: `{}`", form.text())),
        },
        "step" => match items {
            [number, nodes @ ..] => {
                let number = int(number)?;
                if number != out.steps.len() as u64 + 1 {
                    return Err(format!("step {number} out of order"));
                }
                out.steps.push(nodes.iter().map(Tree::text).collect());
            }
            [] => return Err("a step line is numbered".to_owned()),
        },
        "obligation" => match items {
            [kind, subject, discharged] => out.obligations.push(ObligationLine {
                kind: kind.atom()?.to_owned(),
                subject: subject.text(),
                discharged_by: texts_under(discharged, "discharged-by")?,
            }),
            _ => return Err(format!("no obligation: `{}`", form.text())),
        },
        "owed" => match items {
            [subject, fact, by] => out.owed.push(Owed {
                subject: subject.text(),
                fact: fact.atom()?.to_owned(),
                by: one_under(by, "by")?,
            }),
            _ => return Err(format!("no owed fact: `{}`", form.text())),
        },
        "complete" => match items {
            [Tree::Atom(word)] if word == "yes" => out.complete = true,
            [Tree::Atom(word)] if word == "no" => out.complete = false,
            _ => return Err(format!("not `yes` or `no`: `{}`", form.text())),
        },
        _ => return Err(format!("no form of the plan: `{head}`")),
    }
    Ok(())
}

/// What `text` holds, or why it is not the canonical text of a plan, naming the line.
///
/// # Errors
///
/// A line that does not read, is not written as §3 writes it, or stands out of §3's order; a form written once missing
/// or written twice; no task (§1); a text with no last line end, or with a blank line.
pub fn read(text: &str) -> Result<Contents, String> {
    let Some(body) = text.strip_suffix('\n') else {
        return Err("a plan's last line ends with a newline".to_owned());
    };
    let mut out = Contents::default();
    let mut section = 0;
    let mut seen = [false; ORDER.len()];
    for (number, line) in body.split('\n').enumerate() {
        let at = |why: String| format!("line {}: {why}", number + 1);
        let form = parse(line).map_err(&at)?;
        if form.text() != line {
            return Err(at("not written as §3 writes it".to_owned()));
        }
        let head = form
            .head()
            .ok_or_else(|| at("no form of the plan".to_owned()))?;
        let place = ORDER
            .iter()
            .position(|known| *known == head)
            .ok_or_else(|| at(format!("no form of the plan: `{head}`")))?;
        if place < section {
            return Err(at(format!("`{head}` stands after `{}`", ORDER[section])));
        }
        if seen[place] && ONCE.contains(&head) {
            return Err(at(format!("`{head}` is written once")));
        }
        section = place;
        seen[place] = true;
        let Tree::List(items) = &form else {
            return Err(at("no form of the plan".to_owned()));
        };
        take(head, &items[1..], &form, &mut out).map_err(&at)?;
    }
    for head in ONCE {
        let place = ORDER
            .iter()
            .position(|known| *known == head)
            .ok_or_else(|| format!("no form of the plan: `{head}`"))?;
        if !seen[place] {
            return Err(format!("a plan writes `{head}`"));
        }
    }
    if out.tasks.is_empty() {
        return Err("a plan holds at least one task (§1)".to_owned());
    }
    Ok(out)
}
