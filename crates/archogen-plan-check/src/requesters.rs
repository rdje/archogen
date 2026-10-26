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
/// reach through `uses` and `needs`, at any depth, for what it writes at its own level — each with its side.
///
/// # Errors
///
/// The first side the model refuses, with its declaration.
pub fn served<'p>(program: &'p Program, system: &str) -> Result<Served<'p>, Refused> {
    let mut declarations: BTreeMap<String, (usize, &Form)> = BTreeMap::new();
    for instance in &program.instances {
        for form in &instance.declarations {
            if let Some(name) = form.items().get(1).and_then(Form::as_symbol) {
                declarations.insert(instance.qualify(name), (instance.id, form));
            }
        }
    }
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
    Ok(served)
}
