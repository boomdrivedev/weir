# Design

## Context

`InboxAt::read_capture(key, signals, raw)` forms the write of a reader's
reading. It refuses the reading when the capture is not held, or has an
own signal or a reading already. Otherwise it reads each `ReadSignal` to
a `Signal` through `to_signal`, which names each field lacking or out of
shape. Each signal whose fields hold is then checked by `stands_in`:
its excerpt's bytes stand anywhere in any of `raw`, the raw files the
writer hands in, read from its bank, each a `RawFile` of the path the
offer's `raw` named and its bytes. A signal that fails is named with
`excerpt` and `stands in none of the capture's raw files, byte for byte`,
and the reading is refused whole, naming each signal not taken (28).

millpond's statement, its 32, defines the check otherwise: one unbroken
run of one raw file's text, line endings evened, beginning at the file
and the line its position names. boomdrive-design ruled, in millpond's
core.1 and core.21 at `75b0cea`, that the check is weir's, one check in
`weir-signals` to 32's definition. Record 0010, proposed, carries that
ruling. It spells a position `<path>:<line>`, evens `\r\n` to `\n`, lets
an excerpt begin anywhere on its line, and names the first thing wrong
once per signal. core.16 says so.

The statement's clauses this design satisfies: 26, which holds as
written, since an excerpt 32 takes stands verbatim in its capture's raw
material; 28, the reading refused whole naming each signal not taken;
37, an imported signal exempt. millpond's 32 and 107 are the definition
it builds.

Rules this design relies on, all in `docs/architecture/core.md`, the one
page whose area the change touches:

- core.2: neither the new module nor a test names a tool of the family.
  The module cites 26, core.16 and record 0010.
- core.4: the check compares bytes and reads no meaning from them.
- core.16: a reader's excerpt stands only as one unbroken run of one raw
  file's text, `\r\n` evened to `\n`, beginning on the line of that file
  its position names as `<path>:<line>`, checked by one function in
  `weir-signals`. Its reader's excerpt waits on the operator's yes to
  record 0010.
- core.17: a reading is written whole or refused whole, naming each
  signal not taken.
- core.21 and core.22: no written record is read again or changed, and
  `weir-signal/1` keeps its fields.
- core.35: each new pack case is one folder under the inbox shape and
  cites 26.
- core.37: the change cites 26, which holds as written.
- core.39: no moon project and no file type is added. A pack case's
  `raw/` is excluded from formatting by name in `treefmt.nix` already.

The design departs from no rule.

## Goals / Non-Goals

**Goals:**

- One check takes a reader's signal only as millpond's 32 defines it,
  the position spelled `<path>:<line>`.
- A refusal names the signal and `position` or `excerpt`, with words
  saying which file and line.
- `read_capture` keeps its signature, and every other operation of the
  records runs as before.

**Non-Goals:**

- A check a host calls apart from `read_capture`.
- A spelling for an own signal's or an imported signal's position, or a
  check of either against raw material (26, 37).
- Any check of a position already written.
- A rule's own signal (127), which weir's change of the policy's
  reading takes.
- The statement's C.2, which gives `position` no spelling. Blueprints
  holds it.

## Decisions

### 1. One check, in a module of its own

```rust
// crates/weir-signals/src/stands.rs
pub(crate) fn stands(
    excerpt: &str,
    position: &str,
    raw: &[RawFile],
) -> Result<(), (&'static str, String)>;
```

`Ok` where the excerpt stands where its position says. Otherwise `Err`
holds the field to name, `position` or `excerpt`, and why, for the first
of decisions 2 to 5 that fails. `lib.rs` declares the module as `mod
stands;`, so nothing outside the crate calls it. Its module doc says it
is the one check of a reader's excerpt, citing 26, core.16 and record
0010. `read_capture` calls it for each signal whose fields hold, in
place of `stands_in`, which is removed. It pushes
`NotTaken::field(&id, field, why)` for an `Err`, so each signal not
taken is named once, in reading order, as now. `read_capture`'s doc
says a signal is refused whose position names no raw file handed in and
line of it, as `<path>:<line>`, or whose excerpt does not stand there.

Not taken: a public check a host could call apart from a reading. It
would be a second entry to one check, and no host asks for one.

### 2. The position is `<path>:<line>`

The position is split at its last `:`. `<path>`, before it, is not
empty. `<line>`, after it, is one or more ASCII digits, the first not
`0`, that parse as a `usize`. Any other position fails:

- `position`: ``is `<position>`, which names no raw file and line as `<path>:<line>` ``

The split at the last `:` lets a path hold a `:`, as
`meeting:12/transcript.txt:2` does.

### 3. Its file is one of the raw files handed in

The file is the first of `raw` whose `path` equals `<path>`, byte for
byte, with no `./` stripped and no case evened. Where none does:

- `position`: ``names `<path>`, which is none of the capture's raw files``

A capture with nothing banked is handed no raw file, so each signal of
its reading fails here, and only a reading of none is taken for it
(millpond's 107).

### 4. Its line is a line of that file

The file's bytes and the excerpt are each evened: every `\r\n` becomes
`\n`, and a lone `\r` stays as it is. Line 1 of the evened file begins
at its first byte, and each later line just after a `\n` that is not the
file's last byte. So `a\nb\n` and `a\nb` both have two lines, and an
empty file has none. A line past the file's last fails:

- `position`: ``names line <line>, past the end of `<path>` ``

### 5. Its excerpt is one run beginning on that line

The excerpt stands when, for some offset at or after the line's first
byte and before the next line's first byte, or the file's end, the
evened file from that offset begins with the evened excerpt. So it may
begin anywhere on the line, and may run on over later lines of the same
file. Bytes are compared as they are, nothing decoded, no whitespace or
letter's case evened. Otherwise:

- `excerpt`: ``does not stand in `<path>` as one run beginning on line <line>, line endings evened``

The signal written holds the excerpt as the reader handed it, its
`\r\n` among it where it holds one. recfix takes a value holding `\r`.

Not taken: the first offset of the excerpt in the file, compared to the
line. An excerpt can stand twice in a file, and the one beginning on the
named line is the one meant.

### 6. The tests and the pack name their files and lines

Every reading a test of weir-signals hands `read_capture` spells each
position `<path>:<line>`, naming a raw file it hands in and the line its
excerpt begins on. `signals.rs`'s `read_signal` takes the position
beside the excerpt. `common/mod.rs`'s `with_signals` names
`notes.md:<n>` for the `n`th line, its raw file's lines joined by `\n`.
`records.rs` names `transcript.txt:1` to `:3`, and `moves.rs`'s
`an_old_unmoved_signal` names `transcript.txt:1`.

The scenarios of inbox-signals are held in `crates/weir-signals/tests/signals.rs`,
each by a test of its name in snake case, over `TRANSCRIPT` as
`transcript.txt`, three lines each ended by `\n`, and a second file
`notes.md` where a scenario needs two. The test `a_paraphrase` keeps
only its first half; its reading of a capture handed no raw file becomes
`a_capture_with_nothing_banked`.

The pack's reading cases hand `raw/notes.md`, whose lines are
`## acme 1.1`, an empty line, `- The shim is back, behind a flag.` and
`- Pin 1.0 to keep the old behaviour.`. Their positions become:

| case | signal | position |
|---|---|---|
| `a-reading-written-whole` | 1, 2 | `raw/notes.md:3`, `raw/notes.md:4`, in `case.toml` and `tip/` |
| `a-reading-refused-whole` | 1, 3 | `raw/notes.md:3`, `raw/notes.md:4` |
| `an-excerpt-standing-nowhere-refused` | 1 | `raw/notes.md:3`, its `reason_holds` `as one run beginning on line 3` |

`a-second-reading-refused` is refused before any signal is read, and
keeps its position. Two cases are added, each satisfying `[26]`, its
`start/` and `raw/` as `an-excerpt-standing-nowhere-refused`'s:

- `an-excerpt-on-another-line-refused`: the excerpt `Pin 1.0 to keep the
  old behaviour.` at `raw/notes.md:3`, refused naming
  `release/acme/1.1#1`, `excerpt` and `as one run beginning on line 3`.
- `a-position-naming-no-file-and-line-refused`: the excerpt `The shim is
  back, behind a flag.` at `the notes, the first item`, refused naming
  `release/acme/1.1#1`, `position` and `names no raw file and line`.

`pack.rs`'s `the_cases_named_are_among_them` names both.

## Risks / Trade-offs

- [A reader counts lines otherwise, from 0, or with a lone `\r` as an
  ending] → Its reading is refused, naming the signal, `position` or
  `excerpt`, and the file and line it gave, so it corrects the reading,
  as S5 of millpond's statement does.
- [A host hands in a raw file under another path than its offer named]
  → No position names it, and every signal of that file is refused
  naming `position`. The path is the offer's, which both hosts bank.
- [The spelling is weir's own, unspelled by the statement's C.2] → It is
  cheap to change while no reader writes it, and record 0010 says so.
