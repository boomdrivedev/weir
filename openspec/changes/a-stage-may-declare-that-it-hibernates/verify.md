# Verify

The change a-stage-may-declare-that-it-hibernates, on
`change/a-stage-may-declare-that-it-hibernates` at 562e095, holds: the
whole checks pass, every task is ticked, and every requirement and
scenario of its delta spec is met by the code and a test of the
scenario's name.

## The whole checks

`devenv shell -- check-fast` (`moon run :#check-fast`), run once at
562e095: 46 tasks completed, none failed, 32 of them from moon's cache.
Ran and passed: `root:format`, `root:toolchain`, and `format`, `lint`,
`typecheck` and `test` of `kinds`, `pack` and `weir`. Taken from the
cache, their inputs unchanged: `root:hooks`, `root:openspec-claude`,
`root:prose`, `root:specs` (`openspec validate --strict`, 27 passed),
and the four checks of `adapters`, `bounded`, `branch`, `cadence`,
`capture`, `delivery` and `signals`.

## Tasks

All 8 tasks of `tasks.md` are ticked, 1.1 to 1.6, 2.1 and 2.2, and each
test a task names is in the tree under that name and passed in the run
above: `refuse`'s `every_other_shape_the_format_names_is_checked` and
`a_hibernate_of_the_wrong_shape`; `read`'s
`a_stage_reads_whether_it_hibernates`, `any_stage_may_hibernate` and
`a_hibernate_on_a_kinds_table_is_a_section`; `pack`'s
`a_case_whose_stage_differs_in_its_hibernate_is_named` and
`a_stage_that_hibernates_is_a_case_of_its_own`. 1.6's `kinds:lint`
passed, `missing_docs` denied.

## kinds

| Requirement | Scenario | Where it holds |
|---|---|---|
| A stage reads whether it hibernates | A stage reads whether it hibernates | `Stage::hibernate: Option<bool>`, read last in `read_stage` through `fields::flag`; `STAGE_FIELDS` names `hibernate`, so it is never a section; `tests/read.rs` with `hibernates.toml`, `Some(true)`, `Some(false)`, `None`, every stage's `sections` empty |
| Any stage may declare that it hibernates | Any stage may hibernate | `flag` is read on every stage, after `repeat`, against no other field; `tests/read.rs` with `hibernate-on-any-stage.toml`, a waiting conversation and a `tasks` stage repeating per task group both `Some(true)` |
| A hibernate on a kind's table is a section | A hibernate on a kind's table is a section | `KIND_FIELDS` and `DECLARING_FIELDS` do not name it; `tests/read.rs` with `hibernate-on-a-kind.toml`, the kind's `sections` equal `hibernate = true`, `draft`'s `hibernate` `None` |
| A stage's hibernate that is not a bool is refused | A hibernate of the wrong shape | `flag` refuses any other value naming the field, "must be true or false"; `boolean` is `flag(..).unwrap_or(false)`, so `wait`, `lands`, `starts_unready` and `transcripts` keep the same words; `tests/refuse.rs` with `hibernate-shapes.toml`, the three `kinds.talk.stages[n].hibernate` problems in order and `read` an `Err` |
| The pack's kinds cases hold a stage that hibernates | A stage that hibernates is a case of its own | `pack/kinds/a-stage-that-hibernates/case.toml`, as design 5 writes it, citing 53 and 58; `tests/pack.rs` reads its citations, the expected `Some(true)`, `Some(false)`, `None`, and no differences |
| A declaration's kinds case gives each kind's version, stages and landing fix (modified) | A kind that names its fix, and one with stages that may not; A case whose stage differs in its hibernate is named | `ExpectStage::hibernate: Option<bool>`; `kind_differences` holds four-part stages, name, end, `repeat` and `hibernate`; every earlier kinds case, none declaring `hibernate`, still passes in `pack:test`; `tests/pack.rs`'s `wrong-hibernate` case gives the one difference the scenario names |

The modified requirement keeps the one scenario the main spec holds and
adds one, so archiving drops none.

## The rules

- core.2: no fixture, test, doc or case of the change names a tool; the
  new case's kind is `talk`, and `weir`'s `names` test passes.
- core.30: one reader of kinds; `hibernate` on a kind's table is
  returned as a section, unread and unrefused.
- core.31: a `hibernate` that is not a bool is refused naming the field,
  and one left out is returned left out, as amended on this branch at
  7b83df5.
- core.32: weir judges `hibernate` against nothing else of the stage,
  and reads a left-out one as neither true nor false.
- core.35: the new case is one folder under `kinds` and cites 53 and
  58; `pack/clauses.toml` is unchanged, 30dce35 adding no clause.
- `crates/weir` is unchanged, as design 6 says: `weir check` lists no
  stage's `hibernate`.

## Findings

None. The statement is cited at blueprints 30dce35, on the branch
`design/a-stage-declares-it-hibernates`, unpushed, as the design's
risks say.
