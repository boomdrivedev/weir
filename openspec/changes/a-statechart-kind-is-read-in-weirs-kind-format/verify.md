# Verify

The change a-statechart-kind-is-read-in-weirs-kind-format, on
`change/a-statechart-kind-is-read-in-weirs-kind-format` at 64c5215,
falls short, and is not archived: the whole checks pass, every task is
ticked, and every requirement and scenario of its four delta specs is
met by the code and a test of the scenario's name, but its two modified
requirements, merged into `openspec/specs/` as `openspec archive` writes
them, fail `root:specs` (see Findings).

## The whole checks

`devenv shell -- check-fast` (`moon run :#check-fast`), run once at
64c5215, exit 0: 50 tasks completed, none failed, one of them from
moon's cache. Ran and passed: `root:format`, `root:hooks`,
`root:openspec-claude`, `root:specs` (`openspec validate --strict`, 27
items passed), `root:toolchain`, and `format`, `lint`, `typecheck` and
`test` of every crate's project, `adapters`, `bounded`, `branch`,
`cadence`, `capture`, `delivery`, `engine`, `kinds`, `pack`, `signals`
and `weir`. Taken from the cache, its inputs unchanged: `root:prose`.

## Tasks

All 16 tasks of `tasks.md` are ticked, 1.1 to 1.3, 2.1 to 2.7, 3.1, 4.1,
5.1 to 5.3 and 6.1, and each test a task names is in the tree under that
name and passed in the run above.

## statechart-kinds

The tests are `crates/weir-kinds/tests/statechart.rs`'s but where named.

| Requirement | Scenario | Where it holds |
|---|---|---|
| A kind's table holding regions is a statechart kind | A statechart kind beside a simple kind; read-kinds gives the engine's types | `src/statechart.rs`'s `read_statechart_kind`, called from `src/kinds.rs`'s walk; `a_statechart_kind_beside_a_simple_kind` over `tests/fixtures/persona.toml`, read twice and equal, which is C.4's `persona` as blueprints 3a5c19d writes it; `read_kinds_gives_the_engines_types` names `weir_kinds::weir_engine::Statechart` |
| A table holding both stages and regions is refused | A table holding both stages and regions | `a_table_holding_both_stages_and_regions`: beside `bump` and `persona`, `both`'s failing `version`, `title` and region are not named, only `kinds.both.regions` |
| A table holding agents and regions is a statechart kind | Agents beside regions | `agents_beside_regions`: no settings, `agents` among the kind's sections |
| A simple kind reads as it did | A simple kind's title and label are sections | `a_simple_kinds_title_and_label_are_sections`; every earlier kinds test passes unchanged |
| A statechart kind reads its own fields | A statechart kind reads with every field; A statechart kind's fields left out | the two tests of those names |
| A statechart kind holds its title | A statechart kind with no title | `a_statechart_kind_with_no_title`, both kinds' `title` named |
| Only a statechart kind that lands names its landing fix | A landing fix on a statechart kind that does not land | the test of that name, `landing_fix` then `max_landing_fixes`, each `does not land` |
| A statechart's regions and states read as written | A statechart reads as written | `a_statechart_reads_as_written` over `persona.toml` |
| A statechart holds regions and each region states | A statechart kind with no region; A region with no states | the two tests of those names |
| A region's initial state is one it holds | An initial state the region does not hold | the test of that name, its reason holding `` `c` `` |
| A region's and a state's key is a name | A state named by a key that is not a name | the test of that name; the state's failing `final` and `regions` are not named |
| A state's own regions are checked as any region | A nested region is checked as any | the test of that name, reading the nested region and then refusing its `initial` |
| A transition goes to a state its region holds | A transition to a state its region does not hold; A transition with no guard or target | the two tests of those names, the first S20's path |
| A guard is one of the forms C.4 names | Every form of guard reads | `every_form_of_guard_reads`, all twenty guards read into the engine's `Guard`, `older` two hours; `src/guard.rs` |
| A guard of no form is refused naming its path | A guard of a form the format does not name | the test of that name, the seventh at `when.any[1]` |
| An older is a duration | An older that is no duration | the test of that name; `older` read through `quiet::parse` in `src/guard.rs` |
| Effects and decisions read with their atoms and answers | Effects and decisions read as written | the test of that name, `note` and `doc` among sections, `notify`'s args empty |
| An effect names its atom and a decision its kind | An effect with no atom and a decision with no kind | the test of that name |
| A state's params read the stage template's parameters | A state's params read as the stage template's | the test of that name, `persona`'s `stages.build` and `stages.test` and `mover`'s `main.move`, its `command` as written |
| A join is all, any or a count | A join of all, any or a count | the test of that name, `count:0`, `count:03` and `most` refused, `Join`'s `Display` checked |
| A state's deliverables are of the format's spellings | A deliverable in params not of the forms | the test of that name, several and none read |
| A send-back names a state of its region | A send-back to no state of its region | the test of that name |
| A state's agents are entries or a rule | An agents rule of the wrong shape | the test of that name |
| Which templates and atoms there are is the reader's | Any template or atom name reads | `any_template_or_atom_name_reads` |
| A statechart kind is checked whole | A statechart kind checked whole | the test of that name, the six fields in the order the scenario gives, refused whole, so the simple kind `tidy` that reads is not returned either |
| A declaration's kinds case expects its statechart kinds | A case whose statechart differs is named | `tests/pack.rs`'s `statechart_differences` and `regions_differences`; `a_case_whose_statechart_differs_is_named` names the case, `persona`, the state and both target lists, an unexpected kind, and a misspelt field refused |
| The pack's kinds cases hold S19 and S20 | The cases of S19 and S20; The cases cite 94 and 95 | the five cases under `pack/kinds/`, design 13's table and citations; `tests/pack.rs`'s two tests of those names and `every_kinds_case_is_answered_as_it_says` |
| The engine's crate names no tool | The engine's code names no tool of the family | `crates/weir-engine/tests/names.rs`, both tests |

## kinds

| Requirement | Scenario | Where it holds |
|---|---|---|
| weir check reads a declaration (modified) | A statechart kind is listed | `crates/weir/src/main.rs`'s `listing` and `statechart_listing`, both formats in name order; `crates/weir/tests/check.rs`'s `a_statechart_kind_is_listed`, the exact output the scenario gives |

The modified requirement keeps the five scenarios the main spec holds,
each still passing in `check.rs`, so archiving drops none.

## kind-packages

| Requirement | Scenario | Where it holds |
|---|---|---|
| A package of a statechart kind is refused naming its kind | A package of a statechart kind; A simple kind and a statechart kind in one package | `src/package.rs` counts statechart kinds among the package's kinds; `tests/package.rs`'s two tests of those names |

## conformance-pack

| Requirement | Scenario | Where it holds |
|---|---|---|
| The pack keeps an index of the statement's clauses (modified) | The index read | `pack/clauses.toml` holds 94 and 95 under A.8 and none of 96 to 99, still read at 006fc34; `crates/weir-pack/tests/clauses.rs` reads A.8 with them, 110 numbers in all, and counts 94 and 95 |

The modified requirement keeps its one scenario.

## The rules

- core.1: `weir-engine` is plain data with public fields; it reads no
  text, keeps no state and starts nothing.
- core.2: `weir-engine`'s `names` test passes; the crates' fixtures name
  `bump` and `persona`, no tool.
- core.30: `read-kinds` is the one reader of a statechart kind, into the
  types `weir-engine` defines, which `weir-kinds` re-exports.
- core.32 and core.54: no `machine`, `ev`, `do`, decision `kind`, `bump`
  or `response` is checked against a list, and a state's five stage
  parameters are read wherever `params` is given.
- core.39 and core.40: `engine` is a moon project in
  `.moon/workspace.yml`, its four checks one task each, tagged
  `check-fast`, its inputs as weir-cadence's; `kinds` and `weir` depend
  on it and read its files.
- core.43: a package of a statechart kind is refused naming it, which
  design 12 records as this change's departure from 89 and 92, leaving a
  package of either format to a change of the plan's.

## Findings

- The archive would fail the gate. `openspec archive` applied the
  deltas cleanly (+29, ~2, -0), and `openspec validate --specs
  --strict`, `root:specs`'s command, then failed 2 of 28 specs, each
  for a warning strict mode refuses, "Requirement text is very long
  (>500 characters)":
  - `kinds`, "weir check reads a declaration": 608 characters as this
    change's delta modifies it, under 500 in the main spec now.
  - `conformance-pack`, "The pack keeps an index of the statement's
    clauses": 537 characters as this change's delta modifies it, under
    500 in the main spec now.

  The whole checks above pass because `root:specs` reads
  `openspec/specs/` alone, and the change itself is valid under
  `openspec validate a-statechart-kind-is-read-in-weirs-kind-format
  --strict`, which raises no length warning on a delta. Each of the two requirements
  wants its text under 500 characters in the change's delta, in the
  delta's own words or split into requirements that each state one
  behavior, its scenarios kept; then `openspec archive` leaves
  `openspec validate --specs --strict` passing. The archive was undone,
  so nothing of it is on the branch.
