//! Who requires, and where (leaf `M3.5.4.4`), from `docs/semantics/model.md` §8 rules 1 and 7 alone: a system's own
//! level, each of its tasks, and every declaration they reach through `uses` and `needs`, each a requester with its
//! side — every requirement it writes, located by the checker over the description's forms at the positions the model
//! reads a side from, by the model's own readers, so a test holds that every requirement the model reads is located
//! once.

use std::collections::{BTreeMap, BTreeSet};

use eadl_front::{Form, Program, Span};

use crate::assignment::Requester;
use crate::model::requirement::{self, NotJudged, Requirement};

/// One requirement a requester writes: as the model reads it, and where it is written.
#[derive(Debug, Clone)]
pub struct Located {
    /// The requirement.
    pub requirement: Requirement,
    /// Where it is written: the constraint, or the operand of `needs`.
    pub span: Span,
}

/// What one requester writes.
#[derive(Debug, Clone)]
pub struct Side<'f> {
    /// The task, for a system's task; none for the declaration's own level.
    pub task: Option<&'f str>,
    /// Every requirement it writes, each located once.
    pub requirements: Vec<Located>,
    /// Every operand of its `uses` and `needs` that is a name, where it is written: what it may reach.
    pub names: Vec<(&'f str, Span)>,
}

/// The positions one requester's items are read from: the `requires` clauses and the `needs` and `uses` lists.
#[derive(Default)]
struct Positions<'f> {
    clauses: Vec<&'f Form>,
    lists: Vec<&'f Form>,
}

/// The `needs` and `uses` lists and the `requires` clauses among `form`'s items, at any depth, never looking inside
/// one: the model's own positions.
fn walk<'f>(form: &'f Form, at: &mut Positions<'f>) {
    for item in form.items().iter().skip(1) {
        match item.head() {
            Some("needs" | "uses") => at.lists.push(item),
            Some("requires") => at.clauses.push(item),
            _ => {
                if matches!(item, Form::List { .. }) {
                    walk(item, at);
                }
            }
        }
    }
}

/// The requesters one declaration writes for (rule 1): its own level, and, for a system, each of its tasks — every
/// item at the positions the model reads the declaration's side from, read by the model's own readers.
///
/// # Errors
///
/// The model's refusal of the declaration's side ([`requirement::read_side`]).
pub fn sides(decl: &Form) -> Result<Vec<Side<'_>>, NotJudged> {
    requirement::read_side(decl)?;
    let system = decl.head() == Some("defsystem");
    let mut own = Positions::default();
    let mut tasks = Vec::new();
    for clause in decl.items().iter().skip(1) {
        match clause.head() {
            None | Some("offers" | "absent" | "refines") => {}
            Some("needs" | "uses") => own.lists.push(clause),
            Some("requires") => own.clauses.push(clause),
            // What else a `platform` clause holds the model's reading has refused.
            Some("platform") => {
                for item in clause.items().iter().skip(1) {
                    match item.head() {
                        Some("needs" | "uses") => own.lists.push(item),
                        Some("requires") => own.clauses.push(item),
                        _ => {}
                    }
                }
            }
            Some("task") if system => {
                let mut at = Positions::default();
                walk(clause, &mut at);
                let name = clause.items().get(1).and_then(Form::as_symbol);
                tasks.push((name.unwrap_or("<unnamed>"), at));
            }
            Some(_) => walk(clause, &mut own),
        }
    }
    let mut out = vec![side(None, own)?];
    for (task, at) in tasks {
        out.push(side(Some(task), at)?);
    }
    Ok(out)
}

/// One requester's items read: each clause's own constraints and the lists inside them, a clause nested in one read as
/// its own; each operand of `needs` that is a fact a presence requirement.
fn side<'f>(task: Option<&'f str>, mut at: Positions<'f>) -> Result<Side<'f>, NotJudged> {
    let mut constraints = Vec::new();
    while let Some(clause) = at.clauses.pop() {
        constraints.extend(
            clause
                .items()
                .iter()
                .skip(1)
                .filter(|i| !matches!(i.head(), Some("needs" | "uses" | "requires"))),
        );
        walk(clause, &mut at);
    }
    let (mut requirements, mut names) = (Vec::new(), Vec::new());
    let mut read = |read: Result<Requirement, NotJudged>, span| match read {
        Ok(requirement) => {
            requirements.push(Located { requirement, span });
            Ok(())
        }
        // A name the vocabulary does not declare: presence's and the closure's (§1).
        Err(NotJudged::NotAFact) => Ok(()),
        Err(e) => Err(e),
    };
    for list in at.lists {
        let needs = list.head() == Some("needs");
        for operand in list.items().iter().skip(1) {
            if let Some(name) = operand.as_symbol() {
                names.push((name, operand.span()));
            }
            if needs {
                read(requirement::read_needs(operand), operand.span())?;
            }
        }
    }
    for item in constraints {
        read(requirement::read_constraint(item), item.span())?;
    }
    Ok(Side {
        task,
        requirements,
        names,
    })
}

/// The requesters one system serves, each with its side.
#[derive(Debug, Clone, Default)]
pub struct Served<'p> {
    /// Each requester, with what it writes.
    pub sides: BTreeMap<Requester, Side<'p>>,
    /// The requesters each one names through its `uses` and `needs`, directly.
    pub names: BTreeMap<Requester, BTreeSet<Requester>>,
    /// The blocks and platforms counted as requesters, by [`served`]: those a block or platform reached and not among them writes bind nothing.
    pub parts: BTreeSet<String>,
}

impl Served<'_> {
    /// Every requester `requester` reaches through `uses` and `needs`, at any depth, itself apart.
    #[must_use]
    pub fn reach(&self, requester: &Requester) -> BTreeSet<&Requester> {
        let mut reached = BTreeSet::new();
        let mut todo: Vec<&Requester> = self.names.get(requester).into_iter().flatten().collect();
        while let Some(next) = todo.pop() {
            if next != requester && reached.insert(next) {
                todo.extend(self.names.get(next).into_iter().flatten());
            }
        }
        reached
    }
}

/// A side the model refuses: the declaration's qualified name, its span and the refusal.
pub type Refused = (String, Span, NotJudged);

fn refusal<'a>(name: &'a str, form: &'a Form) -> impl FnOnce(NotJudged) -> Refused + 'a {
    move |e| (name.to_owned(), form.span(), e)
}

/// The requesters system `system` serves (rules 1, 7): its own level, each of its tasks, and every declaration they
/// reach through `uses` and `needs`, at any depth, for what it writes at its own level — each with its side; but a block or a platform only as it may be
/// bound (model §2 rule 3, §8 rule 5): one the system names itself, in its `platform` clause, its own `uses` and `needs` or its tasks', or a requester it
/// serves that is no part names, or one that offers a fact a requester counted requires, a fact the vocabulary derives such a fact of, or a fact an adapter
/// kind adding such a fact needs beneath, or one a part counted names by `needs`, to a fixed point. A part named only by another's composition and
/// offering nothing asked is no requester, its needs binding nothing (`ROADMAP.md` §5.3; leaf `M3.4.17.6`).
///
/// # Errors
///
/// The first side the model refuses, with its declaration.
pub fn served<'p>(program: &'p Program, system: &str) -> Result<Served<'p>, Refused> {
    let declarations = declarations(program);
    let mut served = Served::default();
    let Some(&(id, form)) = declarations.get(system) else {
        return Ok(served);
    };
    let mut todo = Vec::new();
    for side in sides(form).map_err(refusal(system, form))? {
        let requester = side.task.map_or_else(
            || Requester::Named(system.to_owned()),
            |task| Requester::Task {
                system: system.to_owned(),
                task: task.to_owned(),
            },
        );
        todo.push((requester, id, side));
    }
    let mut seen = BTreeSet::from([system.to_owned()]);
    while let Some((requester, id, side)) = todo.pop() {
        let mut named = BTreeSet::new();
        for (name, span) in &side.names {
            let Ok(Some(qualified)) = program.resolve(id, name, *span) else {
                continue;
            };
            let Some(&(reached, form)) = declarations.get(&qualified) else {
                continue;
            };
            named.insert(Requester::Named(qualified.clone()));
            if seen.insert(qualified.clone()) {
                // What a declaration writes at its own level, outside any task of its own (rule 1).
                let own = sides(form)
                    .map_err(refusal(&qualified, form))?
                    .into_iter()
                    .find(|s| s.task.is_none())
                    .expect("a declaration's own level is a side");
                todo.push((Requester::Named(qualified), reached, own));
            }
        }
        served.names.insert(requester.clone(), named);
        served.sides.insert(requester, side);
    }
    let is_part = |name: &str| {
        declarations
            .get(name)
            .is_some_and(|(_, form)| matches!(form.head(), Some("defblock" | "defplatform")))
    };
    let reached_parts: BTreeSet<String> = served
        .sides
        .keys()
        .filter_map(|requester| match requester {
            Requester::Named(name) if is_part(name) => Some(name.clone()),
            _ => None,
        })
        .collect();
    // What each part offers, by its own reading; a part whose offers do not read is counted, failing closed.
    let offers: BTreeMap<&String, Option<BTreeSet<String>>> = reached_parts
        .iter()
        .map(|name| {
            let offered = crate::model::provider::read(declarations[name].1)
                .ok()
                .map(|provider| {
                    provider
                        .stated
                        .iter()
                        .filter(|(_, stated)| {
                            !matches!(stated, crate::model::provider::Stated::Absent)
                        })
                        .map(|(fact, _)| fact.clone())
                        .collect()
                });
            (name, offered)
        })
        .collect();
    // Each part a part names by `needs`, an edge presence follows where a `uses` composes.
    let needed: BTreeMap<&String, Vec<String>> = reached_parts
        .iter()
        .map(|name| {
            let (at, part) = declarations[name];
            let mut written = Vec::new();
            needs_names(part, &mut written);
            let named = resolve(program, at, &written)
                .into_iter()
                .filter(|named| reached_parts.contains(named))
                .collect();
            (name, named)
        })
        .collect();
    let mut seeds = Vec::new();
    names(form, &mut seeds, true);
    let mut counted: BTreeSet<String> = resolve(program, id, &seeds)
        .into_iter()
        .filter(|name| reached_parts.contains(name))
        .collect();
    counted.extend(
        offers
            .iter()
            .filter(|(_, offered)| offered.is_none())
            .map(|(name, _)| (*name).clone()),
    );
    // And each part a served requester that is no part names, a service's `uses` among them, as presence reads it: only a part's composition asks nothing.
    for (requester, named) in &served.names {
        if matches!(requester, Requester::Named(name) if reached_parts.contains(name)) {
            continue;
        }
        counted.extend(named.iter().filter_map(|named| match named {
            Requester::Named(name) if reached_parts.contains(name) => Some(name.clone()),
            _ => None,
        }));
    }
    loop {
        let mut wanted: BTreeSet<&str> = served
            .sides
            .iter()
            .filter(|(requester, _)| match requester {
                Requester::Named(name) if reached_parts.contains(name) => counted.contains(name),
                _ => true,
            })
            .flat_map(|(_, side)| {
                side.requirements
                    .iter()
                    .map(|located| fact_of(&located.requirement))
            })
            .filter(|fact| *fact != "reachable-at-privilege")
            .collect();
        // A fact the vocabulary's rule derives one of, or the `beneath` of an adapter kind adding one names, an instance over its provider binding it too.
        let mut inputs: Vec<&str> = wanted.iter().copied().collect();
        while let Some(fact) = inputs.pop() {
            let derived = crate::model::vocab::entry(fact)
                .into_iter()
                .flat_map(|entry| entry.derived_from.iter().copied());
            let beneath = crate::adapters::ADAPTERS
                .iter()
                .filter(|kind| kind.adds().contains(&fact))
                .flat_map(crate::adapters::Kind::beneath_names);
            let found: Vec<&str> = derived.chain(beneath).collect();
            for input in found {
                if wanted.insert(input) {
                    inputs.push(input);
                }
            }
        }
        let more: Vec<String> = offers
            .iter()
            .filter(|(name, offered)| {
                !counted.contains(name.as_str())
                    && offered.as_ref().is_some_and(|offered| {
                        offered.iter().any(|fact| wanted.contains(fact.as_str()))
                    })
            })
            .map(|(name, _)| (*name).clone())
            .chain(
                counted
                    .iter()
                    .filter_map(|name| needed.get(name))
                    .flatten()
                    .filter(|named| !counted.contains(*named))
                    .cloned(),
            )
            .collect();
        if more.is_empty() {
            break;
        }
        counted.extend(more);
    }
    let dropped = |requester: &Requester| matches!(requester, Requester::Named(name) if reached_parts.contains(name) && !counted.contains(name));
    served.sides.retain(|requester, _| !dropped(requester));
    served.names.retain(|requester, _| !dropped(requester));
    for named in served.names.values_mut() {
        named.retain(|requester| !dropped(requester));
    }
    served.parts = counted;
    Ok(served)
}

/// The fact a requirement is on: a group's head for a group.
fn fact_of(requirement: &Requirement) -> &str {
    match requirement {
        Requirement::Presence(fact)
        | Requirement::Constraint { fact, .. }
        | Requirement::Group { fact, .. }
        | Requirement::Statement(fact, _) => fact,
    }
}

/// Every declaration of `program` by its qualified name, with the instance that wrote it.
#[must_use]
pub fn declarations(program: &Program) -> BTreeMap<String, (usize, &Form)> {
    let mut out = BTreeMap::new();
    for instance in &program.instances {
        for form in &instance.declarations {
            if let Some(name) = form.items().get(1).and_then(Form::as_symbol) {
                out.insert(instance.qualify(name), (instance.id, form));
            }
        }
    }
    out
}

/// Rule 7: a system's hardware — the blocks and platforms its `platform` clause, its own `uses` and `needs` and its
/// tasks' name, and every block or platform those blocks and platforms name in turn through `uses` or `needs`, a clause
/// of their own or inside `requires`, at any depth, a declaration of another kind ending the path; `refines` no edge.
/// `crates/eadl-resolve/tests/system_reading.rs` holds it equal to check (5)'s reading and the generator's.
#[must_use]
pub fn hardware(
    program: &Program,
    declarations: &BTreeMap<String, (usize, &Form)>,
    system: &str,
) -> BTreeSet<String> {
    let Some(&(id, system)) = declarations.get(system) else {
        return BTreeSet::new();
    };
    let is_hardware = |name: &String| {
        declarations
            .get(name)
            .is_some_and(|(_, form)| matches!(form.head(), Some("defblock" | "defplatform")))
    };
    let mut seeds = Vec::new();
    names(system, &mut seeds, true);
    let mut reached = BTreeSet::new();
    let mut todo: Vec<String> = resolve(program, id, &seeds)
        .into_iter()
        .filter(is_hardware)
        .collect();
    while let Some(provider) = todo.pop() {
        if !reached.insert(provider.clone()) {
            continue;
        }
        let (pid, form) = declarations[&provider];
        let mut named = Vec::new();
        names(form, &mut named, false);
        todo.extend(
            resolve(program, pid, &named)
                .into_iter()
                .filter(is_hardware),
        );
    }
    reached
}

/// The operands of every `uses` and `needs` a declaration writes — its own, inside `requires`, and, for a system,
/// inside its `platform` clause and its tasks.
fn names<'f>(form: &'f Form, out: &mut Vec<(&'f str, Span)>, system: bool) {
    for clause in &form.items()[1..] {
        match clause.head() {
            Some("uses" | "needs") => out.extend(
                clause.items()[1..]
                    .iter()
                    .filter_map(|f| f.as_symbol().map(|name| (name, f.span()))),
            ),
            Some("requires") => names(clause, out, false),
            Some("platform" | "task") if system => names(clause, out, false),
            _ => {}
        }
    }
}

/// The names a block's or a platform's `needs` lists write, at its own level or in its `requires` (leaf `M3.4.17.6`).
fn needs_names<'f>(form: &'f Form, out: &mut Vec<(&'f str, Span)>) {
    for clause in &form.items()[1..] {
        match clause.head() {
            Some("needs") => out.extend(
                clause.items()[1..]
                    .iter()
                    .filter_map(|f| f.as_symbol().map(|name| (name, f.span()))),
            ),
            Some("requires") => needs_names(clause, out),
            _ => {}
        }
    }
}

fn resolve(program: &Program, id: usize, names: &[(&str, Span)]) -> Vec<String> {
    names
        .iter()
        .filter_map(|(name, span)| program.resolve(id, name, *span).ok().flatten())
        .collect()
}
