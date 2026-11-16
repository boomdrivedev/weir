//! The decisions standing in an object's state, each derived from the live
//! region whose state carries it (97).

use std::collections::BTreeMap;

use time::OffsetDateTime;

use crate::tick::Charts;
use crate::{Decision, ObjectState, Refusal, Statechart};

/// A decision standing: the live region whose state carries it, that
/// state, the decision as declared and when the region entered the state
/// (97).
#[derive(Debug, Clone, PartialEq)]
pub struct Standing {
    /// The region path whose state carries it.
    pub region: String,
    /// The state that carries it.
    pub state: String,
    /// The decision as declared.
    pub decision: Decision,
    /// When the region entered the state, where the object's state holds
    /// it.
    pub since: Option<OffsetDateTime>,
}

/// The decisions standing in `state`: for each live region path, in path
/// order, whose state carries a decision, that decision. A decision on a
/// region no longer live stands no more. It numbers, folds and groups
/// nothing, which are its caller's (97).
///
/// Refused, naming the path, as [`tick`](crate::tick) refuses a state it
/// cannot read.
pub fn decisions(
    chart: &Statechart,
    templates: &BTreeMap<String, Statechart>,
    state: &ObjectState,
) -> Result<Vec<Standing>, Refusal> {
    let charts = Charts { chart, templates };
    charts.check(state)?;
    let mut standing = Vec::new();
    for (path, held) in &state.regions {
        if !state.is_live(path) {
            continue;
        }
        let (_, def) = charts.held(path, held)?;
        if let Some(decision) = &def.decision {
            standing.push(Standing {
                region: path.clone(),
                state: held.clone(),
                decision: decision.clone(),
                since: state.entered_at.get(path).copied(),
            });
        }
    }
    Ok(standing)
}
