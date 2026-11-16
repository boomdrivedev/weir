//! A statechart kind: a `[kinds.<kind>]` table holding `regions` and no
//! `stages`, read into the engine's types beside the simple kinds (51, 81,
//! 94, 95; C.4; core.54).

use std::collections::BTreeMap;
use std::fmt;

use toml::Table;
use weir_engine::Statechart;

use crate::deliverable::Deliverable;
use crate::identity::Identity;

/// A statechart kind: its statechart, in the engine's types, its name and
/// version the kind's, and the fields of the kind the format names beside
/// it (81, 94; C.4).
#[derive(Debug, Clone, PartialEq)]
pub struct StatechartKind {
    /// Its statechart: its regions side by side, its name the kind's key
    /// under `kinds` and its version the kind's `version`, at least 1 and 1
    /// when left out (81, 94; C.4).
    pub statechart: Statechart,
    /// Its `title`, the words a person reads for it, a non-empty string
    /// (94).
    pub title: String,
    /// Its `label`, a non-empty string; `None` when left out. What it
    /// means is its reader's (94).
    pub label: Option<String>,
    /// The prefix of its places' branches; the kind's name when left out,
    /// as a simple kind's (52, 94).
    pub branch: String,
    /// Whether its work is submitted; false when left out (52, 94).
    pub lands: bool,
    /// Its landing fix, the kind a place runs when the merge queue sends
    /// its branch back, named by that kind's identity with its version and
    /// returned as written; `None` when left out. Only a statechart kind
    /// that lands names it, and which kind it resolves to is the reader's
    /// (52, 58, 94; core.32; record 0005).
    pub landing_fix: Option<Identity>,
    /// How many landing fixes it runs, at least 0; `None` when left out,
    /// its meaning then its reader's. Only a statechart kind that lands
    /// names it (52, 58, 94; record 0005).
    pub max_landing_fixes: Option<u64>,
    /// The stage template's parameters of each state that gives `params`,
    /// by the state's path: its regions' names and its own joined by `.`,
    /// as `stages.test` (94, C.4).
    pub params: BTreeMap<String, Params>,
    /// Every key of the kind the format does not name, unread and as
    /// written, `agents` and `starts_unready` among them (56).
    pub sections: Table,
}

impl StatechartKind {
    /// The kind's name, its statechart's (81, C.4).
    pub fn name(&self) -> &str {
        &self.statechart.name
    }

    /// The kind's version, its statechart's (81, C.4).
    pub fn version(&self) -> u64 {
        self.statechart.version
    }
}

/// The stage template's parameters a state gives in its `params`, read
/// wherever a state gives them, whatever its `machine` names; every other
/// key of `params` is a template's own, left as written in the engine's
/// state (94, 95, 98; C.4).
#[derive(Debug, Clone, Default, PartialEq)]
pub struct Params {
    /// Its `agents`: entries with their settings, or a rule its reader
    /// resolves; `None` when left out (94, C.4).
    pub agents: Option<ParamAgents>,
    /// Its `join`: all, any or a count; `None` when left out (94, 95).
    pub join: Option<Join>,
    /// Its `on_fail`, the state of its own region a send-back returns to;
    /// `None` when left out (52, 94, C.4).
    pub on_fail: Option<String>,
    /// Its `max_send_backs`, how many times before the work waits on its
    /// caller, at least 0; `None` when left out (52, 94, C.4).
    pub max_send_backs: Option<u64>,
    /// Its `deliverables`, each spelled as a simple stage's is, several or
    /// none; `None` when left out (54, 94, 95; C.4).
    pub deliverables: Option<Vec<Deliverable>>,
}

/// A state's `params.agents` (94, C.4).
#[derive(Debug, Clone, PartialEq)]
pub enum ParamAgents {
    /// A list of entries, each a table of its settings as written, as a
    /// simple stage's `agents` are (53, C.4).
    Entries(Vec<Table>),
    /// `{ rule, pattern }`: a rule its reader resolves against the place
    /// (94, C.4).
    Rule {
        /// Its `rule`, by name as written; what it means is the reader's.
        rule: String,
        /// Its `pattern`, as written; `None` when left out.
        pattern: Option<String>,
    },
}

/// A state's `params.join`: all, any or a count (94, 95; C.4). Its
/// [`Display`](fmt::Display) is its spelling: `all`, `any` or
/// `count:<n>`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Join {
    /// `all`.
    All,
    /// `any`.
    Any,
    /// `count:<n>`, `n` at least 1.
    Count(u64),
}

impl fmt::Display for Join {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::All => f.write_str("all"),
            Self::Any => f.write_str("any"),
            Self::Count(n) => write!(f, "count:{n}"),
        }
    }
}
