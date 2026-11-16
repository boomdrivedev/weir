# Design

## Context

`weir-kinds` reads a declaration's `[kinds.<kind>]` tables in one walk,
`read_kinds`, gathering every failing field into one `Refusal`. A table
whose key is no name is refused. A table holding `agents` and nothing
that declares a kind is settings for a kind declared elsewhere (57,
core.41). Every other table is a simple kind: its version, branch,
`starts_unready`, `lands`, `agents`, stages, landing fix and its bound,
and every other key returned among its `sections` (56). A table with
`stages` left out is refused naming its `stages`, so a statechart
kind's table, which holds `regions`, is refused today.

The statement holds the second format. 51 says a kind is a simple kind
or a statechart kind. 94 says what a statechart kind's declaration
holds, as amended at blueprints 8eec6dd (its landing fix) and 0252395
(its title and its label). 95 says how it is checked whole, and 81 that
`read-kinds` gives it with its statechart. C.4 spells it in TOML: a
kind's table holding `regions` and no `stages`, "its fields are the
engine's own names. So the engine's types define a statechart's fields
once, and `read-kinds` reads the table into them, the kind's name and
`version` as the statechart's own". Section 9 puts "the engine's crate
beside `weir-kinds`", moved from the original Flywheel's
flywheel-engine, and has a reader take `weir-kinds` and the engine at
one revision of weir. S19 and S20 are the scenarios. flume's record
0028, the user's ruling, keeps simple kinds as they are beside the
second format, neither translated into the other.

flywheel-engine's `defs.rs` (agentplot/flywheel-next) is the original's
statechart: `Region { initial, states }`, `State { final, decision,
entry, exit, machine, params, regions, enter, transitions }`,
`Transition { when, to, effects, bump, enter }`, the guard algebra,
`Effect { do, args }` and `Decision { kind, answers, shows }`, read
from YAML by serde with `serde_json::Value`s. C.4 names the same
fields, and the types below keep them.

Rules this design relies on, all in `docs/architecture/core.md`, the one
page whose area the change touches:

- core.1: `weir-engine` holds types and no state, and reads nothing.
- core.2: neither crate's code, nor a fixture of the crates' tests,
  names a tool of the family; the pack's cases may.
- core.30: one parser of kinds, `read-kinds`; the statechart's types in
  the engine's crate, which reads no text, are not ruled out.
- core.31: a simple kind reads and is refused as it was.
- core.32 and core.54: which templates and atoms there are, and what
  `machine`, `ev` and `do` name, are the reader's.
- core.39 and core.40: `weir-engine` is a moon project of its own, its
  four checks one task each, with no tool outside the tree's pins.
- core.41: a table holding `agents` and `regions` declares a kind and
  gives no settings.
- core.42: region and state names are held to the one check of a name.
- core.43: a package declares exactly one kind (see Decision 12).
- core.54: a statechart kind is read into the engine's types beside the
  simple kinds and checked whole.
- core.35: the pack's kinds cases cite the clauses they exercise, and
  A.8, with 94 and 95, is counted.
- kit:testing.1 and kit:testing.3: the new project's tasks are tagged
  with their layer and declare every input they read.

The design departs from one clause and no rule: a package of a
statechart kind is refused (Decision 12), where 89 and 92 would read it.

## Goals / Non-Goals

**Goals:**

- `read-kinds` reads a statechart kind into the engine's types and
  returns it beside the simple kinds, with its title, label, branch,
  landing, landing fix and its bound, and each state's stage-template
  parameters read.
- A statechart kind is checked whole: every field 95 names, and every
  field of the wrong shape, refused by its path in one refusal.
- A simple kind's every field, reading and refusal is as it was.
- The pack holds S19 and S20 as kinds cases, and counts 94 and 95.

**Non-Goals:**

- The tick, its tests and its toy statechart (96 to 99, A.13, the
  `tick` shape): the change that brings the engine's tick into
  `weir-engine` takes them in, with the cases of the `tick` shape.
- A package of a statechart kind: `Package` holds one simple kind; a
  package of either format is a change of its own (Decision 12).
- Which templates and atoms there are, whether a kind names only those
  its reader binds, and what a template's parameters mean beyond the
  stage template's five: the reader's (58, 95, 98).
- The seventh deliverable, a plan (54 as amended at blueprints
  9c7490c): the format's one spelling, `Deliverable`, holds six today,
  and a state's deliverables take the seventh with it when it is added.
- flume reading or running a statechart kind: its own change.

## Decisions

### 1. The engine's crate holds a statechart's types

A new library crate, `crates/weir-engine`, the moon project `engine`.
It depends on the workspace's `toml` alone, for the values a statechart
holds, and re-exports it. It holds the types of Decision 2, plain data
with public fields, each documented, and no function that reads text,
evaluates a guard or moves a state. `weir-kinds` depends on it and
re-exports it whole, `pub use weir_engine;`, as it re-exports `toml`, so
a reader names a statechart's types at the version `read-kinds` reads
into (section 9).

Not taken: the types in `weir-kinds`, which C.4 gives to the engine,
and which a later move to the engine's crate would break for every
reader. Not taken: flywheel-engine moved whole now, its tick and its
tests with it: the tick is A.13, its own change, which core.35 leaves
uncounted until then. Not taken: `weir-statechart` or another name. The
engine's crate is `weir-engine`, as each crate is `weir-<what it
holds>`, and the name is this change's own, cheap to change before it
lands.

### 2. The types

```rust
// crates/weir-engine
pub struct Statechart {
    pub name: String,      // the kind's name (C.4)
    pub version: u64,      // the kind's version (C.4)
    pub regions: BTreeMap<String, Region>,   // one or more, by name
}
pub struct Region {
    pub initial: String,
    pub states: BTreeMap<String, State>,     // one or more, by name
    pub sections: Table,
}
pub struct State {
    pub is_final: bool,                      // `final`; false when left out
    pub decision: Option<Decision>,
    pub entry: Vec<Effect>,
    pub exit: Vec<Effect>,
    pub machine: Option<String>,             // a template or statechart, by name
    pub params: Option<Table>,               // as written
    pub enter: BTreeMap<String, String>,     // region to state, as written
    pub regions: BTreeMap<String, Region>,   // its own; none when left out
    pub transitions: Vec<Transition>,        // in priority order
    pub sections: Table,
}
pub struct Transition {
    pub when: Guard,
    pub to: String,
    pub effects: Vec<Effect>,
    pub bump: Option<String>,
    pub enter: BTreeMap<String, String>,
    pub sections: Table,
}
pub enum Guard {
    All(Vec<Guard>),
    Any(Vec<Guard>),
    Not(Box<Guard>),
    Evidence { ev: String, test: Test },
    Response(String),
    Final(String),                           // `<state>` or `<region>.<state>`
    Region { name: String, within: Vec<String> },   // `in`
    Parent { within: Vec<String> },                 // `in`
    Children { kind: String, test: ChildrenTest },
    Always(bool),
}
pub enum Test {
    Is(Value), In(Vec<Value>), Exists(bool),
    Gt(Value), Gte(Value), Lt(Value), Lte(Value),
    Older(Duration),
    EqEv(String), NeEv(String), GtEv(String), GteEv(String), LtEv(String),
}
pub enum ChildrenTest { All(Vec<String>), Any(Vec<String>), None(Vec<String>), CountGte(u64) }
pub struct Effect { pub atom: String /* `do` */, pub args: Table, pub sections: Table }
pub struct Decision { pub kind: String, pub answers: Vec<String>, pub shows: Vec<String>, pub sections: Table }
```

Each derives `Debug`, `Clone` and `PartialEq`. `Table` and `Value` are
`toml`'s. A `sections` holds every key of its table the format does not
name, unread and as written, `doc` and `note` among them, which are
words for a person and change nothing (56, C.4). An effect's `args` is
empty when left out, and a decision's `answers` and `shows` are empty
when left out. Field names follow C.4 but where Rust takes the word:
`final` is `is_final`, `do` is `atom` and `in` is `within`.

Not taken: `serde_json::Value`, flywheel-engine's. A declaration's
values would be converted from TOML, a datetime to a string among them,
and `weir-kinds` returns every section as `toml`'s values already. Not
taken: a value type of the engine's own, which would hold the same as
`toml::Value` and convert to and from it.

### 3. A declaration returns statechart kinds beside the simple kinds

```rust
// crates/weir-kinds
pub struct Declaration {
    pub kinds: BTreeMap<String, Kind>,                  // as before
    pub statecharts: BTreeMap<String, StatechartKind>,  // by name
    pub settings: BTreeMap<String, KindSettings>,       // as before
    pub sections: Table,                                // as before
}
pub struct StatechartKind {
    pub statechart: weir_engine::Statechart,  // its name and version are the kind's
    pub title: String,
    pub label: Option<String>,
    pub branch: String,                       // the kind's name when left out
    pub lands: bool,                          // false when left out
    pub landing_fix: Option<Identity>,
    pub max_landing_fixes: Option<u64>,
    pub params: BTreeMap<String, Params>,     // by the state's path (Decision 9)
    pub sections: Table,
}
```

`StatechartKind` gives `name()` and `version()`, its statechart's. A
name is under one of `kinds` and `statecharts` at most, since TOML
holds one table per key.

Not taken: `Kind` made an enum of both formats. Every reader of simple
kinds would change its reading for a format it does not run, where
record 0028 keeps simple kinds as they are. Not taken: a statechart
kind returned among a declaration's `sections`, unchecked, which 95
rules out.

### 4. A kind's table says its format by what it holds

`read_kinds` walks each `[kinds.<kind>]` table as before, with one step
first. A table holding `regions` is a statechart kind. If it holds
`stages` too, it is refused naming `kinds.<kind>.regions`, "is declared
beside `stages`, and a kind's table holds stages for a simple kind or
regions for a statechart kind, never both", and nothing else of it is
read (95). A table holding `agents` and nothing that declares a kind
gives settings, and every other table is a simple kind, both as before.

A table holding `agents` and `regions` is a statechart kind, never
settings (core.41). A simple kind's `title` and `label` are no fields
of a simple kind and stay among its sections, as they were (52).

### 5. The kind's own fields

The fields a statechart kind's table names are `version`, `title`,
`label`, `branch`, `lands`, `landing_fix`, `max_landing_fixes` and
`regions`. Every other key, `agents`, `starts_unready` and
`elaboration` among them, is returned among its `sections` (56).

- `version`, `branch` and `lands` read as a simple kind's: a version of
  at least 1, 1 when left out; a non-empty branch, the kind's name when
  left out; a bool, false when left out (52, 57, 94).
- `title` is a non-empty string, and a kind with none is refused naming
  its `title`: 94 says the declaration holds it, and flume 175 that the
  kind carries it.
- `label` is a non-empty string, left out when left out. That it is one
  word, and what it means, are its reader's (flume 180).
- `landing_fix` and `max_landing_fixes` read in a simple kind's shapes,
  an identity with its version and a count of at least 0, each without
  the other. Either on a statechart kind whose `lands` reads false is
  refused naming it, "is on a kind that does not land, and only a kind
  that lands names its landing fix"; beside a `lands` the read refused,
  neither is judged against it (94, 57).

Not taken: a statechart kind with no title read with the kind's name as
its title, a default 94 does not give. Not taken: a landing fix on a
kind that does not land returned as written. 94 gives it to "a
statechart kind that lands", and a simple kind names one only where it
lands with no stages.

### 6. Regions and states

- `regions` is a table of one or more regions; a value that is no
  table, or an empty one, is refused naming the kind's `regions` (94).
  A state's own `regions` is a table of regions, none when left out or
  empty.
- A region's or a state's key is a name, held to `check_name`, so a
  `final` guard's `<region>.<state>` and a `region` guard's dotted path
  each read one way. A key that is no name is refused naming it, and
  nothing under it is read (core.42's check, C.4's dotted paths).
- A region is a table. Its `states` is a table of one or more states:
  left out, empty or of another shape, it is refused naming the
  region's `states` (95). Its `initial` is a non-empty string, refused
  naming it when missing or of another shape, and refused naming it
  when it names no state of the region (95).
- A state is a table. `final` is a bool, false when left out. `machine`
  is a non-empty string, `params` a table, `enter` a table of non-empty
  strings, `entry` and `exit` arrays of effects, and `transitions` an
  array of transitions, each in the order written. Every other key is
  among its `sections`.

Not taken: a state's `params` or `enter` refused on a state with no
`machine`, and a `final` state's transitions refused, neither of which
95 names.

### 7. Transitions and guards

A transition is a table. `when` is a guard and `to` a non-empty string,
each refused naming it when missing or of another shape. A `to` that
names no state of the transition's own region is refused naming it
(95, S20). `effects` is an array of effects, `bump` a non-empty string
and `enter` a table of non-empty strings.

A guard is a table of exactly one form C.4 names:

| form | keys |
|---|---|
| all, any | `all` or `any`, an array of guards |
| not | `not`, a guard |
| evidence | `ev`, a non-empty string, and exactly one of `is`, `gt`, `gte`, `lt`, `lte` (any value), `in` (an array), `exists` (a bool), `older` (a duration) or `eq_ev`, `ne_ev`, `gt_ev`, `gte_ev`, `lt_ev` (a non-empty string, another evidence) |
| response | `response`, a non-empty string |
| final | `final`, a non-empty string |
| region | `region`, a table of `name`, a non-empty string, and `in`, an array of strings |
| parent | `parent`, a table of `in`, an array of strings |
| children | `children`, a table of `kind`, a non-empty string, and exactly one of `all`, `any`, `none` (arrays of strings) or `count_gte` (an integer of at least 0) |
| always | `always`, a bool |

A value that is no table, a table whose keys are those of no form, an
evidence with no test or with two, and a `children` with none of its
tests or with two, are refused naming the guard's path, "is not a guard
of a form the format names" (95). A form whose own field is of the wrong
shape is refused naming that field. A guard's path is its transition's
`when` and, within it, `all[<n>]`, `any[<n>]` and `not`:
`kinds.persona.regions.stages.states.test.transitions[1].when.any[0].older`.

`older` is a duration in the format's one spelling, a whole number
followed by `s`, `m` or `h`, as a stage's `quiet` is (C.4), read by the
same function into a `Duration`. Not taken: flywheel-engine's own
spelling, which takes `d` and `w` too, a second spelling of a duration
in one format.

### 8. Effects and decisions

An effect is a table: `do` a non-empty string, refused naming it when
missing or of another shape; `args` a table, empty when left out.
A decision is a table: `kind` a non-empty string, refused naming it when
missing; `answers` and `shows` arrays of strings, empty when left out.
Every other key of each is among its `sections`.

### 9. A state's parameters, read as the stage template's

C.4 names the stage template's parameters, `agents`, `join`, `on_fail`,
`max_send_backs` and `deliverables`. `read-kinds` reads those five
wherever a state gives `params`, whatever its `machine` names, since
which name is the stage template is the reader's (95, 98). Every other
key of `params` is a template's own, unread; the whole table stays in
the engine's `State.params` as written.

```rust
// crates/weir-kinds
pub struct Params {
    pub agents: Option<ParamAgents>,
    pub join: Option<Join>,
    pub on_fail: Option<String>,
    pub max_send_backs: Option<u64>,
    pub deliverables: Option<Vec<Deliverable>>,
}
pub enum ParamAgents { Entries(Vec<Table>), Rule { rule: String, pattern: Option<String> } }
pub enum Join { All, Any, Count(u64) }   // spelled `all`, `any`, `count:<n>`
```

- `agents` is an array of tables, each entry's settings as written, or a
  table holding `rule`, a non-empty string, and `pattern`, a string,
  where given, and no other key (C.4).
- `join` is `all`, `any` or `count:<n>`, `n` a whole number of at least
  1 in decimal digits with no leading zero (95).
- `on_fail` is a non-empty string naming a state of the state's own
  region, and `max_send_backs` an integer of at least 0. C.4 gives both
  "as on a simple kind's last stage", whose `on_fail` naming no stage of
  its kind is refused, so a statechart's is refused naming it where it
  names no state of its region.
- `deliverables` is an array, each spelled as a simple stage's
  deliverable is, through `Deliverable` (54, 95). A state may name
  several, and none.

`StatechartKind.params` holds the five read for each state that gives
`params`, by the state's path: its region's name and its own, joined by
`.`, and so on down through the regions it lies in,
`stages.test` and `stages.build.checks.lint`. A reader walking the
engine's statechart finds a state's parameters by the path it walked.

Not taken: the five typed in the engine's `State`, which would put
`Deliverable` and the stage template, the reader's, in the engine (98).
Not taken: the five checked only where `machine = "stage"`, which would
have weir read what a template's name means.

### 10. Refusals and their order

Every problem is a `Problem` of the crate's one `Refusal`, named by its
path from the declaration's root, as a simple kind's are. A statechart
kind's walk is its `version`, `branch`, `title`, `label` and `lands`,
then its regions in name order, then `landing_fix` and
`max_landing_fixes`. In a region, its key, `initial`'s shape, then its
states in name order. In a state, its key, `final`, `decision`,
`entry`, `exit`, `machine`, `params`, `enter`, its own regions, then its
transitions in order. Once a region's states are read, its `initial`,
then each transition's `to`, then each state's `params.on_fail`, in
state order, are checked against them. Where its `states` is refused,
its `initial` is named for its own shape alone and judged against no
states, as a simple kind's `repeat` is judged against no refused end.
A declaration holding a
statechart kind that fails is refused whole, its simple kinds with it,
and returns no kind.

### 11. `weir check` lists a statechart kind

`weir check` lists the kinds of both formats in name order. A
statechart kind is `<name>@<version>`, then `  title <title>`, then the
landing fix lines as a simple kind's, then one line per region in name
order: `  region <region> from <initial>: <state>, <state> (final), …`,
its states in name order, each final state marked `(final)`. A state's
own regions are not listed. C.4's persona lists as:

```text
persona@1
  title Persona test
  region stages from build: build, dropped (final), passed (final), stopped, test
```

### 12. A package of a statechart kind is refused

`read_package` counts a statechart kind among a package's kinds, so a
`kind.toml` declaring a simple kind and a statechart kind declares two.
A package whose one kind is a statechart kind is refused naming
`kinds.<kind>`: "is a statechart kind, and read-package reads a package
of a simple kind alone". `Package.kind` is a `Kind`, and every reader of
packages names it.

This departs from 89 and 92, which give a package of either format its
kind. Carrying a statechart kind changes `Package` for every reader of
packages, a change of its own, which this one leaves to the plan. Not
taken: the refusal `declares no kind`, which is false.

### 13. The pack

A declaration's kinds case gains `statecharts`, beside `kinds`, in its
expectation:

```toml
[expect.statecharts.persona]
version = 1
title = "Persona test"
# label, landing_fix and max_landing_fixes where the kind names them
unread = []                   # the kind's sections

[expect.statecharts.persona.regions.stages]
initial = "build"
states.build = { machine = "stage", to = ["test", "stopped"] }
states.passed = { final = true }
states.stopped = { answers = ["retry", "drop"], to = ["build", "dropped"] }

[expect.statecharts.persona.params."stages.test"]
agents = "rule every-persona-definition"   # or "entries <n>"
join = "all"
on_fail = "build"
max_send_backs = 2
deliverables = ["committed:tests/personas.md"]
```

A state's expectation gives `final` (false when left out), `machine`,
`to`, each transition's target in order (none when left out), `answers`,
its decision's, and `regions`, its own, as a region's. The crate's
`tests/pack.rs` compares each statechart kind's version, title, label,
landing fix and its bound, sections, each region's initial and states,
each state's expectation, and the paths and parameters of `params`, and
names the case, the kind and the field of each difference. A kind read
that the case does not expect, in either map, is a difference.

The cases, under `pack/kinds/`:

| case | holds | satisfies |
|---|---|---|
| `a-statechart-kind-beside-a-simple-kind` | S19's bump and C.4's persona, read the same twice | 51, 81, 94 |
| `a-kind-holding-both-stages-and-regions` | S19's third kind, refused naming its `regions` | 95 |
| `a-transition-to-a-state-its-region-does-not-hold` | S20, refused naming the transition's `to` | 95 |
| `a-statechart-checked-whole` | a region with no states, an `initial` it does not hold, a guard of no form, `join = "most"`, `deliverables = ["commit"]` and `final = "yes"`, each refused naming its field in one refusal | 95 |
| `every-field-of-a-statechart-kind` | a title, label, branch, landing, landing fix and bound, a state's own regions, a decision, entry and exit effects, `machine`, `params` and `enter`, transitions with effects, a bump and an enter, every form of guard, and a `doc` returned unread | 94 |

`pack/clauses.toml` takes 94 and 95 into A.8, its comment saying 96 to
99, the engine, stay out until the change that brings the engine's tick
takes them in. The index stays read at blueprints 006fc34, where 94 and
95 stand in A.8: moving it would bring the clauses the statement has
added since, each with cases of its own, into this change.
`crates/weir-pack/tests/clauses.rs` reads A.8 with 94 and 95.

### 14. The README and the workspace's list

The README's crate table gains `weir-engine`, "A statechart's types,
which `weir-kinds` reads a statechart kind into and the engine ticks",
and its row for `weir-kinds` names simple and statechart kinds. The
comment over `Cargo.toml`'s `members` lists `crates/weir-engine`.

## Risks / Trade-offs

- [`Declaration` gains a field] → a caller that builds one by its fields
  gives `statecharts`. Readers read it; none in the family builds one.
- [A declaration that was refused now reads] → a `[kinds.<kind>]` table
  holding `regions` and no `stages` was refused naming its `stages`.
  flume refuses a launch of a statechart kind until it runs them (flume
  4), so reading one runs nothing.
- [`params` is held twice, as written in the engine's state and read in
  `StatechartKind.params`] → the five read are a reading of the table
  as written, by the one parser, and both come from one read.
- [A statechart kind's package is refused] → no reader packages one yet;
  the refusal names it as what it is.
- [`older` takes `s`, `m` and `h` alone] → a week is `168h`; a second
  spelling can be added to the format's one function later.

## Migration Plan

Nothing migrates: no record, layout or shared-branch write changes.
Every simple kind reads, returns and is refused as it was. flume and
sounding take `statecharts` when they move their pin of weir, each in a
change of its own.
