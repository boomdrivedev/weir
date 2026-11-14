# Proposal

## Why

flume's record 0069, accepted by the user on 2026-11-14, lets a kind say
which of its stages hibernate. A stage that hibernates has its agent let
go once idle, and its session resumed later. Some kinds allow it and
some do not, in the user's words: "certain flume kinds will allow
hibernation and resume while others won't." The statement at blueprints
30dce35, on the branch `design/a-stage-declares-it-hibernates` until the
user pushes it, writes this into weir's kind format:

- 53 lets a simple kind's stage declare whether it hibernates, a bool.
- C.4 spells it `hibernate = true` on a stage. A field left out is
  returned left out, and what it means is the reader's (53, 58).

flume reads its kinds through `weir-kinds` alone (core.30). Its change
`a-stage-whose-kind-hibernates-it-sleeps-and-resumes` waits on this one
at its task 8.1, which maps `Stage`'s `hibernate` onto its own stage.
So `weir-kinds` reads the field before flume can.

## What Changes

- `read-kinds` reads a stage's `hibernate` into the stage: `true` or
  `false` as written, left out when the stage leaves it out, and never
  among the stage's sections (53, 56, 58, C.4).
- A `hibernate` that is not a bool is refused naming the stage's
  `hibernate` field (57).
- Any stage may declare it: a conversation or a stage with a
  deliverable, one that waits or repeats or not. What hibernating means,
  and what a stage that leaves it out does, are the reader's (58;
  core.32).
- `hibernate` on a kind's own table is no field the format names, and is
  returned among the kind's sections (56).
- The pack's kinds cases say a stage's `hibernate` where it declares
  one. A new case reads a stage that hibernates, one that does not, and
  one that leaves it out (53, 58; C.7).
- core.31 and core.32, with their details, say it as a rule, amended on
  this branch from 53 and C.4 at 30dce35 and flume's record 0069.

## Capabilities

### New Capabilities

None.

### Modified Capabilities

- `kinds`: a stage reads whether it hibernates, a bool returned as
  written and left out when left out, refused naming the field when it
  is not a bool. A declaration's kinds case gives a stage's `hibernate`,
  and the pack holds a case of it.

## Impact

- `crates/weir-kinds`: `Stage` gains `hibernate: Option<bool>`, read by
  `read_stage` (`src/kinds.rs`). `STAGE_FIELDS` names `hibernate`. A
  reading of an optional flag joins `src/fields.rs`. The crate's docs
  and `Stage`'s say what the field holds.
- `crates/weir-kinds/tests`: `read.rs` and `refuse.rs` gain the
  scenarios' tests, with fixtures of their own. `pack.rs`'s
  `ExpectStage` reads `hibernate`, and `kind_differences` holds it
  against the stage read.
- `pack/kinds/a-stage-that-hibernates`: a new case citing 53 and 58.
- `docs/architecture/core.md`: core.31 and core.32, with their details,
  amended from 53 and C.4 at 30dce35 and flume's record 0069.
- `crates/weir`: unchanged. `weir check` lists a stage's name and end,
  and no other setting of it.
- `pack/clauses.toml` is unchanged. 30dce35 amends 53 and C.4, and
  adds, removes or renumbers no clause of weir's.
- flume moves its pin of `weir-kinds` to the revision of weir's main
  that lands this change, at its task 8.1.
