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
use crate::name::{fault, NAME};
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
const STATE_FIELDS: [&str; 5] = ["final", "machine", "params", "enter", "regions"];

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
    let regions = walk.regions(kind.get("regions"), &field(at, "regions"), None);
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

    /// A statechart's `regions` at `here`, by name in name order: the
    /// kind's, `within` none, one or more, and a state's own, `within` the
    /// state's path, none when left out or empty (94, 95). A region's key is
    /// a name, and one that is no name is refused naming it, nothing under
    /// it read (C.4; core.42).
    fn regions(
        &mut self,
        regions: Option<&Value>,
        here: &str,
        within: Option<&str>,
    ) -> BTreeMap<String, Region> {
        let mut read = BTreeMap::new();
        let table = match (regions, within) {
            (None, Some(_)) => return read,
            (Some(Value::Table(table)), Some(_)) => table,
            (Some(Value::Table(table)), None) if !table.is_empty() => table,
            (Some(Value::Table(_)), None) => {
                self.problems
                    .refuse(here, "is empty, and a statechart holds one or more regions");
                return read;
            }
            _ => {
                self.problems.refuse(
                    here,
                    "must be a table of regions, each with its initial state and its states",
                );
                return read;
            }
        };
        for (name, region) in in_name_order(table) {
            let at = field(here, name);
            if let Some(fault) = fault(name) {
                self.problems
                    .refuse(&at, format!("{fault}, and a region's name is {NAME}"));
                continue;
            }
            let Value::Table(region) = region else {
                self.problems.refuse(
                    &at,
                    "must be a table holding its initial state and its states",
                );
                continue;
            };
            let path = match within {
                None => name.clone(),
                Some(within) => format!("{within}.{name}"),
            };
            read.insert(name.clone(), self.region(region, &at, &path));
        }
        read
    }

    /// A region, `regions.<region>` at `at`, its path `path`: its initial
    /// state and its states, the states in name order and then, once they
    /// are read, every name of a state of the region checked against them
    /// (94, 95; C.4). Where its `states` is refused, its `initial` is named
    /// for its own shape alone, judged against no states.
    fn region(&mut self, region: &Table, at: &str, path: &str) -> Region {
        let initial = match region.get("initial") {
            None => {
                self.problems.refuse(
                    &field(at, "initial"),
                    "is missing, and a region names the state it starts in",
                );
                None
            }
            Some(Value::String(initial)) if !initial.is_empty() => Some(initial.clone()),
            Some(_) => {
                self.problems.refuse(
                    &field(at, "initial"),
                    "must be a non-empty string naming a state of the region",
                );
                None
            }
        };
        let here = field(at, "states");
        let mut states = BTreeMap::new();
        let mut targets = Targets::default();
        let names = match region.get("states") {
            None => {
                self.problems
                    .refuse(&here, "is missing, and a region holds one or more states");
                None
            }
            Some(Value::Table(table)) if table.is_empty() => {
                self.problems
                    .refuse(&here, "is empty, and a region holds one or more states");
                None
            }
            Some(Value::Table(table)) => {
                for (name, state) in in_name_order(table) {
                    let at = field(&here, name);
                    if let Some(fault) = fault(name) {
                        self.problems
                            .refuse(&at, format!("{fault}, and a state's name is {NAME}"));
                        continue;
                    }
                    let Value::Table(state) = state else {
                        self.problems.refuse(&at, "must be a table");
                        continue;
                    };
                    let state = self.state(state, &at, &format!("{path}.{name}"), &mut targets);
                    states.insert(name.clone(), state);
                }
                Some(table)
            }
            Some(_) => {
                self.problems
                    .refuse(&here, "must be a table of one or more states");
                None
            }
        };
        if let Some(names) = names {
            if let Some(initial) = &initial {
                self.held(names, &field(at, "initial"), initial);
            }
            for (at, target) in &targets.to {
                self.held(names, at, target);
            }
            for (at, target) in &targets.on_fail {
                self.held(names, at, target);
            }
        }
        Region {
            initial: initial.unwrap_or_default(),
            states,
            sections: unnamed(region, &REGION_FIELDS),
        }
    }

    /// A state's name a field at `at` gives, refused naming the field
    /// where `states`, its region's as written, holds no state of that
    /// name (95).
    fn held(&mut self, states: &Table, at: &str, name: &str) {
        if !states.contains_key(name) {
            self.problems
                .refuse(at, format!("`{name}` names no state of this region"));
        }
    }

    /// A state, `states.<state>` at `at`, its path `path`, every name of a
    /// state of its region it gives gathered in `targets` (94, C.4).
    fn state(&mut self, state: &Table, at: &str, path: &str, targets: &mut Targets) -> State {
        let _ = targets;
        let is_final = boolean(state, "final", at, self.problems);
        let machine = self.name(state, "machine", at);
        let params = match state.get("params") {
            None => None,
            Some(Value::Table(params)) => Some(params.clone()),
            Some(_) => {
                self.problems.refuse(
                    &field(at, "params"),
                    "must be a table of the parameters it gives what it runs",
                );
                None
            }
        };
        let enter = self.enter(state, at);
        let regions = self.regions(state.get("regions"), &field(at, "regions"), Some(path));
        State {
            is_final,
            decision: None,
            entry: Vec::new(),
            exit: Vec::new(),
            machine,
            params,
            enter,
            regions,
            transitions: Vec::new(),
            sections: unnamed(state, &STATE_FIELDS),
        }
    }

    /// A field holding a name as written, a non-empty string; `None` when
    /// left out (94).
    fn name(&mut self, table: &Table, name: &str, at: &str) -> Option<String> {
        match table.get(name)? {
            Value::String(text) if !text.is_empty() => Some(text.clone()),
            _ => {
                self.problems
                    .refuse(&field(at, name), "must be a non-empty string");
                None
            }
        }
    }

    /// An `enter`, a state's or a transition's: for each region of what the
    /// state runs, the state it enters, a non-empty string, as written; none
    /// when left out (94, 96).
    fn enter(&mut self, table: &Table, at: &str) -> BTreeMap<String, String> {
        let here = field(at, "enter");
        let mut read = BTreeMap::new();
        match table.get("enter") {
            None => {}
            Some(Value::Table(enter)) => {
                for (region, state) in in_name_order(enter) {
                    match state {
                        Value::String(state) if !state.is_empty() => {
                            read.insert(region.clone(), state.clone());
                        }
                        _ => self.problems.refuse(
                            &field(&here, region),
                            "must be a non-empty string naming the state it enters",
                        ),
                    }
                }
            }
            Some(_) => self.problems.refuse(
                &here,
                "must be a table naming, for each region, the state it enters",
            ),
        }
        read
    }
}

/// Every name of a state of its region a region's states give, each with
/// the path of the field that gives it, checked once the region's states
/// are read (95).
#[derive(Default)]
struct Targets {
    /// Each transition's `to`, in state order and then the order written.
    to: Vec<(String, String)>,
    /// Each state's `params.on_fail`, in state order.
    on_fail: Vec<(String, String)>,
}

/// A table's entries in name order, whatever order the table keeps, so
/// that a walk names its problems in one order (57).
fn in_name_order(table: &Table) -> Vec<(&String, &Value)> {
    let mut entries: Vec<_> = table.iter().collect();
    entries.sort_by(|a, b| a.0.cmp(b.0));
    entries
}
