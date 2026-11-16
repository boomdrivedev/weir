## Purpose

A statechart kind, the kind format's second format beside the simple
kind: what its declaration holds, how `read-kinds` reads it into the
types of `weir-engine`, the engine's crate, and checks it whole, and the
pack's cases of it (51, 81, 94, 95; C.4; S19, S20).

## ADDED Requirements

### Requirement: A kind's table holding regions is a statechart kind

A `[kinds.<kind>]` table holding `regions` and no `stages` SHALL be read by `read-kinds` as a statechart kind, into the engine's types, the kind's name and version its statechart's own, and returned in the declaration's `statecharts`, beside its simple kinds in `kinds` (51, 81, 94; C.4; core.54).

#### Scenario: A statechart kind beside a simple kind
- **WHEN** a declaration holds the simple kind `bump`, with stages `code` and `review`, and C.4's statechart kind `persona`, whose `test` state runs `stage` with agents by the rule `every-persona-definition` and joins all
- **THEN** `read-kinds` gives `bump` in `kinds` with its two stages, and `persona` in `statecharts` at version 1, its statechart named `persona` with the one region `stages`, each as written, and a second read gives a declaration equal to the first (S19)

#### Scenario: read-kinds gives the engine's types
- **WHEN** a statechart kind is read
- **THEN** its `statechart` is a `weir_engine::Statechart`, named through `weir_kinds::weir_engine`

### Requirement: A table holding both stages and regions is refused

A `[kinds.<kind>]` table holding both `stages` and `regions` SHALL be refused naming its `regions`, and nothing else of it SHALL be read (95; core.54).

#### Scenario: A table holding both stages and regions
- **WHEN** a declaration holds `bump` and `persona` as C.4 writes them and a third kind, `both`, whose table holds one stage and a region
- **THEN** the declaration is refused naming `kinds.both.regions`, its reason holding `beside \`stages\``, and naming no other field of `both` (S19)

### Requirement: A table holding agents and regions is a statechart kind

A `[kinds.<kind>]` table holding `agents` and `regions` SHALL be read as a statechart kind, `agents` among its sections, and SHALL give no settings (57; core.41).

#### Scenario: Agents beside regions
- **WHEN** a kind's table holds `[kinds.persona.agents]` with `model = "opus"` beside its title and regions
- **THEN** it reads as the statechart kind `persona`, `agents` returned among its sections, and gives no settings for `persona`

### Requirement: A simple kind reads as it did

A simple kind SHALL read, return and be refused as it did, a `title` or `label` it declares returned among its sections (52, 56; flume record 0028).

#### Scenario: A simple kind's title and label are sections
- **WHEN** a simple kind with one stage declares `title = "Bump"` and `label = "construction"`
- **THEN** it reads in `kinds` as before, with `title` and `label` returned among its sections

### Requirement: A statechart kind reads its own fields

A statechart kind SHALL read its `version`, 1 when left out; its `title`; its `label`, left out when left out; its `branch`, its name when left out; its `lands`, false when left out; and its `landing_fix` and `max_landing_fixes`, each left out when left out and either without the other, each in a simple kind's shape (52, 57, 94; C.4). Every other key of its table SHALL be returned among its sections (56).

#### Scenario: A statechart kind reads with every field
- **WHEN** a statechart kind declares `version = 2`, `title = "Persona test"`, `label = "construction"`, `branch = "persona"`, `lands = true`, `landing_fix = "acme/fix@1"`, `max_landing_fixes = 2` and `owner = "release"` beside its regions
- **THEN** it reads at version 2 with that title, label and branch, landing, its landing fix `acme/fix@1` and its bound 2, and `owner` returned unread

#### Scenario: A statechart kind's fields left out
- **WHEN** a statechart kind declares its title and regions alone
- **THEN** it reads at version 1, its branch its own name, not landing, with its label, landing fix and bound left out and no section unread

### Requirement: A statechart kind holds its title

A statechart kind SHALL hold a `title`, a non-empty string, the words a person reads for it, and one with none, or with one of another shape, SHALL be refused naming its `title` (94).

#### Scenario: A statechart kind with no title
- **WHEN** one statechart kind declares no `title`, and another `title = ""`
- **THEN** the declaration is refused naming each kind's `title`

### Requirement: Only a statechart kind that lands names its landing fix

A `landing_fix` or `max_landing_fixes` on a statechart kind whose `lands` reads false SHALL be refused naming each; beside a `lands` the read refused, neither SHALL be judged against it (52, 57, 94; record 0005).

#### Scenario: A landing fix on a statechart kind that does not land
- **WHEN** a statechart kind with `lands` left out declares `landing_fix = "acme/fix@1"` and `max_landing_fixes = 2`
- **THEN** the declaration is refused naming its `landing_fix` and then its `max_landing_fixes`, each reason holding `does not land`

### Requirement: A statechart's regions and states read as written

Each region SHALL read its `initial` and its `states` in name order. Each state SHALL read `final`, false when left out, its `decision`, `entry` and `exit`, `machine`, `params` as written, `enter`, its own `regions`, none when left out, and its `transitions` in the order written. Every other key of a region or a state SHALL be returned among its sections (56, 94; C.4).

#### Scenario: A statechart reads as written
- **WHEN** C.4's `persona` is read
- **THEN** its region `stages` starts at `build` and holds `build`, `dropped`, `passed`, `stopped` and `test`, `dropped` and `passed` final and the others not; `build` and `test` run `stage`; `test`'s transitions go to `passed`, `build` and `stopped` in that order; and `stopped` carries a decision of kind `stalled` answering `retry` or `drop`

### Requirement: A statechart holds regions and each region states

A statechart kind's `regions` SHALL be a table of one or more regions, and each region's `states` a table of one or more states; one with none, or of another shape, SHALL be refused naming the field (94, 95).

#### Scenario: A statechart kind with no region
- **WHEN** a statechart kind declares `regions = {}`
- **THEN** the declaration is refused naming its `regions`

#### Scenario: A region with no states
- **WHEN** a region declares `initial = "a"` and `states` left out, and another region `states = {}`
- **THEN** the declaration is refused naming each region's `states`

### Requirement: A region's initial state is one it holds

A region's `initial` SHALL be a non-empty string naming a state of the region, and SHALL be refused naming it when missing, of another shape, or naming no state the region holds (95).

#### Scenario: An initial state the region does not hold
- **WHEN** a region holding the states `a` and `b` declares `initial = "c"`
- **THEN** the declaration is refused naming that region's `initial`, its reason holding `c`

### Requirement: A region's and a state's key is a name

A region's or a state's key SHALL be a name, one or more ASCII letters, digits, `_` and `-`, so a dotted path to it reads one way; a key that is no name SHALL be refused naming it, and nothing under it SHALL be read (C.4; core.42).

#### Scenario: A state named by a key that is not a name
- **WHEN** a region holds a state under the key `"a.b"`
- **THEN** the declaration is refused naming `kinds.<kind>.regions.<region>.states."a.b"`, and nothing under it is read

### Requirement: A state's own regions are checked as any region

A state's own `regions` SHALL be read and checked as a statechart kind's regions are (94, 95).

#### Scenario: A nested region is checked as any
- **WHEN** a state holds a region of its own, `checks`, whose `initial` names no state it holds
- **THEN** the declaration is refused naming `kinds.<kind>.regions.<region>.states.<state>.regions.checks.initial`

### Requirement: A transition goes to a state its region holds

A transition SHALL hold `when`, a guard, and `to`, a non-empty string naming a state of the transition's own region, and MAY hold `effects`, `bump` and `enter` (94, 96; C.4). A transition with no `when` or no `to`, a `to` naming no state of its region, and a field of the wrong shape SHALL each be refused naming the field (95).

#### Scenario: A transition to a state its region does not hold
- **WHEN** `persona`'s `test` state's first transition declares `to = "tested"`, a state its region does not hold
- **THEN** the declaration is refused naming `kinds.persona.regions.stages.states.test.transitions[0].to` (S20)

#### Scenario: A transition with no guard or target
- **WHEN** a transition declares `to = "b"` and no `when`, and another declares a `when` and no `to`
- **THEN** the declaration is refused naming the first's `when` and the second's `to`

### Requirement: A guard is one of the forms C.4 names

A guard SHALL be a table of exactly one form: `all` or `any`; `not`; `ev` with exactly one of `is`, `in`, `exists`, `gt`, `gte`, `lt`, `lte`, `older`, `eq_ev`, `ne_ev`, `gt_ev`, `gte_ev` or `lt_ev`; `response`; `final`; `region`, with `name` and `in`; `parent`, with `in`; `children`, with `kind` and exactly one of `all`, `any`, `none` or `count_gte`; or `always` (94; C.4).

#### Scenario: Every form of guard reads
- **WHEN** a statechart's transitions declare, in turn, `{ all = [{ always = true }] }`, `{ any = [{ response = "yes" }] }`, `{ not = { final = "done" } }`, `{ ev = "switch", is = "up" }`, `{ ev = "n", in = [1, 2] }`, `{ ev = "lock", exists = false }`, `{ ev = "n", gt = 1 }`, `{ ev = "n", gte = 1 }`, `{ ev = "n", lt = 1 }`, `{ ev = "n", lte = 1 }`, `{ ev = "seen", older = "2h" }`, `{ ev = "a", eq_ev = "b" }`, `{ ev = "a", ne_ev = "b" }`, `{ ev = "a", gt_ev = "b" }`, `{ ev = "a", gte_ev = "b" }`, `{ ev = "a", lt_ev = "b" }`, `{ region = { name = "lamp", in = ["on"] } }`, `{ parent = { in = ["open"] } }`, `{ children = { kind = "task", all = ["done"] } }` and `{ children = { kind = "task", count_gte = 2 } }`
- **THEN** each reads as its form with its values, `older` as two hours

### Requirement: A guard of no form is refused naming its path

A guard of no form C.4 names SHALL be refused naming its path, built from its transition's `when` and, within it, `all[<n>]`, `any[<n>]` and `not`, its reason holding `is not a guard of a form the format names`; a form's field of the wrong shape SHALL be refused naming that field (95).

#### Scenario: A guard of a form the format does not name
- **WHEN** transitions declare `{ ev = "switch" }`, `{ ev = "n", is = 1, gt = 2 }`, `{ all = [{ always = true }], any = [{ always = true }] }`, `{ always = true, response = "yes" }`, `{ children = { kind = "task" } }`, `"always"` and `{ any = [{ always = true }, { when = 1 }] }`
- **THEN** the declaration is refused naming each guard's path, the last at its `when.any[1]`, each reason holding `is not a guard of a form the format names`

### Requirement: An older is a duration

A guard's `older` SHALL be a whole number followed by `s`, `m` or `h`, read as the duration it names, as a stage's `quiet` is, and any other SHALL be refused naming it (95; C.4).

#### Scenario: An older that is no duration
- **WHEN** a guard declares `{ ev = "seen", older = "2 days" }`
- **THEN** the declaration is refused naming that guard's `older`

### Requirement: Effects and decisions read with their atoms and answers

An effect SHALL hold `do`, a non-empty string, and `args`, a table, empty when left out. A decision SHALL hold `kind`, a non-empty string, and `answers` and `shows`, arrays of strings, each empty when left out. Every other key of each, `doc` and `note` among them, SHALL be returned among its sections (56, 94, 97; C.4).

#### Scenario: Effects and decisions read as written
- **WHEN** a state declares `entry = [{ do = "light", args = { level = 3 } }]`, `exit = [{ do = "dim", note = "softly" }]` and `decision = { kind = "stalled", answers = ["retry", "drop"], shows = ["log"], doc = "asks once" }`, and its transition `effects = [{ do = "notify" }]`, `bump = "tries"` and `enter = { inner = "start" }`
- **THEN** each reads as written, `note` and `doc` returned among the sections of their effect and decision, and `notify`'s args empty

### Requirement: An effect names its atom and a decision its kind

An effect with no `do`, a decision with no `kind`, and a field of either of the wrong shape SHALL each be refused naming the field (95).

#### Scenario: An effect with no atom and a decision with no kind
- **WHEN** a state declares `entry = [{ args = { level = 3 } }]` and `decision = { answers = ["yes"] }`
- **THEN** the declaration is refused naming `entry[0].do` and `decision.kind` of that state

### Requirement: A state's params read the stage template's parameters

Wherever a state gives `params`, whatever its `machine` names, `read-kinds` SHALL read its `agents`, `join`, `on_fail`, `max_send_backs` and `deliverables` as the stage template's parameters, returned in the kind's `params` by the state's path, its regions' and its own names joined by `.`. Every other key of `params` SHALL be returned as written in the state's `params` (94; C.4).

#### Scenario: A state's params read as the stage template's
- **WHEN** C.4's `persona` is read, and a state `move` of another kind runs `command` with `params = { command = ["bin/move"], deliverables = ["commits"], on_fail = "fix", max_send_backs = 3 }` beside a state `fix`
- **THEN** `persona`'s params at `stages.build` give one agent entry, join all, a send-back to `build` at most twice and the deliverable commits; at `stages.test` the rule `every-persona-definition` with the pattern `personas/*.md`, join all and the deliverable a committed report at `tests/personas.md`; and `move`'s give a send-back to `fix` at most three times and commits, its `command` returned as written in its state's `params`

### Requirement: A join is all, any or a count

A state's `params.join` SHALL be `all`, `any` or `count:<n>`, `n` a whole number of at least 1 with no leading zero, and any other SHALL be refused naming it (95; C.4).

#### Scenario: A join of all, any or a count
- **WHEN** states declare `join = "any"`, `join = "count:3"`, `join = "count:0"`, `join = "count:03"` and `join = "most"`
- **THEN** the first two read as any and a count of 3, and the declaration is refused naming the `params.join` of each of the other three

### Requirement: A state's deliverables are of the format's spellings

Each of a state's `params.deliverables` SHALL be spelled as a simple stage's deliverable is, and a state SHALL name several or none; any other SHALL be refused naming `params.deliverables` (54, 95; C.4).

#### Scenario: A deliverable in params not of the forms
- **WHEN** a state's params declare `deliverables = ["commits", "commit"]`
- **THEN** the declaration is refused naming that state's `params.deliverables`

### Requirement: A send-back names a state of its region

A state's `params.on_fail` SHALL name a state of the state's own region, as a simple stage's names a stage of its kind, and `max_send_backs` SHALL be at least 0; any other SHALL be refused naming the field (52, 95; C.4).

#### Scenario: A send-back to no state of its region
- **WHEN** a state's params declare `on_fail = "nowhere"`, a state its region does not hold
- **THEN** the declaration is refused naming that state's `params.on_fail`, its reason holding `nowhere`

### Requirement: A state's agents are entries or a rule

A state's `params.agents` SHALL be an array of tables, each entry's settings as written, or a table holding `rule`, a non-empty string, and, where given, `pattern`, a string, and no other key; any other SHALL be refused naming it (94, 95; C.4).

#### Scenario: An agents rule of the wrong shape
- **WHEN** one state's params declare `agents = { pattern = "personas/*.md" }` and another's `agents = { rule = "each", extra = 1 }`
- **THEN** the declaration is refused naming each state's `params.agents`

### Requirement: Which templates and atoms there are is the reader's

`read-kinds` SHALL check no `machine`, `ev`, `do`, decision `kind`, `bump` or `response` against a list, SHALL hold no `enter` or `final` guard to a state of a statechart the state runs, and SHALL evaluate no guard, perform no effect and answer no decision (58, 95, 98; core.32, core.54).

#### Scenario: Any template or atom name reads
- **WHEN** a statechart kind's states run `machine = "anything"`, its guards read `ev = "whatever"` and `final = "inner.done"`, its effects `do = "something"`, and an `enter = { inner = "elsewhere" }` names a state no region of the kind holds
- **THEN** it reads, each name as written

### Requirement: A statechart kind is checked whole

A declaration holding a failing statechart kind SHALL be refused whole, returning no kind, its simple kinds' included, and naming every failing field in the reader's walk: `version`, `branch`, `title`, `label`, `lands`, the regions in name order, then `landing_fix` and `max_landing_fixes`. A region's states come in name order, then its `initial`, each `to` and each `params.on_fail` checked against them; where its `states` is refused, `initial` is named for its shape alone (57, 81, 95).

#### Scenario: A statechart kind checked whole
- **WHEN** a declaration holds a simple kind that reads and a statechart kind with `version = 0`, a region `a` declaring `initial = "x"` and no states, and a region `b` whose `initial` names no state it holds, whose state `s` declares `final = "yes"` and `params = { join = "most" }`, and whose transition from `s` goes to `nowhere`
- **THEN** the declaration is refused naming, in order, the kind's `version`, `regions.a.states`, `regions.b.states.s.final`, `regions.b.states.s.params.join`, `regions.b.initial` and `regions.b.states.s.transitions[0].to`, and returns no kind

### Requirement: A declaration's kinds case expects its statechart kinds

A declaration's kinds case SHALL name in `statecharts`, beside `kinds`, the statechart kinds it reads into: each one's version, title, label and landing fix and bound where named, sections unread, regions' initials and states, each state's `final`, `machine`, targets, answers and own regions, and `params` by path. The kinds crate's tests SHALL name the case, the kind and the field of each difference (C.4; C.7; core.35).

#### Scenario: A case whose statechart differs is named
- **WHEN** a case expects `persona`'s `test` state's transitions to go to `passed` and `stopped`, and the kind read goes to `passed`, `build` and `stopped`
- **THEN** the crate's tests give a difference naming the case, `persona`, the state and both lists of targets

### Requirement: The pack's kinds cases hold S19 and S20

The pack SHALL hold kinds cases of S19's declaration, its kind holding both `stages` and `regions`, and S20's transition, and its kinds cases SHALL cite 94 and 95 (67; C.7; core.35).

#### Scenario: The cases of S19 and S20
- **WHEN** the kinds cases `a-statechart-kind-beside-a-simple-kind`, `a-kind-holding-both-stages-and-regions` and `a-transition-to-a-state-its-region-does-not-hold` are read by the kinds crate
- **THEN** the first gives `bump` with its stages and `persona` with its statechart as the case expects, the same twice; the second the refusal naming `kinds.both.regions`; and the third the refusal naming the transition's `to`

#### Scenario: The cases cite 94 and 95
- **WHEN** every kinds case's `satisfies` is read
- **THEN** 94 and 95 are each cited by a case

### Requirement: The engine's crate names no tool

`weir-engine`'s code SHALL name no tool of the family, each name read as a whole word (core.2).

#### Scenario: The engine's code names no tool of the family
- **WHEN** every file under `crates/weir-engine/src` is read
- **THEN** none holds `millpond`, `flume`, `sounding`, `crew`, `headwater`, `eddy`, `boomdrive` or `swancloud` as a whole word in any case
