# Tasks

One agent works these in order, reading this change (proposal, design,
specs) and building on the tasks before each. Record 0010 and core.16
are on this branch; the statement's 26 holds as written, read at
blueprints 3a5c19d. No source or test of a crate names a tool of the
family (core.2): the check's docs cite 26, core.16 and record 0010. A
test named for a scenario holds that scenario and keeps its name, in
snake case. Each check runs one crate's tests, one of its test binaries
or one test, through the crate's moon task, moon passing what follows
`--` to `cargo test`. The task groups are headings for reading. The
verify stage runs the repository's whole checks once every task is done.

## 1. Every reading a test hands in names its file and line

- [x] 1.1 In `crates/weir-signals/tests/signals.rs`, have `read_signal` take the position beside the excerpt, and have each test that calls it name `transcript.txt:<line>`, the line of `TRANSCRIPT` its excerpt begins on: `three_signals_read` lines 1, 2 and 3, `a_second_reading` line 2, `one_bad_signal_of_three` the line each excerpt is quoted from, line 3 for `we ask for nothing.`, `a_paraphrase` line 2 and `verbatim_across_lines` line 2 (design 6). True when `devenv shell -- moon run signals:test -- --test signals` passes.
- [x] 1.2 Have `crates/weir-signals/tests/records.rs`'s `read_signal` take the position, its three signals at `transcript.txt:1`, `:2` and `:3`; `crates/weir-signals/tests/moves.rs`'s `an_old_unmoved_signal` name `transcript.txt:1`; and `crates/weir-signals/tests/common/mod.rs`'s `with_signals` name `notes.md:<n>` for its `n`th line (design 6). True when `devenv shell -- moon run signals:test` passes.
- [ ] 1.3 Spell the pack's reading positions as design 6's table gives them: `pack/inbox/a-reading-written-whole/case.toml` and its `tip/inbox/captures/release/acme/1.1/signals.rec` at `raw/notes.md:3` and `raw/notes.md:4`, and `pack/inbox/a-reading-refused-whole/case.toml`'s first and third signals at `raw/notes.md:3` and `raw/notes.md:4` (design 6; core.35). True when `devenv shell -- moon run signals:test -- --test pack the_crate_passes_every_inbox_case` passes.

## 2. The one check

- [ ] 2.1 Add `crates/weir-signals/src/stands.rs`, declared in `lib.rs` as `mod stands;`, holding `stands(excerpt, position, raw)` and what it reads as design 1 to 5 give them: the position read at its last `:`, its file the first raw file of its path, both texts with `\r\n` evened to `\n`, its line counted from 1, and the excerpt one run beginning on that line, each failure with its field and its words as design 2 to 5 spell them. Its module doc says it is the one check of a reader's excerpt (26; core.16; record 0010). Have `read_capture` call it for each signal whose fields hold, in place of `stands_in`, which is removed, naming an `Err` by `NotTaken::field(&id, field, why)`, and have its doc say what is refused as design 1 says. Have `a_paraphrase` hold its scenario alone, its reading handed no raw file becoming `a_capture_with_nothing_banked`, and add `words_from_within_a_line` (design 1 to 6; 26, 28; core.4, core.16, core.17). True when `devenv shell -- moon run signals:test -- --test signals` passes `a_paraphrase`, `verbatim_across_lines`, `words_from_within_a_line` and `a_capture_with_nothing_banked`, holding the inbox-signals scenarios of those names, and every other test of the binary.
- [ ] 2.2 Add to `crates/weir-signals/tests/signals.rs` the tests of a position: `a_position_naming_no_file_and_line`, `a_line_spelled_otherwise`, `a_position_naming_a_file_not_handed_in`, `a_line_past_the_files_end`, `a_last_line_with_no_line_ending` and `a_path_holding_a_colon`, each refusal asserted by the signal it names, `position`, and a phrase of design 2, 3 or 4's words (design 2, 3, 4; core.16). True when `devenv shell -- moon run signals:test -- --test signals` passes those six, holding the inbox-signals scenarios of those names.
- [ ] 2.3 Add to `crates/weir-signals/tests/signals.rs` the tests of an excerpt: `line_endings_evened`, which also reads back the signal written and finds its excerpt as handed, `an_excerpt_on_another_line`, `an_excerpt_in_another_file`, `an_excerpt_across_two_files` and `whitespace_and_case_kept`, each refusal asserted by the signal it names, `excerpt`, and a phrase of design 5's words (design 5; core.16). True when `devenv shell -- moon run signals:test -- --test signals` passes those five, holding the inbox-signals scenarios of those names.
- [ ] 2.4 Have `pack/inbox/an-excerpt-standing-nowhere-refused/case.toml` name its signal at `raw/notes.md:3`, its refusal holding `as one run beginning on line 3` (design 5, 6; core.35). True when `devenv shell -- moon run signals:test -- --test pack the_crate_passes_every_inbox_case` passes.

## 3. The pack's cases of a reading's position

- [ ] 3.1 Add `pack/inbox/an-excerpt-on-another-line-refused` and `pack/inbox/a-position-naming-no-file-and-line-refused` as design 6 gives them, each a `case.toml` satisfying `[26]` with a comment saying what it holds, and `start/` and `raw/` as `an-excerpt-standing-nowhere-refused`'s, byte for byte. Add both names to `crates/weir-signals/tests/pack.rs`'s `the_cases_named_are_among_them` (design 6; 26; core.35). True when `devenv shell -- moon run signals:test -- --test pack` passes `the_crate_passes_every_inbox_case` and `the_cases_named_are_among_them`, holding the inbox-cases scenario "The cases named".
