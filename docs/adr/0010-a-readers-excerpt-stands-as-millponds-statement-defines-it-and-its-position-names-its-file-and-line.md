---
number: 10
title: A reader's excerpt stands as millpond's statement defines it, and its position names its file and line as `<path>:<line>`
status: proposed
date: 2026-11-15
decision-makers:
- boomdrive-design
- the construct stage of change/a-reading-stands-as-millponds-statement-defines-it
---

# A reader's excerpt stands as millpond's statement defines it, and its position names its file and line as `<path>:<line>`

Status: proposed on 2026-11-15. The direction is boomdrive-design's ruling, written into boomdrivedev/millpond's core.1 and core.21 on its branch `design/millponds-constitution` at `75b0cea`: the verbatim check is weir's, one check in `weir-signals` to millpond's clause 32, and millpond keeps none of its own. The position's spelling, which line endings are evened, where on its line an excerpt may begin and what a refusal names are the construct stage's own choices. Each is cheap to change while no reader writes a position, and none is the user's.

Sources: the brief of the change `a-reading-stands-as-millponds-statement-defines-it`; boomdrivedev/blueprints `design/millpond/requirements.md` 32 and 107, and `design/weir/requirements.md` 26, 37 and C.2, at blueprints `3a5c19d`; the millpond model `design/millpond/models/reconstruction-1.md`, its §3.10 and its question Q10, there; boomdrivedev/millpond's core.1, core.21 and record 0002, at `75b0cea`; `crates/weir-signals/src/write.rs` `read_capture` and `stands_in`, and `crates/weir-capture/src/take.rs` `RawFile`, at `5e3f128`; rules core.16, core.17 and core.37.

## Context and problem statement

weir-signals takes a reader's signal when its excerpt's bytes stand anywhere in any raw file the writer hands in (26; core.16). millpond's statement defines the check more narrowly, and in one respect more widely (32): an excerpt stands when it is one unbroken run of one raw file's text, line endings evened, beginning at the file and the line its position names. millpond takes every check through weir's crates and keeps no twin of one (its record 0002 and core.1), and Boomdrive is weir's other host. So the two refuse the same reading only when weir's one check is 32's. weir's 26 holds under it: an excerpt 32 takes is its file's words as they stand there, in their lines, whatever ends each line.

A signal's `position` is free text today, `release notes` or `the transcript, line 2` (C.2). No statement spells how a position names a file and a line. Four questions are left:

1. How is a position spelled?
2. Which line endings are evened?
3. Where on its line may an excerpt begin?
4. What does a refusal name when a signal does not stand?

## Considered options

For the spelling:

1. **`<path>:<line>`**, the path as the writer hands the raw file in, the line a whole number from 1, read after the last `:` (chosen).
2. **Two fields of `weir-signal/1`, `file` and `line`.** A field the format adds is the statement's C.2 to change, and a reader's record would differ from an own signal's and an imported one's.
3. **`<path>#L<line>`**, a forge's link form. `#` already parts a signal's key from its number.
4. **A range, `<path>:<from>-<to>`.** 32 names the line an excerpt begins on, and nothing reads where it ends.

For line endings:

1. **`\r\n` evened to `\n`, in the raw text and the excerpt both, before lines are counted and the excerpt is looked for** (chosen).
2. **A lone `\r` evened too.** A log that redraws a line with `\r` would number its lines unlike every tool that splits on `\n`, and unlike the reader counting them.

For where it begins:

1. **Anywhere on the named line**: the excerpt's first byte lies at or after the line's first byte and before the next line's (chosen). A speaker's words quoted from inside a transcript line stand.
2. **At the line's first byte.** A reader would quote a line's start, a speaker's name and all, to quote the words inside it.

For a refusal:

1. **The first thing wrong, once per signal**: the position's spelling, its file, its line, then the excerpt, each naming its field (chosen).
2. **Every field wrong.** A position that names no file leaves nothing to look for the excerpt in, so a second name would say nothing more.

## Decision outcome

- **One check, in weir-signals, to 32.** A reader's signal is taken only when its position names one raw file the writer hands in and a line of it, and its excerpt is one unbroken run of that file's text beginning on that line, both with `\r\n` evened to `\n`. Bytes are compared as they are: no whitespace or letter's case is evened, and nothing is decoded. The excerpt is written as the reader handed it.
- **The spelling.** A position is `<path>:<line>`. `<path>`, everything before the last `:`, is a raw file's path exactly as the writer hands it, the path the offer's `raw` named. `<line>` is a whole number of at least 1, with no sign or leading zero. A file of `m` lines, counted after evening, has lines 1 to `m`, its last line with no line ending among them.
- **What a refusal names.** A signal not taken is named once, with its field, for the first of: its position spelled otherwise (`position`); its path none of the raw files handed in (`position`); its line past the file's last (`position`); its excerpt not standing there (`excerpt`). A capture with nothing banked hands in no raw file, so only a reading of none is taken for it (millpond's 107).
- **What it does not touch.** A capture's own signal and an imported signal are not looked for in raw material (26, 37). Their positions stay free text, and so does every position already written: nothing reads a written signal's position.

### Consequences

- millpond and Boomdrive refuse the same reading, through `read_capture`, and neither keeps a check of its own.
- A reader writes each position as `<path>:<line>`. A reading whose positions are free text is refused, naming each signal's `position`.
- An excerpt that stands in its file but begins on another line, or in another file than its position names, or that runs across two files, is refused. Today each is taken.
- An excerpt quoted from a file with `\r\n` line endings stands, and is written with the line endings the reader gave.
- `read_capture` keeps its signature, so no host's call changes.
- The statement's C.2 gives `position` no spelling, so the spelling is weir's own until blueprints writes it there.
- This record amends core.16. The change `a-reading-stands-as-millponds-statement-defines-it` builds it.
