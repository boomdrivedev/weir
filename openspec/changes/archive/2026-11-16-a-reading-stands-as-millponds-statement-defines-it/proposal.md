# Proposal

## Why

weir-signals takes a reader's signal when its excerpt's bytes stand
anywhere in any raw file the writer hands in. millpond's statement
defines the check otherwise (its 32): an excerpt stands when it is one
unbroken run of one raw file's text, line endings evened, beginning at
the file and the line its position names. boomdrive-design ruled, in
boomdrivedev/millpond's core.1 and core.21 on its branch
`design/millponds-constitution` at `75b0cea`, that the check is weir's,
one check in `weir-signals` to 32's definition, and that millpond keeps
none of its own. millpond and Boomdrive are weir's two hosts, so both
refuse the same reading only when weir's one check is 32's.

weir's 26 holds under it: an excerpt 32 takes stands verbatim in its
capture's raw material. A signal's `position` is free text today, and no
statement spells how it names a file and a line. Record 0010, proposed,
spells it `<path>:<line>`, evens `\r\n` to `\n`, lets an excerpt begin
anywhere on its line, and says what a refusal names. core.16 says so.

## What Changes

- `InboxAt::read_capture` takes a reader's signal only when its position
  names, as `<path>:<line>`, one raw file the writer hands in and a line
  of it, and its excerpt is one unbroken run of that file's text
  beginning on that line, `\r\n` evened to `\n` in both. No whitespace or
  letter's case is evened, and the excerpt is written as the reader
  handed it. The check is one function in a module of its own,
  `crates/weir-signals/src/stands.rs`, in place of `stands_in`.
- A signal not taken is named once, with its field: `position` where the
  position is spelled otherwise, names no raw file handed in, or names a
  line past the file's last; `excerpt` where the excerpt does not stand
  there. **BREAKING** for a reader that writes a position in free text, or
  an excerpt from another line or file than its position names: its
  reading is refused.
- `read_capture` keeps its signature, so no host's call changes. A
  capture's own signal and an imported signal are not looked for in raw
  material, and their positions stay free text.
- The weir-signals tests and the pack's reading cases spell each
  reader's position `<path>:<line>`. The pack gains two inbox cases: an
  excerpt standing in its file but on another line than its position
  names, and a position naming no file and line, each refused.

## Capabilities

### New Capabilities

None.

### Modified Capabilities

- `inbox-signals`: a reader's excerpt stands only as one unbroken run of
  one raw file's text, `\r\n` evened, beginning on the line of that file
  its position names as `<path>:<line>`, and a refusal names `position`
  or `excerpt` for the first that fails.
- `inbox-cases`: the pack holds a case of an excerpt on another line
  than its position names, and one of a position naming no file and
  line, each refused.

## Impact

- `crates/weir-signals`: `src/stands.rs`, new; `src/lib.rs` declares it;
  `src/write.rs`'s `read_capture` calls it and loses `stands_in`; the
  tests `signals.rs`, `records.rs`, `moves.rs`, `pack.rs` and
  `common/mod.rs` spell each reader's position `<path>:<line>`, and
  `signals.rs` gains a test per scenario.
- `pack/inbox/`: `a-reading-written-whole`, `a-reading-refused-whole`
  and `an-excerpt-standing-nowhere-refused` spell their positions
  `raw/notes.md:<line>`; `an-excerpt-on-another-line-refused` and
  `a-position-naming-no-file-and-line-refused` are added.
- No record's format, no layout, no crate's public surface, no moon
  project and no command line changes. Record 0010 and core.16 are on
  this branch.
- millpond and Boomdrive call `read_capture` as they would today, and
  their readers write positions as `<path>:<line>`.
