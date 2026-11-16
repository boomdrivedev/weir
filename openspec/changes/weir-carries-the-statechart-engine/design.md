# Design

## Context

flume's record 0029, the user's ruling of 2026-11-01, runs statechart
kinds on the original Boomdrive's engine, flywheel-engine, moved into
weir as a crate beside `weir-kinds` with its tests, its toy machine and
its test that it names no reader's domain. What is the original's own
stays with it: its loader of YAML files and its recutils reader. The
machine's types become the kind format's, so nothing defines a
statechart's fields twice. A reader takes the kind format and the engine
at one revision of weir.

The original is `agentplot/flywheel-next` at f2ce8e3, its
`crates/flywheel-engine` last changed at 8ce0c12. On this host it is the
bare repository
`~/.local/share/boomdrive/repositories/agentplot/flywheel-next/flywheel-next.git`,
read with `git show f2ce8e3:crates/flywheel-engine/src/<file>`. Its
parts: `defs.rs`, the machine as serde types; `eval.rs`, guards over a
`Snapshot`, with `match_answer`; `tick.rs`, `plan_tick` (pure) and
`apply` (to one object), a submachine instantiated on entry and set
aside under its attempt, `commands` for `enter`; `runtime.rs`, `Object`,
`Snapshot`, `EvidenceSource`, `Register`; `rail.rs`, decisions derived
and numbered, folds, batches and the tail; `load.rs` and `rec.rs`; and
`src/tests/`, `lamp.rs` (the toy lamp driven end to end), `guards.rs`
(the answer matcher and the duration parser) and `domain_boundary.rs`.
Its model is `design/models/statechart/model.md` there, sections 1.3
and 2.

The statement, read at blueprints 006fc34 where A.13 and 99 stand as
they do at its main:

- 96: a tick, given a statechart, its templates, one object's state and
  one snapshot, gives the transitions that fire, the effects to perform
  and the next state; every guard reads the one snapshot; in each
  region, innermost first, the first transition whose guard holds fires,
  at most one per region; a move is seen at the next tick; firing runs
  exit, transition and entry effects, each only while its proof is
  absent, bumps its counter, records when each region entered its
  state, and moves a statechart the target runs to the state its enter
  names.
- 97: a state that runs a template starts it afresh on each entry and
  sets the earlier aside under its attempt, readable; the statechart
  that runs it reads it by its final states and moves it by an enter; a
  response is applied once; a decision is derived from the state that
  carries it while it is active.
- 98: the engine names no reader's object or atom, and its tests run a
  toy statechart that shares no atom with any reader's; it holds no
  store and performs nothing; the same input always gives the same
  tick, and ticking changes nothing (69, I7).
- 99, B.4's `tick`. C.7's `tick` shape, the toy lamp among its cases.
  Section 9's crate. S21.

a-statechart-kind-is-read-in-weirs-kind-format, the other weir change of
the bolt `weir-carries-the-release-chains-parts`, makes
`crates/weir-engine`, the moon project `engine`, holding a statechart's
types and nothing else: `Statechart { name, version, regions }`,
`Region { initial, states, sections }`, `State { is_final, decision,
entry, exit, machine, params, enter, regions, transitions, sections }`,
`Transition { when, to, effects, bump, enter, sections }`, the `Guard`
algebra with `Test` and `ChildrenTest`, `Effect { atom, args, sections
}` and `Decision { kind, answers, shows, sections }`, values `toml`'s
and `older` a `std::time::Duration`. `weir-kinds` depends on it,
re-exports it and reads a statechart kind into it (core.54). Its design
leaves "the tick, its tests and its toy statechart (96 to 99, A.13, the
`tick` shape)" to this change.

Rules this design relies on, all in `docs/architecture/core.md`, the one
page whose area the change touches:

- core.1: the tick, `start`, `decisions` and `names` answer from their
  inputs alone; the engine holds no store and reads no clock.
- core.2: no source or test of the engine names a tool of the family;
  the pack's cases may, and these name none.
- core.4: the engine performs no effect, starts no agent and lands
  nothing.
- core.30 and core.54: one parser of kinds, `read-kinds`, reads a
  statechart into the engine's types; the tick cases read their
  statecharts through it.
- core.32: which templates and atoms there are, and what a template's
  parameters mean, are the reader's.
- core.35: the `tick` shape, its cases run against weir's crate, and
  A.13 counted. Amended by task 1.2.
- core.37: the clauses stand in the statement, at 006fc34.
- core.39 and core.40: `weir-engine` stays a moon project of its own,
  its four checks one task each.
- kit:testing.1 and kit:testing.3: `weir-pack`'s tasks declare every
  input they read, the engine's and the kind format's among them.

The design departs from no rule and no clause. It adds a rule, core.55,
drawn from 96 to 99 and record 0029 (decision 16).

## Goals / Non-Goals

**Goals:**

- The tick as 96 to 99 say, ported from flywheel-engine onto the types
  the kind format reads into, with `start`, `decisions` and `names`.
- The original's tests that hold what weir keeps, moved onto a toy lamp
  built from the engine's types.
- The pack's `tick` shape, its cases S21 and fourteen more, and A.13
  counted.

**Non-Goals:**

- The statechart's types and their reading: the kind format's change
  made them, and this one changes none.
- What the original keeps: its YAML loader, its recutils reader, the
  rail's register and numbers, folds and batches, decision groups, the
  tail, leases, the five engine machines, its objects' `record`, `$`
  references read from that record, and evidence the engine served
  itself (decision 2).
- flume running a statechart kind, binding its atoms and keeping an
  object's state in a place's held state: flume's change.
- A package of a statechart kind, which the kind format's change refuses.

## Decisions

### 1. The change builds on the kind format's change

The tick ticks the types a-statechart-kind-is-read-in-weirs-kind-format
puts in `weir-engine`, so it builds on that change. Both were cut from
the bolt's branch at one revision. Task 1.1 brings the bolt's branch,
holding that change landed, into this one before any code. Where the
landed types differ in a name or a field from the account in Context,
the tick follows the landed types and this change alters none of them.

Not taken: a second definition of a statechart in this change, which
record 0029 and core.30 rule out. Not taken: flywheel-engine's own
`defs.rs` moved beside the kind format's types and converted at the
tick, two definitions again.

### 2. What moves and what stays

| flywheel-engine | in weir |
|---|---|
| `plan_tick`, `apply`, `state_def`, `active_regions`, `set_aside_prior`, `init_nested`, `commands`, `commanded_effects` (`tick.rs`) | moved into one `tick` and its helpers (decision 6) |
| `initialise` (`lib.rs`) | `start`, for a fresh object (decision 11) |
| `eval`, `eval_ev`, `eval_response`, `match_answer`, `json_eq`, `cmp` (`eval.rs`) | moved onto `toml` values (decisions 7 to 9) |
| `Object`'s `config`, `entered_at`, `counters`, `applied_responses`, and its record's `prior` | `ObjectState` (decision 4) |
| `Snapshot`, `EvidenceSource` | `Snapshot`, a value its caller reads once (decision 5) |
| `rail::derive`'s reading of live decision states | `decisions`, with no number (decision 12) |
| `Machine::atom_names` | `names` (decision 13) |
| `tests/lamp.rs`, `tests/guards.rs`'s matcher tests, `tests/domain_boundary.rs` | the engine's tests over a toy lamp and bulb (decision 15) |
| `defs.rs` | the kind format's types, already in `weir-engine` |
| `load.rs`, `rec.rs`, `rail.rs`'s register, numbers, folds, batches, groups and tail, `Register`, `ResponseKind`, `TailEntry`, `Atoms`, `Object`'s `id`, `machine`, `parent`, `record`, `effects`, `seq` and `created`, `parse_duration` | stay with the original: the reader's, or the kind format's (`older` is read by `weir-kinds`) |

The engine serves no evidence of its own. The original's five,
`state`, `entered_at`, `seq`, `applied_responses` and `now`, are names
the engine bound itself, and 98 gives every evidence name to the reader.
A reader serves a region's entered time from the state it holds, under
a name of its own, at the region's path (decision 5). The original's
`$<field>` and dotted record paths in an effect's arguments read the
original's object record, which weir's object state does not hold; they
stay as written.

### 3. The calls and their types

```rust
// crates/weir-engine
pub fn start(chart: &Statechart, templates: &BTreeMap<String, Statechart>,
             at: OffsetDateTime) -> Result<ObjectState, Refusal>;
pub fn tick(chart: &Statechart, templates: &BTreeMap<String, Statechart>,
            proofs: &BTreeMap<String, String>, state: &ObjectState,
            snapshot: &Snapshot) -> Result<Tick, Refusal>;
pub fn decisions(chart: &Statechart, templates: &BTreeMap<String, Statechart>,
                 state: &ObjectState) -> Result<Vec<Standing>, Refusal>;
pub fn names(chart: &Statechart) -> Names;

pub struct ObjectState {
    pub regions: BTreeMap<String, String>,          // region path -> its state
    pub entered_at: BTreeMap<String, OffsetDateTime>,
    pub counters: BTreeMap<String, u64>,
    pub applied: Vec<String>,                       // response ids, in order
    pub set_aside: BTreeMap<String, BTreeMap<String, String>>,
}
pub struct Snapshot {
    pub at: OffsetDateTime,
    pub evidence: BTreeMap<String, Value>,                   // the object's
    pub regions: BTreeMap<String, BTreeMap<String, Value>>,  // by region path
    pub responses: Vec<Response>,
    pub owner: Option<Related>,
    pub children: Vec<Related>,
}
pub struct Response { pub id: String, pub answer: String }
pub struct Related { pub kind: String, pub states: Vec<String> }

pub struct Tick { pub fired: Vec<Fired>, pub next: ObjectState }
pub struct Fired {
    pub region: String, pub from: String, pub to: String,
    pub bump: Option<String>,
    pub response: Option<Applied>,
    pub effects: Vec<Planned>,
}
pub struct Applied { pub id: String, pub argument: String }
pub struct Planned { pub atom: String, pub args: Table, pub region: String, pub phase: Phase }
pub enum Phase { Exit, Transition, Entry, Enter }

pub struct Standing { pub region: String, pub state: String,
                      pub decision: Decision, pub since: Option<OffsetDateTime> }
pub struct Names { pub evidence: BTreeSet<String>, pub effects: BTreeSet<String>,
                   pub machines: BTreeSet<String> }
pub struct Refusal { pub path: String, pub reason: String }
```

Each derives `Debug`, `Clone` and `PartialEq`, `ObjectState` `Default`
too, and every public item is documented.
`OffsetDateTime` is the workspace's `time`, which `weir-engine` gains;
`Value` and `Table` are `toml`'s, as the kind format's types hold.
`templates` holds each template or statechart a state may run, by the
name its `machine` gives (C.4). `proofs` maps an effect's atom to the
evidence that proves it, which the reader names (96). `Refusal`
displays as `` `<path>`: <reason> ``.

The modules: `state.rs` (`ObjectState`, region paths), `snapshot.rs`,
`eval.rs` (guards, values, the answer matcher), `tick.rs` (`tick`,
`start` and what they share), `decisions.rs`, `names.rs` and
`refusal.rs`, each re-exported from `lib.rs` beside the types.

Not taken: the original's `EvidenceSource`, a trait the engine calls
during a tick. A snapshot that is a value is read once, before the
tick, as 96 and flume 176 say, and a tick over it is pure by its
signature. Not taken: serde on the engine's state. How a reader keeps an
object's state is its own (flume's held state), and a spelling of it
here would be a record weir does not define.

### 4. The object's state and region paths

A region path is `<region>` for a top region and `<path>.<state>.<region>`
for a region under a state: one of the state's own regions, or a region
of the template it runs. `regions` holds each path's state, `entered_at`
when it entered it, `counters` each counter a transition bumped, and
`applied` each response a guard took. `set_aside` holds each instance a
state's re-entry set aside, under `<path>.<state>#<attempt>`, the
attempt counting from 1, each instance's region paths taken relative to
that state with their states.

A region is live when every state its path names is the state its
region holds now. A region under a state its region has left is not
live. It stays at its path, holding the state it reached, and ticks as
any other until an enter moves it or its state is entered again (model
section 1.3).

### 5. The snapshot

The snapshot is the caller's reading of the object's evidence at one
moment (98). `at` is that moment: a region entering a state records it,
and `older` measures an age to it. `evidence` holds values for the
object as a whole. `regions` holds values for one region path, so a
template run in two places, or an instance left beside a fresh one, is
read apart. An evidence is read for a region path from the values given
at that path, else at the nearest path above it, else the object's.
`responses` are those addressed to the object, in the order given;
which decision a response answers, and the numbers that name them, are
the caller's. `owner` and `children` are the kind and the states the
caller reads for the object's owner and for each child, for the
`parent` and `children` guards. Nothing else crosses objects.

### 6. The tick

The tick reads `state` and `snapshot` and writes neither. It refuses
first what decision 14 names. Then, over the region paths `state` holds,
deepest first and in path order at one depth, skipping a path under a
region that fired this tick:

1. The state the path holds is resolved through the statechart, its
   states' own regions and the templates they run.
2. Its transitions are tried in the order written. The first whose
   guard holds over the snapshot and `state` as given (decision 7) is
   the region's one transition this tick.
3. Its effects are planned in order: the left state's `exit`, where the
   target is another state, then the transition's `effects`, then the
   target's `entry`, where it is another state. Then, for each region
   the target state's `enter` and the transition's `enter` move, the
   transition's winning where both name one, the moved-to state's
   `entry`, phase `Enter`, at that region's path. An effect is left out
   while the snapshot holds its proof (decision 8) at the region where it
   is planned. An effect whose atom has no proof given is planned
   whenever it is due.
4. A transition to the state it leaves that plans no effect, takes no
   response, bumps nothing and names no `enter` fires nothing, and the
   region tries no later transition this tick: reading twice with
   nothing changed writes nothing.

An `enter` key names a region by its name, by the state that holds it,
or by the template that state runs. It matches each region path the
state holds, live or not, whose region holds the target state, the
outermost match alone. The original's match of a key with an `s`
added, an English plural, is not taken.

`next` is `state` with each fired transition applied in the order they
fired:

1. Each region its `enter` moves, where it is not already in the
   target: everything under that path is cleared, the path holds the
   target, entered at `at`, and the target's regions start (decision
   11). A region `next` no longer holds is passed over.
2. Where the target state holds an instance under
   `<region>.<to>.`, that instance is set aside under its next attempt
   and cleared.
3. The region holds `to`; where `to` is another state, it entered it at
   `at`.
4. The counter `bump` names gains one.
5. The response the guard took is added to `applied` once.
6. The target's own regions and the regions of the template it runs
   start, entered at `at`.

A transition to the state it leaves runs its own effects alone, keeps
the time its region entered the state, and still starts what the state
runs afresh, the earlier instance set aside (97; model section 1.3).

### 7. Guards

Each form of C.4, over the region path `P` of the transition and the
state `S` it leaves:

| form | holds when |
|---|---|
| `all` | every guard holds; of none, it holds |
| `any` | some guard holds; of none, it does not |
| `not` | its guard does not |
| `always` | its value is `true` |
| `ev` with `is` | the evidence is read and equals the value |
| `ev` with `in` | the evidence is read and equals one of the values |
| `ev` with `exists` | the evidence is read, for `true`; is not, for `false` |
| `ev` with `gt`, `gte`, `lt`, `lte` | the evidence is read and is so ordered against the value |
| `ev` with `older` | the evidence is read as a time and `at` is more than the duration after it |
| `ev` with `eq_ev`, `gt_ev`, `gte_ev`, `lt_ev` | both evidences are read and so compare |
| `ev` with `ne_ev` | the evidence is read, and the other is not or is unequal to it |
| `response` | a response of the snapshot not in `applied` answers it (decision 9) |
| `final` | decision 10 |
| `region` | a dotted name: some region path equal to it or ending `.<name>` holds a state listed; a bare name: a region of that name whose path begins `P.S.`, or whose path is `P`'s with another last name, holds a state listed |
| `parent` | the snapshot's owner holds a state listed |
| `children` with `all` | the snapshot holds a child of the kind, and every child of the kind holds a state listed |
| `children` with `any`, `none` | some child of the kind holds a state listed; none does |
| `children` with `count_gte` | the snapshot holds at least that many children of the kind |

`all` takes the response of the last guard in it that took one, and
`any` the response of the guard that held. `not` takes none.

### 8. Values compared, and a proof present

Two values are equal by `toml`'s equality, but that an integer and a
float are equal where their numbers are, and two times with offsets
where their instants are. Numbers order by value, strings by their
bytes, and times with offsets by instant. Any other pair has no order,
and a guard that orders it does not hold. A time is a TOML datetime
with an offset, or a string in RFC 3339; a datetime with no offset is
no time.

A proof is present when the snapshot holds its evidence, read at the
region, with any value but `false`.

Not taken: the original's loose equality, under which `"true"` equals
`true` and `"3"` equals `3`, and its truthiness, under which `"none"`,
`"absent"` and `""` read false. Those read a store's strings; weir's
reader gives its evidence typed.

### 9. Responses

A `response` guard holds for the first response of the snapshot, in its
order, that `applied` does not hold and whose answer matches the guard's
pattern, the original's `match_answer`, moved whole: a bare word matches
itself, trimmed; `word <arg>` matches the word and binds what follows;
`word: <text>` binds what follows the colon; `<arg>` alone binds the
whole answer; `<arg> tail` binds what comes before the tail. The fired
transition carries the response's id and the argument bound, empty for a
bare word. Where an effect's argument is the string `$response`, it is
planned as the argument bound, empty where the transition took no
response. A response the object applied never holds again (97). Every
guard of one tick reads the same `applied`, so two regions may take one
response in one tick, and it is added once.

### 10. What a state runs

A state that runs a template, or holds regions of its own, starts them
when it is entered: each region at its initial state, entered at `at`,
and what that state runs started in turn. A state that runs a statechart
it already lies inside is refused, so a start always ends.

`{ final = "X" }` holds when a region the guard can see is in a state
named `X` that is final in its region. It sees the regions directly
under a state of `P`'s region, and the regions of a statechart run by a
state of one of `S`'s own regions. Where `S` runs a template or holds
regions of its own, it sees only those under `S`; where it runs
nothing, it sees what an earlier state of its region ran, still standing
at its path. `{ final = "R.X" }` holds where that region is named `R`,
or lies under `S`'s own region `R`.

Not taken: the original's reading of a `final` guard's state as any
state of that name, final or not, since 97 has the statechart that runs
it read it by its final states. Not taken: its reading of `R.X` as any
region of the object under a region named `R`. A sibling region is the
`region` guard's (94).

### 11. An object's first state

`start` gives each top region at its initial state, entered at `at`,
with what each initial state runs started (decision 10), and no
counter, response or instance set aside. It plans no effect: an initial
state's `entry` runs when a transition enters it, as in the original.

### 12. Decisions

`decisions` gives, for each live region path in path order whose state
carries a decision, the path, the state, the decision as declared and
the time `entered_at` holds for the path. A decision on a region no
longer live stands no more (97). It numbers nothing, folds nothing and
groups nothing.

### 13. Names

`names` gives every evidence a statechart's guards read, `ev` and the
evidence each of `eq_ev`, `ne_ev`, `gt_ev`, `gte_ev` and `lt_ev` names;
every effect its states' `entry` and `exit` and its transitions'
`effects` name; and every template its states' `machine` names, through
its states' own regions and not into the templates. A reader checks
with it that it binds each atom a kind names (flume 178).

### 14. Refusals

`start`, `tick` and `decisions` refuse, naming the path, the first of:

| path | reason |
|---|---|
| a region path | names no region of the statechart |
| a region path | holds a state its region does not hold |
| a state's path | runs `<name>`, which is no template given |
| a state's path | runs `<name>`, a statechart it already lies inside |
| a state's path | holds a region `<region>` as its own and in the template it runs |

A state's path is `<region path>.<state>`.

### 15. The engine's tests and the toy

The tests build the toy from the engine's types in
`tests/common/mod.rs`, with no reading of TOML. The lamp has one region,
`power`:

- `off` carries a decision `switch` answered `on` or `mode <name>`. A
  response `mode <name>` keeps it `off` and plans `set_mode` with `mode
  = "$response"`. A response `on` moves it `on`, as `lamp.switch` read
  `up` does.
- `on` plans `light` on entry, proved by `lamp.lit`. It runs the
  template `bulb` and holds a region `level`, `dim` moving `bright` when
  `lamp.level` is at least 5. It moves `off` when the bulb is `burnt`,
  bumping `burnouts`; when `lamp.faulty` is `true` or a response says
  `off`; and when `lamp.switch` reads `down`, moving the bulb to `cold`.

The bulb has one region, `glow`: `warming` moves `lit` when `bulb.warm`
is `true`, and `warming` or `lit` moves `burnt` when `bulb.burnt` is
`true`. `burnt` and `cold` are final, and `cold` plans `cool` on entry,
proved by `bulb.cool`. S21's lamp is the lamp with `power`'s two states
alone, its one transition `lamp.switch` read `up`.

The original's lamp tests move as test files of the engine:
`initial_configuration_and_nested_region` as `start`'s;
`response_fires_once_and_is_recorded`,
`proof_present_suppresses_the_effect` and
`evidence_and_dictation_leave_on` as the tick's, a response addressed
by the snapshot in place of a numbered answer or a dictation; and its
guard tests over the snapshot. Its rail tests, numbering, folds,
groupings and the tail, stay with the original. `guards.rs`'s matcher
tests move as `eval.rs`'s unit tests; its duration tests stay, the
format's `older` being `weir-kinds`'.

`domain_boundary.rs` becomes `tests/boundary.rs`. No file under `src/`
names, as a whole word in any case, an object the family's readers run:
`place`, `stage`, `session`, `agent`, `intent`, `elaboration`, `bolt`,
`unit`, `claim` and `verdict`. None names `lamp`, `bulb` or an atom
`names` gives for the toy. A test of the test holds that a name put back
is found, as a whole word only. Where a doc the kind format's change
wrote names one, the doc is reworded.

### 16. The engine's rule

Task 1.2 adds to `docs/architecture/core.md`, after core.54, under
"Kinds", the rule and its details, numbered core.55 where that is the
next free number once the bolt's branch is carried, else the next free
one, every mention here following it:

> **[core.55]** The engine's tick is one function, `weir-engine`'s,
> over a statechart, the templates it runs, the proofs its reader
> names, one object's state and one snapshot: it reads nothing else,
> returns the transitions that fire with the effects each plans and the
> object's next state, and changes none of its inputs; every guard of
> one tick reads that snapshot and the state as given, each region fires
> at most one transition, innermost first, and an effect is planned
> only while its proof is absent; the engine serves no evidence of its
> own, interprets no evidence's or effect's name, performs nothing,
> numbers no decision and addresses no response, which are its caller's.

> **core.55** Rules out: a clock, a store, a file or an environment
> read in `weir-engine`; a guard that reads a move made in its own tick;
> two transitions of one region in one tick; an effect planned while the
> snapshot holds its proof; an evidence the engine serves itself,
> `state`, `entered_at`, `seq` or `now` among them; a reader's object, a
> reader's atom or a tool's name in the engine's code (core.2); an
> effect performed, a decision numbered, a response addressed by number
> or a lease held by the engine; a second tick or a second evaluator of
> guards in the family; a template's parameters read by the engine
> (core.32); a statechart's fields defined in the engine apart from the
> types `read-kinds` reads into (core.30, core.54). Not ruled out: an
> effect's argument `$response`, given the argument its response bound;
> an instance a state set aside kept in the object's state; a reader
> serving a region's entered time from the state it holds, by an
> evidence name of its own; an instance left at its path ticking until
> an enter moves it (97). Source: clauses 69, 96 to 99, I7; C.7;
> section 9; S21; flume's record 0029.

The same task amends core.35. Its rule reads "one of C.7's eight", "a
case of the branch, inbox, kinds, tick, cadence or delivery shape" and
"a clause of A.1 to A.10, A.13, A.14 or 71". Its details rule out "a
clause of A.1 to A.10, A.13, A.14 or 71 no case names" and "a tick case
run against any engine but weir's". They drop the sentence that leaves
A.13 and the `tick` shape uncounted. Their source adds 96 to 99, S21 and
flume's record 0029.

### 17. The pack's tick cases

`Shape::Tick`, spelled `tick`, stands between `Kinds` and `Delivery`, as
C.7 lists the shapes, so `Shape::ALL` holds eight. The cases are read
and run in `weir-pack`, as the delivery cases are, so the case's
statechart is read by `read-kinds`, the one parser (core.30), and no
crate depends on another that depends on it. `weir-pack` depends on
`weir-kinds` and `weir-engine`.

A case's `case.toml`:

```toml
# What the case holds.
satisfies = [96, 98, 99]
declaration = """
[kinds.lamp]
version = 1
title = "Lamp"
# ...its regions, and every template it runs as a statechart kind beside it
"""
chart = "lamp"                     # the statechart kind ticked; the others are its templates, by name
proofs = { light = "lamp.lit" }

[state]                            # the object's state; each field but `regions` empty when left out
regions = { power = "off" }
entered_at = { power = 2026-09-04T12:00:00Z }
counters = {}
applied = []
set_aside = {}

[snapshot]                         # each field but `at` empty or none when left out
at = 2026-09-04T12:01:00Z
evidence = { "lamp.switch" = "up" }
regions = { "power.on.level" = { "lamp.level" = 7 } }
responses = [{ id = "r1", answer = "on" }]
owner = { kind = "lamp", states = ["on"] }
children = [{ kind = "bulb", states = ["lit"] }]

[expect]
fired = [
  { region = "power", from = "off", to = "on", effects = [
    { do = "light", region = "power", phase = "entry" },
  ] },
]                                  # `bump`, `response = { id, argument }` and an effect's `args` where expected
next = { regions = { power = "on" }, entered_at = { power = 2026-09-04T12:01:00Z } }
```

`read_tick_case` reads it, every field the form does not name refused
naming it, `chart` refused unless it names a statechart kind of the
declaration, and a declaration `read-kinds` refuses refused with its
refusal. `run_tick_case` ticks the case twice, from clones of its
inputs, and names the case and the field of each difference from what
it expects: a transition, an effect, a part of the next state, the two
ticks unequal, or an input changed.

The cases, each holding the full lamp and bulb unless the table says
otherwise:

| case | holds | satisfies |
|---|---|---|
| `the-lamp-switched-on` | S21's first tick: S21's lamp `off`, `lamp.switch` `up`, `lamp.lit` absent; `off` to `on` with `light` at entry | 96, 98, 99 |
| `the-lamp-on-with-its-proof-present` | S21's second: S21's lamp `on`, `lamp.lit` `true`; nothing fires, the state as given | 96, 98, 99 |
| `an-effect-whose-proof-is-present` | S21's lamp `off`, `lamp.switch` `up`, `lamp.lit` `true`; `off` to `on` with no effect | 96 |
| `the-first-transition-whose-guard-holds` | `off`, a response `on` and `lamp.switch` `up`; the response's transition alone fires | 96 |
| `innermost-first` | `on`, `level` `dim`, `lamp.level` 7, `lamp.faulty` `true`; `level` to `bright`, then `power` to `off` | 96 |
| `a-sibling-sees-a-move-at-the-next-tick` | a lamp of two regions, `shade` guarding on `power` by a `region` guard; `power` moves, `shade` does not | 96 |
| `a-transition-to-its-own-state-with-nothing-to-do` | a lamp whose `off` moves to itself, with nothing, before moving `on` under the same guard; nothing fires | 96 |
| `a-template-started-afresh` | `off`, the bulb left at `power.on.glow` `lit`, `lamp.switch` `up`; `on` entered, the bulb set aside under `power.on#1`, `glow` at `warming` | 97 |
| `a-final-state-of-what-a-state-runs` | `on`, `glow` `burnt`; `on` to `off`, `burnouts` bumped | 96, 97 |
| `an-enter-moves-what-a-state-ran` | `on`, `lamp.switch` `down`; `on` to `off`, `glow` moved to `cold` with `cool` planned at `power.on.glow` | 96, 97 |
| `a-response-applied-once` | `off`, `r1` answering `on`; it fires taking `r1`, and `next` applies it | 97 |
| `a-response-already-applied` | `off`, `r1` answering `on`, `r1` applied; nothing fires | 97 |
| `a-response-binds-its-argument` | `off`, `r1` answering `mode warm`; `off` to `off` planning `set_mode` with `mode = "warm"` | 96, 97 |
| `a-guard-by-age` | a lamp moving `on` when `lamp.since` is `older = "10m"`, eleven minutes before `at`; it fires | 96 |
| `evidence-read-at-its-region` | `on`, `lamp.level` 7 at `power.on.level` and 1 for the object; `level` to `bright` | 96 |

A test of `weir-pack` holds that these are the cases under
`pack/tick/`, each citing what the table cites, and that each case reads
and runs with no difference.

### 18. The index and the check

`pack/clauses.toml` gains A.13, after A.12, holding 96, 97 and 98, and
B.4 gains 99 after 93. Its header says the index holds every clause of
blueprints 006fc34, and the sentence leaving 96 to 99 out is dropped.
The commit stays 006fc34, where A.13 and 99 read as they do at the
statement's main. `COUNTED_SECTIONS` gains A.13, and the docs of
`clauses.rs` and `check.rs` say the check counts A.1 to A.10, A.13,
A.14 and 71. The docs of `pack.rs`, `check.rs` and `lib.rs` say eight
shapes.

### 19. The README and the workspace's list

The README's row for `weir-engine` reads: "A statechart's types, which
`weir-kinds` reads a statechart kind into, and the engine's tick: one
pass over one object's state and one snapshot, giving the transitions
that fire, their effects and the next state." The comment over
`Cargo.toml`'s `members` says the same of `crates/weir-engine`.

## Risks / Trade-offs

- [The kind format's change lands late or changes its types] → task 1.1
  stops short until the bolt's branch holds it, and decision 1 has the
  tick follow the types as they land.
- [Rules taken from the original change for a reader] → `final` reads
  final states alone, values compare strictly, and no evidence is the
  engine's own. A template written for the original that leans on the
  old reading is changed where flume moves it, in flume's change; each
  departure is named in decisions 2, 8 and 10.
- [An instance left at its path keeps ticking] → as in the original,
  until an enter moves it or its state is entered again; a reader that
  wants it quiet commands it to a final state (97).
- [Two regions take one response in one tick] → as in the original; the
  response is recorded once and holds for neither again.
- [`weir-pack` depends on the kind format and the engine] → its tasks
  name both crates' files among their inputs, and its build takes them in
  the one workspace.

## Migration Plan

Nothing migrates: no record, layout, shared-branch write or kind format
changes, and the engine's calls are new. flume takes the engine when it
moves its pin of weir past this change, in its own change, and keeps an
object's state in a place's held state as it chooses.
