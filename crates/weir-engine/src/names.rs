//! The names a statechart reads and runs: its evidence, its effects and
//! its templates, which its reader binds (98).

use std::collections::BTreeSet;

use crate::{Effect, Guard, Region, Statechart, Test};

/// The names a statechart reads and runs, each by name in name order
/// (98).
#[derive(Debug, Clone, PartialEq)]
pub struct Names {
    /// Every evidence its guards read: each `ev`, and the evidence each
    /// comparison with another evidence names.
    pub evidence: BTreeSet<String>,
    /// Every effect its states' `entry` and `exit` and its transitions'
    /// `effects` name.
    pub effects: BTreeSet<String>,
    /// Every template its states' `machine` names.
    pub machines: BTreeSet<String>,
}

/// The names `chart` reads and runs, through its states' own regions and
/// not into the templates they run, for its reader to check that it binds
/// each (98).
pub fn names(chart: &Statechart) -> Names {
    let mut names = Names {
        evidence: BTreeSet::new(),
        effects: BTreeSet::new(),
        machines: BTreeSet::new(),
    };
    for region in chart.regions.values() {
        names.region(region);
    }
    names
}

impl Names {
    fn region(&mut self, region: &Region) {
        for state in region.states.values() {
            self.effects(&state.entry);
            self.effects(&state.exit);
            if let Some(machine) = &state.machine {
                self.machines.insert(machine.clone());
            }
            for transition in &state.transitions {
                self.guard(&transition.when);
                self.effects(&transition.effects);
            }
            for region in state.regions.values() {
                self.region(region);
            }
        }
    }

    fn effects(&mut self, effects: &[Effect]) {
        self.effects
            .extend(effects.iter().map(|effect| effect.atom.clone()));
    }

    fn guard(&mut self, guard: &Guard) {
        match guard {
            Guard::All(guards) | Guard::Any(guards) => {
                for guard in guards {
                    self.guard(guard);
                }
            }
            Guard::Not(guard) => self.guard(guard),
            Guard::Evidence { ev, test } => {
                self.evidence.insert(ev.clone());
                match test {
                    Test::EqEv(other)
                    | Test::NeEv(other)
                    | Test::GtEv(other)
                    | Test::GteEv(other)
                    | Test::LtEv(other) => {
                        self.evidence.insert(other.clone());
                    }
                    _ => {}
                }
            }
            Guard::Response(_)
            | Guard::Final(_)
            | Guard::Region { .. }
            | Guard::Parent { .. }
            | Guard::Children { .. }
            | Guard::Always(_) => {}
        }
    }
}
