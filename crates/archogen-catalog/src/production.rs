//! The `production` namespace (the record's §6, `M2.7.3.5.3`).
//!
//! A record in `production` additionally needs each of its four facets `production`, no `unknown` fact or cost,
//! every record it depends on, describes or is measured with in `production` too, and the facets its catalog
//! requires present and not empty. One that fails any of these is refused at load, and the catalog does not load: it
//! is not demoted, since demoting it quietly would let the next edit to a reviewed record pass unseen.

use crate::hash::Catalog;
use crate::record::{self, Content, FacetKind, FactValue, Namespace, Record};
use crate::refusal::{Code, Refusal};
use crate::status::{Status, Statuses};

/// The facets catalog `kind` requires present and not empty in `production` (§6's table).
fn required(kind: record::Catalog) -> &'static [FacetKind] {
    match kind {
        record::Catalog::Algorithms => &[
            FacetKind::Implementation,
            FacetKind::BehaviorModel,
            FacetKind::TimingModel,
        ],
        record::Catalog::Machine | record::Catalog::Interfaces => &[FacetKind::BehaviorModel],
        record::Catalog::Devices => &[FacetKind::BehaviorModel, FacetKind::Implementation],
    }
}

/// Whether facet `facet` of `record` is present and not empty: an implementation with a package, a behavioral model
/// with a fact, a timing model with a cost.
fn present(record: &Record, facet: FacetKind) -> bool {
    match facet {
        FacetKind::Contract => true,
        // The reader refuses an implementation's `sources` with no package, so present is not empty.
        FacetKind::Implementation => matches!(record.implementation.content, Content::Present(_)),
        FacetKind::BehaviorModel => {
            matches!(&record.behavior_model.content, Content::Present(m) if !m.facts.is_empty())
        }
        FacetKind::TimingModel => {
            matches!(&record.timing_model.content, Content::Present(m) if !m.costs.is_empty())
        }
    }
}

/// §6's rules for every record in `production`, with the statuses the catalog's history gives.
///
/// # Errors
///
/// `catalog-production` for the first rule a production record breaks: a facet not `production`, an `unknown`
/// fact or cost, a dependency, `describes` or `measured-with` record not in `production`, or a facet its catalog
/// requires missing or empty.
pub fn check_production(catalog: &Catalog, statuses: &Statuses) -> Result<(), Refusal> {
    for (id, record) in &catalog.records {
        if record.namespace != Namespace::Production {
            continue;
        }
        let refuse = |field: &str, message: String| {
            Refusal::new(
                Code::Production,
                &record.path,
                field,
                None,
                format!("`{id}` is in `production`, and {message}: review it again, or move it to `experimental`"),
            )
        };
        for facet in FacetKind::ALL {
            let status = statuses.facets.get(&(id.clone(), facet)).copied();
            if status != Some(Status::Production) {
                return Err(refuse(
                    facet.as_str(),
                    format!(
                        "its {} is {}",
                        facet.as_str(),
                        status.map_or("of no status".to_owned(), |s| format!("{s:?}")
                            .to_lowercase())
                    ),
                ));
            }
        }
        let (behavior, timing) = (&record.behavior_model.content, &record.timing_model.content);
        let facts = [behavior_facts(behavior), timing_facts(timing)].concat();
        if let Some(fact) = facts
            .iter()
            .find(|f| matches!(f.value, FactValue::Unknown(_)))
        {
            return Err(refuse(
                "fact",
                format!("its fact `{}` is `unknown`", fact.name),
            ));
        }
        if let Content::Present(model) = timing {
            if let Some(cost) = model.costs.iter().find(|c| c.value.is_err()) {
                return Err(refuse(
                    "cost",
                    format!("its cost `{}` is `unknown`", cost.name),
                ));
            }
        }
        let mut others: Vec<(&str, &str)> = record
            .contract
            .depends
            .iter()
            .map(|d| ("depends", d.id.as_str()))
            .collect();
        if let Content::Present(model) = behavior {
            others.extend(model.describes.iter().map(|d| ("describes", d.as_str())));
        }
        if let Content::Present(model) = timing {
            others.extend(
                model
                    .measured_with
                    .iter()
                    .map(|d| ("measured-with", d.as_str())),
            );
        }
        for (how, other) in others {
            if catalog.records.get(other).map(|r| r.namespace) != Some(Namespace::Production) {
                return Err(refuse(
                    how,
                    format!("`{other}`, which it names in `{how}`, is not"),
                ));
            }
        }
        for facet in required(record.contract.catalog) {
            if !present(record, *facet) {
                return Err(refuse(
                    facet.as_str(),
                    format!(
                        "its catalog requires its {} present and not empty",
                        facet.as_str()
                    ),
                ));
            }
        }
    }
    Ok(())
}

fn behavior_facts(content: &Content<record::BehaviorModel>) -> Vec<&record::Fact> {
    match content {
        Content::Present(m) => m.facts.iter().collect(),
        Content::None(_) => Vec::new(),
    }
}

fn timing_facts(content: &Content<record::TimingModel>) -> Vec<&record::Fact> {
    match content {
        Content::Present(m) => m.facts.iter().collect(),
        Content::None(_) => Vec::new(),
    }
}
