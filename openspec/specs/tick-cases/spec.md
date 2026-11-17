# tick-cases Specification

## Purpose
The pack's `tick` cases: how a case's folder holds a statechart kind,
its templates, its proofs, one object's state, one snapshot and the
tick it expects, how `weir-pack` reads one through the kind format and
runs it twice against weir's engine, and the cases the pack holds, the
toy lamp's of S21 among them (65, 96 to 99; C.7; S21).

## Requirements

### Requirement: A tick case is read whole

A tick case's `case.toml` SHALL hold `satisfies`, a declaration, the statechart kind of it to tick in `chart`, the proofs by effect, the object's state, the snapshot and what it expects: the transitions that fire, each with its effects, and the next state (C.7). A field the form does not name SHALL refuse the case, naming the field (core.35).

#### Scenario: A misspelt field
- **WHEN** a tick case's snapshot holds `evidense` in place of `evidence`
- **THEN** the case is refused naming `evidense`

### Requirement: A tick case's statechart is read by the kind format

A tick case's declaration SHALL be read by `read-kinds`, a declaration it refuses refusing the case with its refusal. Every statechart kind of it but the chart SHALL be a template, by its name, and a `chart` naming no statechart kind of the declaration SHALL refuse the case, naming `chart` (core.30, core.54).

#### Scenario: A chart that is no statechart
- **WHEN** a tick case's `chart` names a simple kind of its declaration, or no kind of it
- **THEN** the case is refused naming `chart`

#### Scenario: A declaration the kind format refuses
- **WHEN** a tick case's declaration holds a transition to a state its region does not hold
- **THEN** the case is refused with `read-kinds`' refusal, naming the transition's `to`

### Requirement: A tick case is ticked twice and compared

The pack SHALL tick each tick case's chart twice, with weir's engine, from copies of its templates, proofs, state and snapshot, and SHALL name the case and the field of each difference: a transition or an effect other than expected, a part of the next state other than expected, two ticks unequal, or an input changed by the tick (65, 96, 98, 99; core.35).

#### Scenario: A case whose next state differs
- **WHEN** a tick case expects `power` in `off` where the tick leaves it in `on`
- **THEN** the run names the case and `next.regions.power`

#### Scenario: A case whose effects differ
- **WHEN** a tick case expects no effect where the tick plans `light`
- **THEN** the run names the case and the transition's `effects`

#### Scenario: A case read from its folder
- **WHEN** one tick case is run from a pack holding its folder alone
- **THEN** it runs with no difference

### Requirement: The pack holds the tick cases

The pack SHALL hold under `pack/tick/` the fifteen cases the design names, S21's two ticks among them, each naming no reader's atom and running with no difference, and together citing 96, 97, 98 and 99 (67, 96 to 99; C.7; S21; core.35).

#### Scenario: The cases named
- **WHEN** the folders under `pack/tick/` are read
- **THEN** they are the fifteen cases the design names, each citing the clauses it names

#### Scenario: The tick cases
- **WHEN** every case under `pack/tick/` is read and run
- **THEN** each reads and runs with no difference

#### Scenario: The tick cases cite 96 to 99
- **WHEN** the repository's pack is read
- **THEN** its tick cases cite each of 96, 97, 98 and 99
