# Verify

The change a-reading-stands-as-millponds-statement-defines-it, on
`change/a-reading-stands-as-millponds-statement-defines-it` at 2fcad3b,
holds: the whole checks pass, every task is ticked, and every
requirement and scenario of its two delta specs is met by the code and a
test of the scenario's name.

## The whole checks

`devenv shell -- check-fast` (`moon run :#check-fast`), run once at
2fcad3b: 46 tasks completed, none failed, 32 of them from moon's cache.
Ran and passed: `root:format`, `root:toolchain`, and `format`, `lint`,
`typecheck` and `test` of `pack`, `signals` and `weir`. Taken from the
cache, their inputs unchanged: `root:hooks`, `root:openspec-claude`,
`root:prose`, `root:specs` (`openspec validate --strict`), and the four
checks of `adapters`, `bounded`, `branch`, `cadence`, `capture`,
`delivery` and `kinds`.

## Tasks

All 9 tasks of `tasks.md` are ticked, 1.1 to 1.3, 2.1 to 2.4 and 3.1,
and each test a task names is in the tree under that name.

## inbox-signals

The one modified requirement, "A signal's excerpt stands verbatim in its
capture's raw material", is held by `crates/weir-signals/src/stands.rs`'s
`stands`, declared `mod stands;` in `lib.rs`, and called by
`read_capture` in `write.rs` for each signal whose fields hold, in place
of `stands_in`, which is gone. An `Err` is pushed as
`NotTaken::field(&id, field, why)`, so each signal not taken is named
once, in reading order. The position is read at its last `:`
(`read_position`), its file is the first raw file whose path equals it
byte for byte, both texts are evened `\r\n` to `\n` with a lone `\r`
kept (`evened`), lines are counted from 1 with a last line without an
ending among them (`line_span`), and the excerpt stands where the evened
file, from some offset of that line, begins with the evened excerpt.
The signal written keeps `signal.excerpt` as handed. The module's own
tests hold the position's spelling, the evening and the line count.

| Scenario | Test in `tests/signals.rs` | Field named |
|---|---|---|
| A paraphrase | `a_paraphrase` | `excerpt` |
| Verbatim across lines | `verbatim_across_lines` | taken, excerpt read back |
| Words from within a line | `words_from_within_a_line` | taken, excerpt read back |
| Line endings evened | `line_endings_evened` | taken, excerpt read back with `\n` |
| An excerpt on another line | `an_excerpt_on_another_line` | `excerpt` |
| An excerpt in another file | `an_excerpt_in_another_file` | `excerpt` |
| An excerpt across two files | `an_excerpt_across_two_files` | `excerpt` |
| Whitespace and case kept | `whitespace_and_case_kept`, both excerpts | `excerpt` |
| A position naming no file and line | `a_position_naming_no_file_and_line`, all three positions | `position` |
| A line spelled otherwise | `a_line_spelled_otherwise`, all four positions | `position` |
| A position naming a file not handed in | `a_position_naming_a_file_not_handed_in` | `position` |
| A capture with nothing banked | `a_capture_with_nothing_banked`, then a reading of none taken | `position` |
| A line past the file's end | `a_line_past_the_files_end` | `position` |
| A last line with no line ending | `a_last_line_with_no_line_ending` | taken |
| A path holding a colon | `a_path_holding_a_colon` | taken |

Each refusal is asserted by the signal it names, its field, and a phrase
of design 2 to 5's words. The readings of `three_signals_read`,
`a_second_reading` and `one_bad_signal_of_three`, of `records.rs`,
`moves.rs`'s `an_old_unmoved_signal` and `common/mod.rs`'s
`with_signals` spell their positions `<path>:<line>` as design 6 gives
them.

## inbox-cases

| Requirement | Scenario | Where it holds |
|---|---|---|
| The inbox cases hold the inbox's clauses (modified) | The cases named | `pack/inbox/an-excerpt-on-another-line-refused` and `pack/inbox/a-position-naming-no-file-and-line-refused`, each satisfying `[26]` with a comment saying what it holds, their `start/` and `raw/` identical to `an-excerpt-standing-nowhere-refused`'s; `tests/pack.rs`'s `the_cases_named_are_among_them` names both, and `the_crate_passes_every_inbox_case` passes every case |

`a-reading-written-whole` (in `case.toml` and `tip/`) and
`a-reading-refused-whole` name `raw/notes.md:3` and `raw/notes.md:4`,
and `an-excerpt-standing-nowhere-refused` names `raw/notes.md:3`, its
`reason_holds` `as one run beginning on line 3`, as design 6's table
gives them. The host cases' `signals.rec` files hold own and imported
signals alone, which the check does not read.

## The rules

- core.2: `stands.rs` cites 26, core.16 and record 0010 and names no
  tool, and the `names` tests pass.
- core.4: the check compares bytes and decodes nothing.
- core.16: one function, `stands`, crate-private, is the one check;
  record 0010 and core.16's amendment are on this branch at 60349cf.
- core.17: the reading is refused whole, naming each signal not taken.
- core.35: each new case is one folder under the inbox shape, citing 26.
- `read_capture` keeps its signature, so the `weir` and `pack` crates
  build and pass unchanged.

## Findings

None.
