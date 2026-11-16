# Proposal

## Why

A statechart kind runs on the family's engine (flume 176), and weir is
to carry that engine: the original Boomdrive's flywheel-engine, moved
into weir as a crate beside `weir-kinds` with its tests and its toy
statechart, and taken by a reader at the revision of weir it reads kinds
at (flume's record 0029, accepted by the user on 2026-11-01). The
statement holds it: A.13 says what a tick is (96), how a statechart a
state runs starts, is set aside and is read (97), and that the engine
names no reader's object or atom, holds no store and performs nothing
(98). B.4 gives `tick` beside `read-kinds` (99), C.7 the pack's `tick`
cases with the toy lamp among them, section 9 the crate, and S21 the
scenario.

`weir-carries-the-release-chains-parts`'s other change,
a-statechart-kind-is-read-in-weirs-kind-format, makes `weir-engine` and
puts a statechart's types in it, the types `read-kinds` reads a
statechart kind into. It leaves the tick, its tests, its toy statechart
and the `tick` cases to this change. Without them nothing in the family
can run a statechart kind, and the pack counts no clause of A.13. The
change was queued in afterthought/crew-state's `plan.rec`, at
agentplot's main `fc738bab`.

## What Changes

- `weir-engine` gains the tick, moved from flywheel-engine onto the
  types the kind format reads into: one pure function over a
  statechart, the templates it runs, the proofs its reader names, one
  object's state and one snapshot, giving the transitions that fire,
  each with its effects, and the object's next state (96, 99). Every
  guard of a tick reads that one snapshot; in each region, innermost
  first, the first transition whose guard holds fires; an effect is
  planned only while its proof is absent.
- A state that runs a template starts it afresh each time it is
  entered, setting the earlier one aside under its attempt; what a state
  ran stays at its path when the state is left; the statechart that runs
  it reads it by its final states and moves it by an enter (97).
- A response is applied once, and a guard on a response binds the
  argument its answer gives (97).
- `start` gives an object's first state, `decisions` the decisions
  standing in a state, derived from the active states that carry one,
  and `names` the evidence, effects and templates a statechart names.
- The engine holds no store, reads no clock and serves no evidence of
  its own: the snapshot is its caller's reading, with the time it was
  read, its evidence by name and by region, the responses addressed to
  the object and the states of its owner and children. Numbering
  decisions, addressing responses and performing effects are the
  caller's (98).
- The engine's tests run a toy statechart, a lamp and the bulb it runs,
  that names no reader's atom, and a test holds that the engine's code
  names no reader's object and none of its toy's atoms (98).
- The pack gains its eighth shape, `tick`, its cases read and run in
  `weir-pack` against the engine, S21 among them, each ticked twice. The
  index takes A.13, 96 to 98, and 99 under B.4, and the check counts
  A.13.
- core.md gains the engine's rule, and core.35 counts A.13 and names
  the `tick` shape.
- The README's row for `weir-engine` names the tick.

## Capabilities

### New Capabilities

- `statechart-engine`: the tick, an object's first state, the decisions
  standing and the names a statechart reads and runs, pure over their
  inputs, naming no reader's object or atom.
- `tick-cases`: the pack's `tick` cases, read and run against the
  engine, the toy lamp's among them.

### Modified Capabilities

- `conformance-pack`: the pack holds eight shapes, `tick` among them;
  the index holds A.13 and 99; the check counts A.13.

## Impact

- `crates/weir-engine`: the tick, `start`, `decisions` and `names`, with
  `ObjectState`, `Snapshot`, `Tick` and their parts; depends on the
  workspace's `time`. The types the kind format put there are unchanged.
- `crates/weir-pack`: `Shape::Tick`, the tick cases read and run;
  depends on `weir-kinds` and `weir-engine`, and its moon project on
  `kinds` and `engine`.
- `pack/tick/`: fifteen cases; `pack/clauses.toml`: A.13 holds 96 to
  98, and B.4 gains 99.
- `docs/architecture/core.md`: the engine's rule, and core.35.
- README and the comment over `Cargo.toml`'s members.
- No record, layout, shared-branch write or kind format changes.
- The change builds on a-statechart-kind-is-read-in-weirs-kind-format:
  its first task carries that change from the bolt's branch, where it
  lands first.
- flume takes the engine when it moves its pin of weir past this change,
  in a change of its own.
