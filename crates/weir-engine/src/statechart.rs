//! A statechart: its regions side by side, each with its initial state and
//! its states, each state with its transitions out (94, C.4).

use std::collections::BTreeMap;

use toml::Table;

use crate::{Decision, Effect, Guard};

/// A statechart kind's statechart: the kind's name and version, and its
/// regions side by side (81, 94; C.4).
#[derive(Debug, Clone, PartialEq)]
pub struct Statechart {
    /// The kind's name, its key under `kinds`, the statechart's own (81,
    /// C.4).
    pub name: String,
    /// The kind's `version`, at least 1, the statechart's own (81, C.4).
    pub version: u64,
    /// Its `regions`, one or more, side by side, by name in name order (94,
    /// 95).
    pub regions: BTreeMap<String, Region>,
}

/// A region, `regions.<region>`: its initial state and its states (94,
/// C.4).
#[derive(Debug, Clone, PartialEq)]
pub struct Region {
    /// Its `initial`, the state it starts in, one of its own states (94,
    /// 95).
    pub initial: String,
    /// Its `states`, one or more, by name in name order (94, 95).
    pub states: BTreeMap<String, State>,
    /// Every key of the region the format does not name, unread and as
    /// written, `doc` and `note` among them (56, C.4).
    pub sections: Table,
}

/// A state, `states.<state>` (94, C.4).
#[derive(Debug, Clone, PartialEq)]
pub struct State {
    /// Its `final`, whether it is a final state; false when left out (94).
    pub is_final: bool,
    /// Its `decision`, asked while it is active; `None` when left out (94,
    /// 97).
    pub decision: Option<Decision>,
    /// Its `entry`, the effects performed on entering it, in the order
    /// written; none when left out (94, 96).
    pub entry: Vec<Effect>,
    /// Its `exit`, the effects performed on leaving it, in the order
    /// written; none when left out (94, 96).
    pub exit: Vec<Effect>,
    /// Its `machine`, the template or statechart it runs, by name as
    /// written; `None` when left out. Which names there are, and what each
    /// means, are the reader's (94, 95, 98).
    pub machine: Option<String>,
    /// Its `params`, the parameters it gives what it runs, as written;
    /// `None` when left out (94, C.4).
    pub params: Option<Table>,
    /// Its `enter`, the state its entering moves each region of what it
    /// runs to, by region, as written; none when left out (94, 96).
    pub enter: BTreeMap<String, String>,
    /// Its own `regions`, by name in name order; none when left out (94).
    pub regions: BTreeMap<String, Region>,
    /// Its `transitions` out, in priority order, as written; none when
    /// left out (94, 96).
    pub transitions: Vec<Transition>,
    /// Every key of the state the format does not name, unread and as
    /// written, `doc` and `note` among them (56, C.4).
    pub sections: Table,
}

/// A transition out of a state (94, 96; C.4).
#[derive(Debug, Clone, PartialEq)]
pub struct Transition {
    /// Its `when`, the guard that fires it (94).
    pub when: Guard,
    /// Its `to`, the state of its own region it goes to (94, 95).
    pub to: String,
    /// Its `effects`, performed when it fires, in the order written; none
    /// when left out (94, 96).
    pub effects: Vec<Effect>,
    /// Its `bump`, the counter it bumps when it fires, by name as written;
    /// `None` when left out (94, 96).
    pub bump: Option<String>,
    /// Its `enter`, the state its target's entering moves each region of
    /// what the target runs to, by region, as written; none when left out
    /// (94, 96).
    pub enter: BTreeMap<String, String>,
    /// Every key of the transition the format does not name, unread and as
    /// written, `doc` and `note` among them (56, C.4).
    pub sections: Table,
}
