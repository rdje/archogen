//! Claims: computed citations, strength and the production rule (the record's §7, `M2.7.3.6.1`).
//!
//! A claim reads, and its citations are what it read: every lookup goes through the claim, which records it, so no
//! claimant writes a citation list and none can leave out a facet it used. The closure of the citations holds every
//! facet whose bound hash enters a cited one's, with each record's contract. Admission reports the strongest verdict
//! with every reason that reaches it: `unsupported-profile` first, then, for a production claim only,
//! `not-established`. Premise 3's causes are `M2.7.3.6.2`'s; what needs an image is held so until `M4` gives one.

use std::collections::BTreeSet;

use crate::history::History;
use crate::invalidation::{ClosureLine, Read};
use crate::load::Loaded;
use crate::lock::KNOWN_VERSIONS;
use crate::lock::{current_version, Line};
use crate::record::{Binary, Content, FacetKind, Namespace, Targets};
use crate::refusal::Refusal;
use crate::replay::lock_at;
use crate::selection::{Found, Selection, Supplied};
use crate::status::Status;

/// A claim's strength (§7).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Strength {
    /// Admitted past the profile's step, naming every experimental record it rests on.
    Exploratory,
    /// Held to the production rule.
    Production,
}

/// Who supplied an input the catalog and the description did not.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Source {
    /// The caller, standing in for the application and the plan until they exist.
    Caller,
    /// The application's separately supplied inputs (`ROADMAP.md` §10.3).
    Application,
}

/// An input taken from outside the catalog and the description, named with its source.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Input {
    /// What it is.
    pub name: String,
    /// Who supplied it.
    pub source: Source,
}

/// Admission's verdict, strongest first (§7).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Verdict {
    /// The closure reaches outside the claim's profile or target.
    UnsupportedProfile,
    /// A production claim that the catalog, its history or its inputs cannot establish.
    NotEstablished,
    /// Admitted.
    Admitted,
}

/// Admission's answer: the strongest verdict, with every reason that reaches it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Admission {
    /// The verdict.
    pub verdict: Verdict,
    /// Every reason for it, none when admitted.
    pub reasons: Vec<String>,
}

/// What a claim result carries (§7).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ClaimResult {
    /// Its strength.
    pub strength: Strength,
    /// Its profile.
    pub profile: String,
    /// Its target, when it has one.
    pub target: Option<String>,
    /// The commit it read.
    pub commit: String,
    /// The `origin/main` commit it was checked against, when the clone has one.
    pub origin_main: Option<String>,
    /// Its closure, one line per facet.
    pub closure: Vec<ClosureLine>,
    /// Its reads, the lookups that found nothing among them.
    pub reads: Vec<Read>,
    /// Every input it took from outside the catalog and the description.
    pub inputs: Vec<Input>,
    /// Every precondition in the closure, as an assumption: an unproved link (`ROADMAP.md` §7.2).
    pub assumptions: Vec<String>,
    /// Every `experimental` record in the closure, with its status.
    pub experimental: Vec<(String, Status)>,
}

/// A claim being made: its selection and strength, what it has read, and what it took from outside.
pub struct Claim<'c> {
    loaded: &'c Loaded,
    history: &'c History,
    origin_main: Option<String>,
    profile: String,
    target: Option<String>,
    strength: Strength,
    reads: Vec<Read>,
    citations: BTreeSet<(String, FacetKind)>,
    costs: Vec<&'c crate::record::Cost>,
    inputs: Vec<Input>,
}

impl<'c> Claim<'c> {
    /// A claim over `loaded`, read from `history`, checked against the `origin/main` commit `origin_main` when the
    /// clone has one. A claim states no image: only an engine-made build gives one (`M4`).
    #[must_use]
    pub fn new(
        loaded: &'c Loaded,
        history: &'c History,
        origin_main: Option<&str>,
        profile: &str,
        target: Option<&str>,
        strength: Strength,
    ) -> Self {
        Self {
            loaded,
            history,
            origin_main: origin_main.map(str::to_owned),
            profile: profile.to_owned(),
            target: target.map(str::to_owned),
            strength,
            reads: Vec::new(),
            citations: BTreeSet::new(),
            costs: Vec::new(),
            inputs: Vec::new(),
        }
    }

    /// A lookup by `(facet, name)` under the claim's selection, recorded as a read, and as a citation when it finds.
    ///
    /// # Errors
    ///
    /// As [`crate::hash::Catalog::lookup`].
    pub fn lookup(&mut self, facet: FacetKind, name: &str) -> Result<Option<Found<'c>>, Refusal> {
        let selection = Selection {
            profile: &self.profile,
            target: self.target.as_deref(),
        };
        let found = self.loaded.catalog.lookup(selection, facet, name)?;
        self.reads.push(Read {
            facet,
            name: name.to_owned(),
            profile: self.profile.clone(),
            target: self.target.clone(),
            found: found.map(|f| f.id.to_owned()),
        });
        if let Some(f) = found {
            self.citations.insert((f.id.to_owned(), facet));
            if let Supplied::Cost(cost) = f.value {
                self.costs.push(cost);
            }
        }
        Ok(found)
    }

    /// An input taken from outside the catalog and the description.
    pub fn input(&mut self, name: &str, source: Source) {
        self.inputs.push(Input {
            name: name.to_owned(),
            source,
        });
    }

    /// The closure of the citations (§7): each cited facet and its record's contract, and the closure of every facet
    /// a derived line of it names.
    #[must_use]
    pub fn closure(&self) -> BTreeSet<(String, FacetKind)> {
        let mut out = BTreeSet::new();
        let mut stack: Vec<(String, FacetKind)> = self.citations.iter().cloned().collect();
        while let Some((id, facet)) = stack.pop() {
            if !out.insert((id.clone(), facet)) {
                continue;
            }
            stack.push((id.clone(), FacetKind::Contract));
            for line in self
                .loaded
                .hashes
                .facet(&id, facet)
                .map(|h| h.derived.as_slice())
                .unwrap_or_default()
            {
                let mut words = line.split(' ');
                if let (Some(kind), Some(other)) =
                    (words.next().and_then(FacetKind::parse), words.next())
                {
                    stack.push((other.to_owned(), kind));
                }
            }
        }
        out
    }

    /// The records the closure holds.
    fn records(&self) -> BTreeSet<String> {
        self.closure().into_iter().map(|(id, _)| id).collect()
    }

    /// The claim result (§7).
    #[must_use]
    pub fn result(&self) -> ClaimResult {
        let catalog = &self.loaded.catalog;
        let closure = self
            .closure()
            .into_iter()
            .filter_map(|(id, facet)| {
                let record = catalog.records.get(&id)?;
                Some(ClosureLine {
                    version: current_version(record, facet),
                    bound: self.loaded.hashes.facet(&id, facet)?.bound,
                    status: *self.loaded.statuses.facets.get(&(id.clone(), facet))?,
                    id,
                    facet,
                })
            })
            .collect();
        let records = self.records();
        let assumptions = records
            .iter()
            .filter_map(|id| catalog.records.get(id))
            .flat_map(|r| {
                r.contract
                    .preconditions
                    .iter()
                    .map(move |p| format!("`{}`: {p}", r.id))
            })
            .collect();
        let experimental = records
            .iter()
            .filter_map(|id| catalog.records.get(id))
            .filter(|r| r.namespace == Namespace::Experimental)
            .map(|r| {
                (
                    r.id.clone(),
                    self.loaded
                        .statuses
                        .records
                        .get(&r.id)
                        .copied()
                        .unwrap_or(Status::Unreviewed),
                )
            })
            .collect();
        ClaimResult {
            strength: self.strength,
            profile: self.profile.clone(),
            target: self.target.clone(),
            commit: self.loaded.commit.clone(),
            origin_main: self.origin_main.clone(),
            closure,
            reads: self.reads.clone(),
            inputs: self.inputs.clone(),
            assumptions,
            experimental,
        }
    }

    /// Admission (§7): the strongest verdict, with every reason that reaches it.
    ///
    /// # Errors
    ///
    /// `catalog-layout` for a commit the history lacks; a refusal met reading a lock.
    pub fn admit(&self) -> Result<Admission, Refusal> {
        let unsupported = self.unsupported()?;
        if !unsupported.is_empty() {
            return Ok(Admission {
                verdict: Verdict::UnsupportedProfile,
                reasons: unsupported,
            });
        }
        if self.strength == Strength::Exploratory {
            return Ok(Admission {
                verdict: Verdict::Admitted,
                reasons: Vec::new(),
            });
        }
        let reasons = self.not_established()?;
        Ok(Admission {
            verdict: if reasons.is_empty() {
                Verdict::Admitted
            } else {
                Verdict::NotEstablished
            },
            reasons,
        })
    }

    /// Admission's first step's reasons.
    fn unsupported(&self) -> Result<Vec<String>, Refusal> {
        let catalog = &self.loaded.catalog;
        let mut out = Vec::new();
        if let Some(target) = &self.target {
            if !catalog.named_targets()?.contains(target) {
                out.push(format!("`{target}` is not a named target"));
            }
        }
        for id in self.records() {
            let Some(record) = catalog.records.get(&id) else {
                continue;
            };
            // One profile exists in `/1`, which the reader requires of every record, so this cannot fire yet.
            if !record.contract.profiles.contains(&self.profile) {
                out.push(format!("`{id}` omits the profile `{}`", self.profile));
            }
            match (&record.contract.targets, &self.target) {
                (Targets::Any, _) => {}
                (Targets::Named(named), Some(target)) if named.contains(target) => {}
                (Targets::Named(_), Some(target)) => {
                    out.push(format!("`{id}` does not admit the target `{target}`"));
                }
                (Targets::Named(_), None) => {
                    out.push(format!(
                        "`{id}` names targets, and a claim with no target needs `(targets any)`"
                    ));
                }
            }
        }
        Ok(out)
    }

    /// A production claim's `not-established` reasons that the catalog and its history can judge.
    fn not_established(&self) -> Result<Vec<String>, Refusal> {
        let catalog = &self.loaded.catalog;
        let mut out = Vec::new();
        for (id, status) in self.result().experimental {
            out.push(format!("`{id}` is `experimental`, and {status:?}"));
        }
        for input in &self.inputs {
            out.push(format!("`{}` came from the {:?}", input.name, input.source));
        }
        for cost in &self.costs {
            let kind = catalog
                .tree
                .get(&format!("targets/{}.env", cost.target))
                .and_then(|b| core::str::from_utf8(b).ok())
                .and_then(|t| t.split('\n').find_map(|l| l.strip_prefix("TARGET_KIND=")))
                .map(str::to_owned);
            if kind.as_deref() != Some("board") {
                out.push(format!(
                    "the cost `{}` is on `{}`, which is not a board: only a board's timing is target evidence",
                    cost.name, cost.target
                ));
            }
            if let Ok(known) = &cost.value {
                if !matches!(known.binary, Binary::Independent(_)) {
                    out.push(format!(
                        "the cost `{}` is measured on an image, or none, and the claim has none (`M4`)",
                        cost.name
                    ));
                }
            }
        }
        for (id, facet) in self.closure() {
            if facet != FacetKind::Implementation {
                continue;
            }
            if let Some(record) = catalog.records.get(&id) {
                if matches!(record.implementation.content, Content::Present(_)) {
                    out.push(format!(
                        "`{id}`'s implementation holds code, and the claim has no image built from it (`M4`)"
                    ));
                }
            }
        }
        out.extend(self.ledger_reasons()?);
        // Premise 3's causes are `M2.7.3.6.2`'s. Its first holds of every claim until the director turns the main
        // line's protection on and a commit is named (the findings record's §11), so it is stated here now.
        out.push(
            "premise 3 has no named commit yet: the published main line's protection is not on"
                .to_owned(),
        );
        Ok(out)
    }

    /// The commit's ledger must hold every line of every ancestor's and of `origin/main`'s and its ancestors'.
    fn ledger_reasons(&self) -> Result<Vec<String>, Refusal> {
        let Some(main) = &self.origin_main else {
            return Ok(vec!["the clone has no `origin/main`".to_owned()]);
        };
        let held: BTreeSet<Line> = self
            .loaded
            .lock
            .as_ref()
            .map(|l| l.lines.iter().cloned().collect())
            .unwrap_or_default();
        let mut commits = self.history.ancestry(&self.loaded.commit)?;
        commits.extend(self.history.ancestry(main)?);
        let mut out = Vec::new();
        for name in commits {
            for line in lock_at(self.history, &name, &KNOWN_VERSIONS)?
                .map(|l| l.lines)
                .unwrap_or_default()
            {
                if !held.contains(&line) {
                    out.push(format!(
                        "the commit read lacks `{}`, which {name} holds",
                        line.render()
                    ));
                }
            }
        }
        out.sort();
        out.dedup();
        Ok(out)
    }
}
