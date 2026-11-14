# Design

## Context

In `weir-kinds`, `read_stage` (`src/kinds.rs`) reads a stage's table into
`Stage`. `STAGE_FIELDS` names the twelve fields the format gives a
stage, and every other key is returned among `Stage::sections` (56).
`wait` reads through `fields::boolean`, false when left out and refused
naming the field with "must be true or false" when it is not a bool.
`quiet`, `on_fail`, `max_send_backs` and `repeat` are `Option`s, left out
when left out, since C.4 gives them no default (58). `read_stage` reads
`repeat` last.

The statement at blueprints 30dce35, on the branch
`design/a-stage-declares-it-hibernates`, amends 53 and C.4 from flume's
record 0069, accepted by the user on 2026-11-14:

- 53: a simple kind's stage may hold whether it hibernates, its agent
  let go once idle and its session resumed later, a bool.
- C.4: `hibernate = true` on a stage, "whether the stage hibernates, a
  bool; a field left out is returned left out, its meaning the reader's
  (53, 58)".

flume's change `a-stage-whose-kind-hibernates-it-sleeps-and-resumes`
reads the field at its task 8.1. It maps `true` to a stage that
hibernates, and `false` or left out to one that does not, never reading
`sections` for it.

Rules this design relies on, all in `docs/architecture/core.md`, the one
page whose area the change touches:

- core.2: no tool named. The fixtures, tests and docs of the crate name
  no tool. The new pack case's kind is `talk`.
- core.4: no agent started. weir hibernates and resumes nothing.
- core.30: one reader of kinds, and a section it does not know returned
  unread and unrefused.
- core.31: a declaration checked whole, a field of the wrong shape
  refused naming it, and a field with no default returned left out. It
  is amended on this branch from 53 and C.4 at 30dce35 to name
  `hibernate`.
- core.32: what a field means is the reader's. It is amended on this
  branch: a stage hibernated is the reader's rule, and `hibernate` is
  judged against nothing else of the stage.
- core.35: each case cites its clauses.
- core.37: a clause changed in the statement first.

The design departs from none.

## Goals / Non-Goals

**Goals:**

- A stage's `hibernate` read as written, left out when left out, and
  refused naming the field when it is not a bool.
- The pack's kinds cases hold the field, so a case says what a stage's
  `hibernate` reads as.

**Non-Goals:**

- What hibernating is, when a stage hibernates, how long its agent may
  be idle first, and how its session is resumed. Those are flume's
  (flume 212, 213; core.4, core.32).
- What a stage that leaves `hibernate` out does. That is the reader's
  (58).
- `hibernate` on a statechart kind. weir reads no statechart kind yet,
  and 53 gives the field to a simple kind's stage.
- `weir check`'s listing. It gives a stage's name and end, and no
  setting of the stage (decision 6).

## Decisions

### 1. `Stage::hibernate` is an `Option<bool>`, left out when left out

`Stage` gains `hibernate: Option<bool>`: `Some(true)` or `Some(false)`
as written, `None` when the stage leaves it out. `STAGE_FIELDS` names
`hibernate`, so it is never among `Stage::sections`. C.4 gives it no
default: "a field left out is returned left out, its meaning the
reader's". flume reads a field left out as false, its own rule.

The way not taken is a `bool`, false when left out, as `wait` is. C.4
says the field left out is returned left out, as `quiet`, `on_fail` and
`max_send_backs` are, and a `bool` would give the format a default 58
leaves to the reader.

### 2. One reading of a flag that may be left out

`src/fields.rs` gains `flag(table, name, at, problems) -> Option<bool>`:
`None` when left out, the bool when it is one, and otherwise `None` with
the field refused, "must be true or false". `boolean` becomes
`flag(...).unwrap_or(false)`, so `starts_unready`, `lands`, `wait`, an
elaboration's `transcripts` and `hibernate` are refused in one place and
in the same words.

The way not taken is a `match` on `stage.get("hibernate")` in
`read_stage`, as `standing` has. It would spell "must be true or false"
a second time, where the words of a flag's refusal are one reading's.

### 3. Any stage may declare it, and weir judges it against nothing

`hibernate` is read on every stage, a conversation or a stage with a
deliverable, one that waits or repeats or not, and is refused only for
its shape. 53 gives it to a simple kind's stage, without restriction,
and what it means is the reader's (58; core.32). It is read last, after
`repeat`, so a stage's refusals name its `hibernate` after every other
field.

The way not taken is refusing it on a stage that is not a conversation,
since flume's standing roles hibernate conversations. flume hibernates a
stage whose kind hibernates it once its agent is idle, whatever the
stage delivers (flume 212). A rule tying the field to a stage's end
would be weir saying what hibernating is for.

### 4. On a kind's table it is a section

`KIND_FIELDS` and `DECLARING_FIELDS` do not name `hibernate`. On a
kind's own table it is returned among `Kind::sections`, as any field
the format does not name is (56). It is never read as a default for the
kind's stages. 53 and C.4 give it to a stage, and a kind's `agents`
table holds settings for agents, not for stages.

### 5. The pack's kinds cases say a stage's `hibernate`

`ExpectStage` in `crates/weir-kinds/tests/pack.rs` gains
`hibernate: Option<bool>`, left out where the case's stage leaves it
out. `kind_differences` holds it as part of each stage, with its name,
its end and its `repeat`. A stage the case expects without `hibernate`
is one read without it. So every case written before this change, none
of which declares `hibernate`, still holds.

A new case, `pack/kinds/a-stage-that-hibernates`, cites 53 and 58:

```toml
satisfies = [53, 58]
declaration = """
[kinds.talk]

[[kinds.talk.stages]]
name = "draft"
prompt = "prompts/talk-draft.md"
conversation = true
wait = true
hibernate = true

[[kinds.talk.stages]]
name = "code"
prompt = "prompts/talk-code.md"
deliverables = ["commits"]
hibernate = false

[[kinds.talk.stages]]
name = "review"
prompt = "prompts/talk-review.md"
conversation = true
"""

[expect.kinds.talk]
version = 1
stages = [
  { name = "draft", end = "conversation", hibernate = true },
  { name = "code", end = "commits", hibernate = false },
  { name = "review", end = "conversation" },
]
```

The refusal of a `hibernate` that is not a bool is the crate's test
alone. 57's refusal of a field of the wrong shape has cases of its own
already.

### 6. `weir check` lists nothing new

`weir check` lists each kind with its stages, each with its name and its
deliverable or that it is a conversation. It lists no `wait`, `quiet`,
`repeat` or `makes`, and no `hibernate` either. The way not taken is a
line such as `  draft: conversation, hibernates`, which would make
`hibernate` the one setting of a stage the listing shows.

### 7. The rules are amended on this branch

core.31 names `hibernate` among the fields returned as written and left
out when left out, and its details rule out a `hibernate` that is not a
bool accepted, a default given it and it returned among a stage's
sections. core.32 leaves a stage hibernated to its reader, and its
details rule out a `hibernate` left out read as true or false, or judged
against the stage's end, `wait` or `repeat`. Both cite 53 and C.4 as
amended at blueprints 30dce35 and flume's record 0069. They are
committed on this branch before the change.

`pack/clauses.toml` is unchanged. 30dce35 amends 53 and C.4 and adds,
removes or renumbers no clause of weir's.

## Risks / Trade-offs

- The statement's branch is unpushed, and its revision is cited as
  30dce35. If the user rewrites the branch before pushing it, the cited
  revision names nothing on blueprints' main. The rule and the change
  then cite the revision that lands, in a change of their own.
- flume's task 8.1 pins `weir-kinds` at the revision of weir's main
  that lands this change. Until it lands, flume's change stops short at
  that task.
