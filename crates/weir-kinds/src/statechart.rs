//! A statechart kind: a `[kinds.<kind>]` table holding `regions` and no
//! `stages`, read into the engine's types beside the simple kinds (51, 81,
//! 94, 95; C.4; core.54).

use std::collections::BTreeMap;
use std::fmt;

use toml::{Table, Value};
use weir_engine::{Region, State, Statechart};

use crate::deliverable::Deliverable;
use crate::fields::{boolean, field, unnamed};
use crate::identity::Identity;
use crate::kinds::{read_branch, read_landing_fix, read_max_landing_fixes, read_version};
use crate::refusal::Problems;

/// The fields of a statechart kind the format names; every other key is a
/// section (56, 94).
const KIND_FIELDS: [&str; 8] = [
    "version",
    "title",
    "label",
    "branch",
    "lands",
    "landing_fix",
    "max_landing_fixes",
    "regions",
];

/// The fields of a region the format names (94, C.4).
const REGION_FIELDS: [&str; 2] = ["initial", "states"];

/// The fields of a state the format names (94, C.4).
const STATE_FIELDS: [&str; 1] = ["final"];

/// Why a statechart kind that does not land names no landing fix (94, 57;
/// record 0005).
const LANDING_FIX_NOT_LANDING: &str =
    "is on a kind that does not land, and only a kind that lands names its landing fix";

/// Why a statechart kind that does not land names no bound on its landing
/// fixes (94, 57; record 0005).
const MAX_LANDING_FIXES_NOT_LANDING: &str =
    "is on a kind that does not land, and only a kind that lands names how many landing fixes it runs";

/// Why a kind's table holding both `stages` and `regions` is refused (95).
pub(crate) const STAGES_AND_REGIONS: &str = "is declared beside `stages`, and a kind's table holds stages for a simple kind or regions for a statechart kind, never both";

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

/// A statechart kind's table, `kinds.<name>` at `at`, holding `regions`
/// and no `stages`, read into the engine's types and checked whole, every
/// failing field refused by its path (81, 94, 95; C.4; core.54).
pub(crate) fn read_statechart_kind(
    name: &str,
    kind: &Table,
    at: &str,
    problems: &mut Problems,
) -> StatechartKind {
    let mut walk = Walk {
        problems,
        params: BTreeMap::new(),
    };
    let version = read_version(kind, at, walk.problems);
    let branch = read_branch(name, kind, at, walk.problems);
    let title = walk.title(kind, at);
    let label = walk.label(kind, at);
    let before = walk.problems.len();
    let lands = boolean(kind, "lands", at, walk.problems);
    let lands_read = (walk.problems.len() == before).then_some(lands);
    let regions = walk.regions(kind.get("regions"), &field(at, "regions"));
    // Beside a `lands` the read refused, `lands` is named once, by its own
    // field, and neither is judged against it (57).
    let not_landing = lands_read == Some(false);
    let landing_fix = read_landing_fix(
        kind,
        not_landing.then_some(LANDING_FIX_NOT_LANDING),
        at,
        walk.problems,
    );
    let max_landing_fixes = read_max_landing_fixes(
        kind,
        not_landing.then_some(MAX_LANDING_FIXES_NOT_LANDING),
        at,
        walk.problems,
    );
    StatechartKind {
        statechart: Statechart {
            name: name.to_owned(),
            version,
            regions,
        },
        title,
        label,
        branch,
        lands,
        landing_fix,
        max_landing_fixes,
        params: walk.params,
        sections: unnamed(kind, &KIND_FIELDS),
    }
}

/// One statechart kind's walk: the problems the declaration's read
/// gathers, and the stage template's parameters of each state read so far,
/// by its path.
struct Walk<'a> {
    problems: &'a mut Problems,
    params: BTreeMap<String, Params>,
}

impl Walk<'_> {
    /// The kind's `title`: a non-empty string it must hold (94).
    fn title(&mut self, kind: &Table, at: &str) -> String {
        match kind.get("title") {
            Some(Value::String(title)) if !title.is_empty() => title.clone(),
            None => {
                self.problems.refuse(
                    &field(at, "title"),
                    "is missing, and a statechart kind holds its title, the words a person reads for it",
                );
                String::new()
            }
            Some(_) => {
                self.problems
                    .refuse(&field(at, "title"), "must be a non-empty string");
                String::new()
            }
        }
    }

    /// The kind's `label`: a non-empty string; `None` when left out (94).
    fn label(&mut self, kind: &Table, at: &str) -> Option<String> {
        match kind.get("label")? {
            Value::String(label) if !label.is_empty() => Some(label.clone()),
            _ => {
                self.problems
                    .refuse(&field(at, "label"), "must be a non-empty string");
                None
            }
        }
    }

    /// A statechart's `regions`, at `here`, by name in name order (94).
    fn regions(&mut self, regions: Option<&Value>, here: &str) -> BTreeMap<String, Region> {
        let mut read = BTreeMap::new();
        let Some(Value::Table(regions)) = regions else {
            return read;
        };
        for (name, region) in in_name_order(regions) {
            let at = field(here, name);
            if let Value::Table(region) = region {
                read.insert(name.clone(), self.region(region, &at));
            }
        }
        read
    }

    /// A region, `regions.<region>` at `at`: its initial state and its
    /// states (94, C.4).
    fn region(&mut self, region: &Table, at: &str) -> Region {
        let initial = match region.get("initial") {
            Some(Value::String(initial)) => initial.clone(),
            _ => String::new(),
        };
        let mut states = BTreeMap::new();
        if let Some(Value::Table(table)) = region.get("states") {
            for (name, state) in in_name_order(table) {
                if let Value::Table(state) = state {
                    let at = field(&field(at, "states"), name);
                    states.insert(name.clone(), self.state(state, &at));
                }
            }
        }
        Region {
            initial,
            states,
            sections: unnamed(region, &REGION_FIELDS),
        }
    }

    /// A state, `states.<state>` at `at` (94, C.4).
    fn state(&mut self, state: &Table, at: &str) -> State {
        State {
            is_final: boolean(state, "final", at, self.problems),
            decision: None,
            entry: Vec::new(),
            exit: Vec::new(),
            machine: None,
            params: None,
            enter: BTreeMap::new(),
            regions: BTreeMap::new(),
            transitions: Vec::new(),
            sections: unnamed(state, &STATE_FIELDS),
        }
    }
}

/// A table's entries in name order, whatever order the table keeps, so
/// that a walk names its problems in one order (57).
fn in_name_order(table: &Table) -> Vec<(&String, &Value)> {
    let mut entries: Vec<_> = table.iter().collect();
    entries.sort_by(|a, b| a.0.cmp(b.0));
    entries
}
