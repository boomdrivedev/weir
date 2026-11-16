# Verify

The change weir-carries-the-statechart-engine, on
`change/weir-carries-the-statechart-engine` at 3782e46, holds: the whole
checks pass, all 26 tasks are ticked, every requirement and scenario of
its three delta specs is met by the code and a test, and archived it
leaves every one of weir's specs passing `root:specs`. It is archived
with this report.

## The whole checks

`devenv shell -- check-fast` (`moon run :#check-fast`), run once at
3782e46, exit 0: 50 tasks completed, none failed. `root:format` and
`root:toolchain` ran; the other 48 replayed from moon's cache, their
inputs unchanged since they last passed (`.moon/workspace.yml`): the
`format`, `lint`, `typecheck` and `test` tasks of every crate's project,
`adapters`, `bounded`, `branch`, `cadence`, `capture`, `delivery`,
`engine`, `kinds`, `pack`, `signals` and `weir`, and `root:hooks`,
`root:openspec-claude`, `root:prose` and `root:specs`. `engine:test`
passed 57 tests and `pack:test` 83. `root:specs` passed 28 of 28.
`root:prose` gave no error or warning and one suggestion, on README line
33, the `weir-engine` row design 19 gives word for word.

`root:specs` validates `openspec/specs` alone. The change itself,
`openspec validate weir-carries-the-statechart-engine --strict
--no-interactive`, is valid.

## The archive

In a copy of `openspec/`, `openspec archive
weir-carries-the-statechart-engine -y` applied the deltas, 24 added and
5 modified: `conformance-pack` updated, `statechart-engine` and
`tick-cases` created, each new spec opening with the `## Purpose` its
delta gives. `openspec validate --specs --strict --no-interactive`,
`root:specs`'s command, then passed 30 of 30.

## Tasks

All 26 tasks of `tasks.md` are ticked, 1.1 and 1.2, 2.1 and 2.2, 3.1 to
3.7, 4.1, 5.1 to 5.5, 6.1 to 6.7, 7.1 and 8.1. Each test a task names
is in the tree under that name and passed. Every scenario of the three
deltas has a test of its name in snake case but three, which their
tasks hold by other names: "The index read" by `clauses.rs`'s
`the_index_holds_every_clause_under_its_section` (6.5), "The sections
the check counts" by `the_check_counts_a1_to_a10_a13_a14_and_71` (6.5),
and "An answer matched by its pattern" by `src/eval.rs`'s matcher unit
tests (3.1), `<text>` among them. 1.1's bolt branch,
`origin/bolt/weir-carries-the-release-chains-parts`, is merged into
this one.

## statechart-engine

The tests are `crates/weir-engine/tests/`'s, by file, but where named.

| Requirement | Scenario | Where it holds |
|---|---|---|
| The tick answers from its inputs alone | The lamp switched on (S21); The lamp on with its proof present (S21); A tick given twice; Ticking changes nothing | `tick.rs`'s four tests of those names; `ticking_changes_nothing` compares the chart, templates, proofs, state and snapshot with copies taken before |
| In each region, innermost first, the first transition whose guard holds fires | The first transition whose guard holds; Innermost first, at most one per region | `regions.rs`'s `the_first_transition_whose_guard_holds` and `innermost_first_at_most_one_per_region`, `power.on.level` then `power` |
| Every guard of a tick reads one snapshot and the state as given | A sibling sees a move at the next tick | `regions.rs`'s test of that name, `shade` moving on the second tick |
| A transition to its own state with nothing to do fires nothing | A transition to its own state with nothing to do | `regions.rs`'s test of that name, the next state the state given |
| Firing plans its effects, each only while its proof is absent | Exit, transition and entry effects in order; An effect whose proof is present; An effect with no proof named; A transition to its own state runs its own effects alone | `effects.rs`'s four tests of those names, each effect's phase and region compared |
| Firing records the move | Firing records the move; A transition to its own state keeps its time | `effects.rs`'s two tests of those names |
| A state that runs a statechart starts it afresh each time it is entered | A template started afresh each time its state is entered; Leaving a state keeps what it ran | `templates.rs`'s two tests of those names, the first setting aside `power.on#1` and then `power.on#2` |
| A final guard reads a final state of what a state runs | A final state of what a state runs; A state of that name that is not final | `templates.rs`'s two tests of those names |
| An enter moves what a state ran | An enter moves what a state ran | `templates.rs`'s test of that name, the enter keyed by `bulb`, `glow` and `on` in turn |
| A response is applied once | A response applied once; A response already applied; Two regions take one response in one tick; The evidence branch of any takes no response | `responses.rs`'s four tests of those names |
| A response's pattern binds the argument its answer gives | A response binds its argument; An answer matched by its pattern | `responses.rs`'s `a_response_binds_its_argument`; `src/eval.rs`'s unit tests `bare_word_matches_exactly`, `word_with_argument_binds_the_rest`, `word_with_colon_binds_the_text_after_it` and `an_argument_before_a_tail` |
| A guard reads its evidence from the snapshot alone | An evidence read or not; Evidence read at its region; No evidence of the engine's own | `guards.rs`'s three tests of those names, the region's path, the nearest above it and never one beside it |
| Values compare by their type, their number and their instant | Is and in; Values in order; Against another evidence; Values of two types | `guards.rs`'s four tests of those names |
| An age is measured to the snapshot's time | A guard by age | `guards.rs`'s `a_guard_by_age`, at ten minutes and eleven, a datetime with and without its offset, no time and no evidence |
| A guard reads another region, its owner and its children as given | A sibling region; The owner and the children; All, any, not and always | `guards.rs`'s three tests of those names |
| An object starts in its initial states | A lamp starts off; A state that runs a template starts it | `start.rs`'s two tests of those names |
| The engine refuses a state or a template it cannot read | A region the statechart does not hold; A state its region does not hold; A template no one gave; A statechart that runs itself; A region a state and its template both hold | `refusals.rs`'s five tests of those names, each refusal's path and reason compared; `a_transition_to_a_state_its_region_does_not_hold` holds design 14's sixth row |
| A decision stands while its state is active | A decision stands while its state is active; A decision whose state was left | `decisions.rs`'s two tests of those names |
| The engine lists the names a statechart reads and runs | The names the lamp reads and runs | `listing.rs`'s test of that name, with the bulb's names and a comparison's other evidence |
| The engine names no reader's object or atom | The code names no tool of the family; The engine names no reader's object; The engine names none of its toy's atoms; A name put back is found as a whole word | `names.rs`'s `the_code_names_no_tool_of_the_family`; `boundary.rs`'s three tests of those names, the toy's atoms read through `names` over `lamp()` and `bulb()` |

## tick-cases

The tests are `crates/weir-pack/tests/tick_cases.rs`'s but where named.

| Requirement | Scenario | Where it holds |
|---|---|---|
| A tick case is read whole | A misspelt field | `src/tick.rs`'s `read_tick_case`; `a_misspelt_field`, a misspelt field of the snapshot and of the case's top, and a file in the folder beside `case.toml` |
| A tick case's statechart is read by the kind format | A chart that is no statechart; A declaration the kind format refuses | the two tests of those names, the second naming `kinds.lamp.regions.power.states.off.transitions[0].to` |
| A tick case is ticked twice and compared | A case whose next state differs; A case whose effects differ; A case read from its folder | `src/tick.rs`'s `run_tick_case`, ticking clones twice and naming each changed input and unequal ticks; the three tests of those names |
| The pack holds the tick cases | The cases named; The tick cases; The tick cases cite 96 to 99 | the fifteen folders under `pack/tick/`, each citing as design 17's table cites; `the_cases_named`, each case's atoms the lamp's or the bulb's, and `the_tick_cases`; `tests/the_pack.rs`'s `the_tick_cases_cite_96_to_99` |

## conformance-pack

The tests are `crates/weir-pack/tests/`'s.

| Requirement | Scenario | Where it holds |
|---|---|---|
| Every case lies in one folder under its shape (modified) | A folder of no shape; A case that cites nothing; A clause with a letter; A delivery case is a case of the pack; A tick case is a case of the pack | `src/pack.rs`'s `Shape::Tick` between `Kinds` and `Delivery`, `Shape::ALL` eight; `check.rs`'s five tests of those names |
| The pack keeps an index of the statement's clauses (modified) | The index read | `pack/clauses.toml`, read at 006fc34, A.13 holding 96 to 98 after A.12 and B.4 holding 99; `clauses.rs`'s `the_index_holds_every_clause_under_its_section` |
| The pack's check fails on a counted clause no case cites (modified) | A clause of Part A cited by no case (S16); A clause of A.14 cited by no case; A clause of A.13 cited by no case; A clause the check does not count; A cadence or branch case counts as any other; The sections the check counts | `src/clauses.rs`'s `COUNTED_SECTIONS` with A.13; `check.rs`'s five tests of those names; `clauses.rs`'s `the_check_counts_a1_to_a10_a13_a14_and_71` |
| weir's pack passes its own check (modified) | The repository's pack | `the_pack.rs`'s `the_repositorys_pack`, a case of each of the eight shapes read |
| The index holds numbers alone and follows the statement (modified) | The index read | `pack/clauses.toml` holds numbers and sections only; `clauses.rs`'s `the_index_holds_every_clause_under_its_section` |

## The rules and the docs

- core.55 stands after core.54 under "Kinds", its details after
  core.54's, as design 16 gives them; core.35's rule and details count
  A.13, name the `tick` shape and rule out a tick case run against any
  engine but weir's, its source adding 96 to 99, S21 and flume's record
  0029.
- core.1 and core.55: `weir-engine`'s calls are functions of their
  arguments; nothing under its `src` reads a clock, a file or the
  environment, `tick` taking its time from the snapshot and `start`
  from its caller.
- core.2: no file under `crates/weir-engine/src` names a tool of the
  family, a reader's object or the toy's atoms (`names.rs`,
  `boundary.rs`).
- core.30 and core.54: a tick case's statechart is read by
  `weir_kinds::read_kinds` alone.
- core.39: `pack`'s moon project depends on `kinds` and `engine` and
  reads their files.
- The README's `weir-engine` row and the comment over `Cargo.toml`'s
  `members` give design 19's words; `weir-engine`'s and `weir-pack`'s
  descriptions name the tick, citing 94 to 99 and 96 to 99.

## Findings

None. The change archives with `root:specs` passing.
