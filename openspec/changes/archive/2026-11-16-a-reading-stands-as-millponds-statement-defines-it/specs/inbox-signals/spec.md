## MODIFIED Requirements

### Requirement: A signal's excerpt stands verbatim in its capture's raw material

A reader's signal whose position holds SHALL be taken only when its excerpt is one unbroken run of the text of the file its position names, beginning on the line it names, `\r\n` evened to `\n` in both; otherwise the reading SHALL be refused naming the signal and `excerpt` (26; record 0010; core.16). No whitespace or letter's case SHALL be evened, and the excerpt SHALL be written as the reader handed it. The own signal's excerpt is the own excerpt as cut and is not looked for in raw material.

#### Scenario: A paraphrase

- **WHEN** a reading's signal has an excerpt whose words are close to the transcript's second line but not its own, at `transcript.txt:2`
- **THEN** the reading is refused, naming that signal and `excerpt`

#### Scenario: Verbatim across lines

- **WHEN** a signal's excerpt is the second and third lines of a raw transcript, as they stand there, at `transcript.txt:2`
- **THEN** it is taken

#### Scenario: Words from within a line

- **WHEN** a signal's excerpt is the words after the speaker's name on the transcript's second line, at `transcript.txt:2`
- **THEN** it is taken

#### Scenario: Line endings evened

- **WHEN** a raw transcript ends its lines with `\r\n`, and a signal's excerpt is its second and third lines ended with `\n`, at `transcript.txt:2`
- **THEN** it is taken, and its signal is written with the excerpt's `\n` line endings, as the reader handed it

#### Scenario: An excerpt on another line

- **WHEN** a signal's excerpt is the transcript's third line, at `transcript.txt:2`
- **THEN** the reading is refused, naming that signal and `excerpt`

#### Scenario: An excerpt in another file

- **WHEN** a signal's excerpt stands in `transcript.txt` alone, and its position names `notes.md:1`, which the writer hands in too
- **THEN** the reading is refused, naming that signal and `excerpt`

#### Scenario: An excerpt across two files

- **WHEN** a signal's excerpt is the last line of `notes.md` followed by the first line of `transcript.txt`, at `notes.md`'s last line
- **THEN** the reading is refused, naming that signal and `excerpt`

#### Scenario: Whitespace and case kept

- **WHEN** a signal's excerpt is the transcript's second line with two spaces where it holds one, or with a capital where it holds none, at `transcript.txt:2`
- **THEN** the reading is refused, naming that signal and `excerpt`

## ADDED Requirements

### Requirement: A reader's signal names its excerpt's file and line

A reader's signal SHALL be taken only when its position names, as `<path>:<line>`, one of the raw files the writer hands in for that capture; otherwise the reading SHALL be refused naming the signal and `position`, and its excerpt is not looked for (26; record 0010; core.16). `<path>` SHALL be everything before the position's last `:`, not empty, and equal byte for byte to the path a raw file is handed in under; `<line>` SHALL be a whole number of at least 1 with no sign or leading zero.

#### Scenario: A position naming no file and line

- **WHEN** a signal's position is `the transcript`, or `transcript.txt`, or `:2`
- **THEN** the reading is refused, naming that signal and `position`

#### Scenario: A line spelled otherwise

- **WHEN** a signal's position is `transcript.txt:0`, `transcript.txt:02`, `transcript.txt:+2` or `transcript.txt:2a`
- **THEN** the reading is refused, naming that signal and `position`

#### Scenario: A position naming a file not handed in

- **WHEN** a signal's position is `minutes.txt:1`, and the writer hands in `transcript.txt` alone
- **THEN** the reading is refused, naming that signal and `position`

#### Scenario: A capture with nothing banked

- **WHEN** a reading of one signal is written for a capture, and the writer hands in no raw file
- **THEN** the reading is refused, naming that signal and `position`, and a reading of none is taken for that capture

#### Scenario: A path holding a colon

- **WHEN** the writer hands in the transcript as `meeting:12/transcript.txt`, and a signal's excerpt is its second line, at `meeting:12/transcript.txt:2`
- **THEN** it is taken

### Requirement: A position's line is counted in its evened file

A reader's signal SHALL be taken only when the `<line>` its position names is a line of the file it names, the file's lines counted from 1 after every `\r\n` is evened to `\n`, a last line with no line ending among them; otherwise the reading SHALL be refused naming the signal and `position`, and its excerpt is not looked for (26; record 0010; core.16).

#### Scenario: A line past the file's end

- **WHEN** a signal's position is `transcript.txt:4`, and the transcript has three lines, each ended by `\n`
- **THEN** the reading is refused, naming that signal and `position`

#### Scenario: A last line with no line ending

- **WHEN** a raw file holds `first\nsecond`, and a signal's excerpt is `second`, at that file's line 2
- **THEN** it is taken
