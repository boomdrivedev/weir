# Proposal

## Why

A kind is a simple kind or a statechart kind (51). flume's record
0028, the user's ruling of 2026-11-01, keeps simple kinds as they are
and brings statechart kinds beside them as a second format on one
branch, both read once by weir. The statement holds the second format:
94 says what a statechart kind's declaration holds, 95 how it is checked
whole, 81 that `read-kinds` gives a statechart kind with its statechart,
and C.4 spells it in TOML in the engine's own field names, read into
the engine's types.

`weir-kinds` reads simple kinds alone. A `[kinds.<kind>]` table holding
`regions` is refused today as a kind with its `stages` left out, so no
reader of the family can declare a statechart kind, and flume, which is
to run them on the engine, has nothing to read one with. The change was
queued in afterthought/crew-state's `plan.rec`, at agentplot's main
`4997273a`.

## What Changes

- A new crate, `weir-engine`, the engine's crate beside `weir-kinds`
  (section 9). It holds a statechart's types and nothing else yet: its
  regions, states, transitions, guards, effects and decisions, each
  field named as C.4 names it. It reads no text. The tick, its tests and
  its toy statechart come with the change that brings the engine's tick
  (96 to 99).
- `read-kinds` reads a kind's table holding `regions` as a statechart
  kind, into the engine's types, the kind's name and version its
  statechart's own (81, C.4). It reads the kind's title, label, branch,
  whether it lands, its landing fix and how many it runs (94). It reads
  each state's `params`, the stage template's agents, join, send-back
  and deliverables, by the state's path (94, C.4). `Declaration` gains
  `statecharts`, beside `kinds`. Simple kinds read, return and refuse as
  they did.
- A statechart kind is checked whole and refused naming every field
  that fails. Refused: a table holding both `stages` and `regions`, a
  region with no states or whose initial state it does not hold, a
  transition to a state its region does not hold, a guard of a form C.4
  does not name, a join not `all`, `any` or a count, a deliverable not
  of the format's spellings, and a field of the wrong shape (95). Which
  templates and atoms there are stays the reader's (58, 95, 98).
- `weir-kinds` re-exports `weir-engine`, as it re-exports `toml`, so a
  reader names a statechart's types at the version `read-kinds` reads
  into.
- `read-package` refuses a package whose `kind.toml` declares a
  statechart kind, naming the kind, since a package's one kind is a
  simple kind's type; a package of a statechart kind is the work of a
  change of its own.
- `weir check` lists a statechart kind beside the simple kinds, with
  its title and each region's states.
- The pack holds kinds cases of a statechart kind, S19 and S20 among
  them, citing 94 and 95. Its index takes 94 and 95 in under A.8, and
  96 to 99 stay out until the engine's tick comes.
- The README lists `weir-engine`, and its row for `weir-kinds` names
  both formats.

## Capabilities

### New Capabilities

- `statechart-kinds`: a statechart kind read by `read-kinds` into the
  engine's types beside the simple kinds, checked whole and refused
  naming the field, with the pack's cases of it.

### Modified Capabilities

- `kinds`: `weir check` lists a statechart kind beside the simple kinds.
- `kind-packages`: a package whose `kind.toml` declares a statechart
  kind is refused naming the kind.
- `conformance-pack`: the index holds 94 and 95 under A.8, and leaves
  out 96 to 99 alone.

## Impact

- `crates/weir-engine`: new, with its moon project `engine`, in the
  workspace's `members` and `.moon/workspace.yml`.
- `crates/weir-kinds`: depends on `weir-engine`; `Declaration` gains
  `statecharts`; `StatechartKind`, `Params`, `ParamAgents` and `Join`
  are new; `read_package` counts a statechart kind among a package's
  kinds and refuses it. Its moon project depends on `engine` and reads
  its files. `Kind`, `Stage` and every simple kind's field are as they
  were. A caller that builds a `Declaration` by its fields gives
  `statecharts` too.
- `crates/weir`: `weir check` lists statechart kinds; its moon project
  reads `weir-engine`'s files.
- `crates/weir-pack`: its index test reads 94 and 95 under A.8.
- `pack/kinds/`: five cases; `pack/clauses.toml`: A.8 gains 94 and 95.
- No record, layout or shared-branch write changes.
- flume reads a statechart kind when it moves its pin of weir past this
  change, in a change of its own; until it runs statechart kinds it
  refuses a launch of one (flume 4).
