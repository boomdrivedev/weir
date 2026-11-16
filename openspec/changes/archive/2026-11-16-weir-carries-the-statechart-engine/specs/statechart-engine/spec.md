## Purpose

The engine a statechart kind runs on, in `weir-engine`: the tick, one
pure function over a statechart, the templates it runs, the proofs its
reader names, one object's state and one snapshot, giving the
transitions that fire with their effects and the object's next state;
an object's first state, the decisions standing in a state and the
names a statechart reads and runs, each answering from its inputs,
holding no store and naming no reader's object or atom (96 to 99; S21).

## ADDED Requirements

### Requirement: The tick answers from its inputs alone

`tick` SHALL take a statechart, the templates it runs by name, the proofs its reader names by effect, one object's state and one snapshot, and SHALL give the transitions that fire, each with the effects it plans, and the object's next state. It SHALL read no clock, store, file or environment, SHALL change none of its inputs, and the same inputs SHALL always give the same tick (69, 96, 98, 99; I7; core.1, core.55).

#### Scenario: The lamp switched on (S21)
- **WHEN** S21's lamp is ticked in `off`, with `lamp.switch` read `up` and `lamp.lit` not read
- **THEN** the tick gives one transition, `power` from `off` to `on`, planning `light` at entry, and the next state holds `power` in `on`

#### Scenario: The lamp on with its proof present (S21)
- **WHEN** S21's lamp is ticked in `on`, with `lamp.lit` read `true`
- **THEN** the tick gives no transition, and the next state is the state given

#### Scenario: A tick given twice
- **WHEN** one statechart, state and snapshot are ticked twice
- **THEN** both ticks are equal

#### Scenario: Ticking changes nothing
- **WHEN** the lamp, its templates, its proofs, a state and a snapshot are ticked
- **THEN** each is equal to a copy taken before the tick

### Requirement: In each region, innermost first, the first transition whose guard holds fires

The tick SHALL try, for each region path the state holds, deepest first and in path order at one depth, the transitions out of its state in the order written, and the first whose guard holds SHALL fire, at most one per region (96; core.55).

#### Scenario: The first transition whose guard holds
- **WHEN** the lamp is ticked in `off` with a response answering `on` and `lamp.switch` read `up`
- **THEN** the transition on the response fires and takes it, and no other transition of `power` fires

#### Scenario: Innermost first, at most one per region
- **WHEN** the lamp is ticked in `on`, `level` in `dim`, with `lamp.level` read 7 and `lamp.faulty` read `true`
- **THEN** `level` moves to `bright` and then `power` to `off`, in that order, each once

### Requirement: Every guard of a tick reads one snapshot and the state as given

Every guard of one tick SHALL read the one snapshot and the object's state as the tick was given it, so a region's move is seen by its siblings, and by the statecharts that run it or that it runs, at the next tick and never within the same one (96; core.55).

#### Scenario: A sibling sees a move at the next tick
- **WHEN** a statechart of two regions, `shade` guarding on `power`'s state, is ticked where `power` moves to the state `shade` waits on
- **THEN** `power` moves and `shade` does not, and the next tick moves `shade`

### Requirement: A transition to its own state with nothing to do fires nothing

A transition to the state it leaves that plans no effect, takes no response, bumps nothing and names no `enter` SHALL fire nothing, and its region SHALL try no later transition that tick, so reading twice with nothing changed writes nothing (96; core.55).

#### Scenario: A transition to its own state with nothing to do
- **WHEN** a state's first transition, to itself with nothing, and its second, to another state, hold under one snapshot
- **THEN** nothing fires

### Requirement: Firing plans its effects, each only while its proof is absent

A fired transition SHALL plan, in order, the left state's `exit` effects where it moves to another state, its own `effects`, and the entered state's `entry` effects where it moves to another state, then the `entry` effects of each state its `enter` moves a region to. An effect SHALL be left out while the snapshot holds its proof, read at the region where it is planned, with any value but `false`; an effect whose atom has no proof given SHALL be planned whenever it is due (96; core.55).

#### Scenario: Exit, transition and entry effects in order
- **WHEN** a transition moves from a state with an `exit` effect to one with an `entry` effect, with an effect of its own
- **THEN** it plans the exit effect, its own and the entry effect, in that order, each with its phase and region

#### Scenario: An effect whose proof is present
- **WHEN** S21's lamp is ticked in `off` with `lamp.switch` read `up` and `lamp.lit` read `true`
- **THEN** `power` moves to `on` and plans no effect

#### Scenario: An effect with no proof named
- **WHEN** the lamp's `set_mode` is due and no proof is given for it
- **THEN** it is planned

#### Scenario: A transition to its own state runs its own effects alone
- **WHEN** a transition from a state with `exit` and `entry` effects to itself plans an effect of its own
- **THEN** it plans its own effect alone

### Requirement: Firing records the move

The next state SHALL hold each fired transition's region in its target, entered at the snapshot's time where the target is another state and at the time it held where it is the same, SHALL count one more for the counter the transition bumps, and SHALL add the response the guard took to the applied responses once (96, 97; core.55).

#### Scenario: Firing records the move
- **WHEN** the lamp moves from `on` to `off` on its bulb's `burnt`, at a snapshot's time
- **THEN** the next state holds `power` in `off`, entered at that time, and `burnouts` one more than before

#### Scenario: A transition to its own state keeps its time
- **WHEN** the lamp in `off` takes a response answering `mode warm`
- **THEN** the next state holds `power` in `off` at the time it was entered before

### Requirement: A state that runs a statechart starts it afresh each time it is entered

A state that runs a template, or holds regions of its own, SHALL start them when entered, each region at its initial state, entered at the snapshot's time. Where the state already holds an instance, its entry SHALL set that instance aside under its next attempt, `<region path>.<state>#<n>` counting from 1, with each region's state, and start a fresh one. Leaving the state SHALL keep what it ran at its path, holding the state it reached (97; core.55).

#### Scenario: A template started afresh each time its state is entered
- **WHEN** the lamp moves from `off` to `on` while its bulb's earlier instance stands at `power.on.glow` in `lit`
- **THEN** the next state holds `power.on.glow` in `warming`, and sets `glow` in `lit` aside under `power.on#1`

#### Scenario: Leaving a state keeps what it ran
- **WHEN** the lamp moves from `on` to `off` with its bulb in `lit` and `level` in `bright`
- **THEN** the next state holds `power.on.glow` in `lit` and `power.on.level` in `bright`

### Requirement: A final guard reads a final state of what a state runs

A `final` guard SHALL hold only where a region it sees is in a state of that name that is final in its region. It SHALL see what the state it leaves runs, its template's regions and its own; where that state runs nothing, it SHALL see what an earlier state of its region ran, still standing at its path (96, 97; core.55).

#### Scenario: A final state of what a state runs
- **WHEN** the lamp in `on` is ticked with its bulb's `glow` in `burnt`
- **THEN** `power` moves to `off` and bumps `burnouts`

#### Scenario: A state of that name that is not final
- **WHEN** a statechart's `final` guard names a state of the template it runs that is not final, and the template is in it
- **THEN** the guard does not hold

### Requirement: An enter moves what a state ran

An `enter`, the entered state's or the transition's, the transition's winning where both name a region, SHALL move each region its key names, by the region's name, the state that holds it or the template that state runs, to the state it names, whether the region is live or left, the outermost match alone (96, 97; core.55).

#### Scenario: An enter moves what a state ran
- **WHEN** the lamp in `on` is ticked with `lamp.switch` read `down`
- **THEN** `power` moves to `off`, `power.on.glow` moves to `cold`, and `cool` is planned at `power.on.glow` in the phase `enter`

### Requirement: A response is applied once

A `response` guard SHALL hold for the first response of the snapshot, in its order, that the state has not applied and whose answer its pattern matches. The fired transition SHALL carry the response's id, the next state SHALL apply it once, and a response the state has applied SHALL never hold again (97; core.55).

#### Scenario: A response applied once
- **WHEN** the lamp in `off` is ticked with `r1` answering `on`
- **THEN** `power` moves to `on` taking `r1`, and the next state has applied `r1`

#### Scenario: A response already applied
- **WHEN** the lamp in `off`, having applied `r1`, is ticked with `r1` answering `on`
- **THEN** nothing fires

#### Scenario: Two regions take one response in one tick
- **WHEN** two regions each hold a transition on the response `go`, and the snapshot holds one response answering `go`
- **THEN** both fire taking it, and the next state applies it once

#### Scenario: The evidence branch of any takes no response
- **WHEN** the lamp in `on` moves to `off` on `lamp.faulty` under its `any` of that evidence and the response `off`
- **THEN** the transition takes no response

### Requirement: A response's pattern binds the argument its answer gives

A pattern SHALL match a bare word as itself, `word <arg>` binding what follows the word, `word: <text>` binding what follows the colon, `<arg>` alone binding the whole answer, and `<arg> tail` binding what comes before the tail. The fired transition SHALL carry the argument bound, and an effect's argument `$response` SHALL be planned as that argument (97; core.55).

#### Scenario: A response binds its argument
- **WHEN** the lamp in `off` is ticked with `r1` answering `mode warm`
- **THEN** `power` moves to `off` taking `r1` with the argument `warm`, and plans `set_mode` with `mode` `warm`

#### Scenario: An answer matched by its pattern
- **WHEN** answers are matched against `yes`, `route <name>`, `redo: <notes>`, `<text>` and `<name>: drop`
- **THEN** each matches and binds as the original's matcher does, and a word that is not first, a colon left out or a case changed does not match

### Requirement: A guard reads its evidence from the snapshot alone

An evidence SHALL be read for a region path from the values the snapshot gives at that path, else at the nearest path above it, else for the object, and the engine SHALL serve no evidence of its own. An evidence not read SHALL hold no test but `exists = false`, and `ne_ev`'s other evidence not read SHALL hold (96, 98; core.55).

#### Scenario: An evidence read or not
- **WHEN** guards test `exists` on an evidence read and on one not read, and `is` on one not read
- **THEN** `exists = true` holds on the first, `exists = false` on the second, and `is` on the third does not

#### Scenario: Evidence read at its region
- **WHEN** the lamp in `on` is ticked with `lamp.level` read 7 at `power.on.level` and 1 for the object
- **THEN** `level` moves to `bright`

#### Scenario: No evidence of the engine's own
- **WHEN** guards test `state`, `entered_at` and `now` by `exists = false`, and the snapshot gives none of them
- **THEN** each holds, whatever the state holds

### Requirement: Values compare by their type, their number and their instant

Two values SHALL be equal by `toml`'s equality, but that an integer and a float are equal where their numbers are and two times where their instants are. Numbers SHALL order by value, strings by their bytes and times by instant, and any other pair SHALL have no order, a guard ordering it not holding (94, 96; core.55).

#### Scenario: Is and in
- **WHEN** guards test `lamp.mode` read `warm` by `is` and `in`
- **THEN** `is = "warm"` and `in = ["cool", "warm"]` hold, and `in = ["cool"]` does not

#### Scenario: Values in order
- **WHEN** guards order `lamp.level` read 3 by `gt`, `gte`, `lt` and `lte`, and a string against a number
- **THEN** each holds as the numbers order, and the string against the number does not

#### Scenario: Against another evidence
- **WHEN** `lamp.level` is read 5, `lamp.target` 5 and `lamp.other` 6
- **THEN** `eq_ev`, `ne_ev`, `gt_ev`, `gte_ev` and `lt_ev` hold as the values compare, and `ne_ev` against an evidence not read holds

#### Scenario: Values of two types
- **WHEN** `lamp.level` is read 5 and tested against `5.0` and `"5"`
- **THEN** it equals `5.0` and does not equal `"5"`

### Requirement: An age is measured to the snapshot's time

A time SHALL be a datetime with its offset or a string in RFC 3339, and `older` SHALL hold where the snapshot's time is more than its duration after the evidence read; an evidence that is no time SHALL never hold it (94, 96; core.55).

#### Scenario: A guard by age
- **WHEN** `lamp.since` is read ten minutes before the snapshot's time, then eleven
- **THEN** `older = "10m"` does not hold at ten and holds at eleven, and an evidence not read or not a time never holds it

### Requirement: A guard reads another region, its owner and its children as given

A `region` guard SHALL read the state the object's state holds at a region: by a dotted name, a path equal to it or ending in it; by a bare name, a region of that name under the state the guard leaves or beside the guard's region. A `parent` guard SHALL read the states the snapshot gives its owner, and a `children` guard the kind and states it gives each child. `all`, `any`, `not` and `always` SHALL combine as their names say, `all` of none holding and `any` of none not (94, 96; core.55).

#### Scenario: A sibling region
- **WHEN** a guard names the region `power` beside its own, and a dotted path to a region under a state of it
- **THEN** each holds where that region is in a state listed, and not otherwise

#### Scenario: The owner and the children
- **WHEN** the snapshot gives an owner in `open` and two children of the kind `bulb`, one `lit` and one `burnt`
- **THEN** `parent` with `in = ["open"]`, `children` with `any = ["lit"]` and with `count_gte = 2` hold, `children` with `all = ["lit"]` and with `none = ["burnt"]` do not, and `children` with `all` of a kind with no child does not

#### Scenario: All, any, not and always
- **WHEN** guards combine one that holds and one that does not
- **THEN** `all` of both does not hold, `any` of both does, `not` of the first does not, `always = true` holds, `always = false` does not, `all` of none holds and `any` of none does not

### Requirement: An object starts in its initial states

`start` SHALL give a statechart's first state for an object: each top region in its initial state, entered at the time given, with what each initial state runs started, and no counter, applied response or instance set aside. It SHALL plan no effect (96, 97; core.55).

#### Scenario: A lamp starts off
- **WHEN** the lamp is started at a time
- **THEN** its state holds `power` in `off`, entered at that time, and nothing else

#### Scenario: A state that runs a template starts it
- **WHEN** a statechart whose initial state runs the bulb and holds a region `level` is started
- **THEN** its state holds the bulb's `glow` in `warming` and `level` in its initial state under that state's path

### Requirement: The engine refuses a state or a template it cannot read

`start`, `tick` and `decisions` SHALL refuse, naming the path: a region path that names no region of the statechart; a region path holding a state its region does not hold; a state that runs a name no template given carries; a state that runs a statechart it already lies inside; and a state holding a region by a name both its own regions and its template's hold (96, 97; core.55).

#### Scenario: A region the statechart does not hold
- **WHEN** the lamp is ticked with a state holding `shade` in `up`
- **THEN** the tick is refused naming `shade`

#### Scenario: A state its region does not hold
- **WHEN** the lamp is ticked with a state holding `power` in `dimmed`
- **THEN** the tick is refused naming `power`

#### Scenario: A template no one gave
- **WHEN** the lamp in `off` is ticked with `lamp.switch` read `up` and no templates given
- **THEN** the tick is refused naming `power.on` and `bulb`

#### Scenario: A statechart that runs itself
- **WHEN** a statechart whose initial state runs that statechart is started
- **THEN** it is refused naming that state's path

#### Scenario: A region a state and its template both hold
- **WHEN** a state holds a region `glow` of its own and runs the bulb
- **THEN** entering it is refused naming its path and `glow`

### Requirement: A decision stands while its state is active

`decisions` SHALL give, in path order, each live region whose state carries a decision, with its path, its state, the decision as declared and the time its state was entered, and SHALL give no decision on a region whose state its region has left. It SHALL number, fold and group nothing (97; core.55).

#### Scenario: A decision stands while its state is active
- **WHEN** the lamp's state holds `power` in `off`
- **THEN** `decisions` gives the decision `switch` at `power`, in `off`, since the time `off` was entered

#### Scenario: A decision whose state was left
- **WHEN** the lamp's state holds `power` in `on`
- **THEN** `decisions` gives none

### Requirement: The engine lists the names a statechart reads and runs

`names` SHALL give every evidence a statechart's guards read, the evidence its comparisons with another evidence name among them, every effect its states and transitions plan, and every template its states run, through its states' own regions and not into the templates (98; core.55).

#### Scenario: The names the lamp reads and runs
- **WHEN** `names` is called on the lamp
- **THEN** it gives the evidence `lamp.switch`, `lamp.level` and `lamp.faulty`, the effects `light` and `set_mode`, and the template `bulb`

### Requirement: The engine names no reader's object or atom

No source of `weir-engine` SHALL name a tool of the family, an object the family's readers run, or an atom of the toy statechart its tests run, as a whole word in any case. The toy SHALL name only atoms of a lamp and its bulb (98; core.2, core.55).

#### Scenario: The code names no tool of the family
- **WHEN** every file under the crate's `src` is read
- **THEN** none names a tool of the family

#### Scenario: The engine names no reader's object
- **WHEN** every file under the crate's `src` is read
- **THEN** none names `place`, `stage`, `session`, `agent`, `intent`, `elaboration`, `bolt`, `unit`, `claim` or `verdict` as a whole word

#### Scenario: The engine names none of its toy's atoms
- **WHEN** every file under the crate's `src` is read
- **THEN** none names `lamp`, `bulb`, or an evidence or effect `names` gives for the lamp or the bulb

#### Scenario: A name put back is found as a whole word
- **WHEN** the check reads `let x = obj.bolt;`, `// a claim`, `let bolted = 1;` and `reunited`
- **THEN** it finds `bolt` and `claim` in the first two and nothing in the last two
