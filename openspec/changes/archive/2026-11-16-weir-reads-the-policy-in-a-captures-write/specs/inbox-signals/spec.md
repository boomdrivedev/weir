## MODIFIED Requirements

### Requirement: A signal holds every field, and its nature is one of five

A signal SHALL be recorded as `weir-signal/1` with its identity, its nature in the field `kind`, who asserted it, what it asserts, its subjects and what it argues with (each a list, possibly empty), its excerpt and where it stands (25; C.2; record 0002). A signal lacking a field, or whose nature is not constraint, ask, question, commitment or reaction, SHALL be refused naming the field, but for an own signal a rule made, which has no nature (127).

#### Scenario: A signal of an unknown nature

- **WHEN** a reading holds a signal whose nature is `complaint`
- **THEN** the reading is refused, naming that signal and its `kind`

#### Scenario: Empty lists

- **WHEN** a signal with no subjects and nothing it argues with is written and read back
- **THEN** it reads back with both lists empty

#### Scenario: An own signal a rule made has no nature

- **WHEN** a tip holds a capture, not unread, titled `Daily stand-up`, whose one signal `#1` has no `kind`, its excerpt and assertion `Daily stand-up`, asserted by its source, at `title`
- **THEN** the inbox reads, and the signal reads back with no nature

#### Scenario: A signal of no nature that no rule made

- **WHEN** a tip holds an unread capture whose reading's signal `#1` has no `kind`
- **THEN** reading the inbox is refused, naming the path of its `signals.rec` and `kind`

### Requirement: A capture's own signal is recorded in the same write as the capture

The capture, own signal and rule's move the capture crate hands over SHALL be written in one write as `capture.rec`, a `signals.rec` holding the one signal `<key>#1`, its excerpt as cut and, where cut, a `cut` field of the lines and bytes left out as `<n> lines and <b> bytes`, the move appended to `moves.rec`, and the item it makes (19, 20, 27, 126; core.13). A capture with no own signal SHALL be written with no `signals.rec`.

#### Scenario: A cut own signal

- **WHEN** the capture crate hands over `release/mise/2026.10.10` with an own signal cut leaving out 812 lines and 39 020 bytes
- **THEN** one write adds `capture.rec` and `signals.rec`, the signal `release/mise/2026.10.10#1` with `cut: 812 lines and 39020 bytes`

#### Scenario: An unread capture

- **WHEN** the capture crate hands over a capture with no own signal
- **THEN** the write adds `capture.rec` alone, marked unread

#### Scenario: A held key

- **WHEN** the capture crate hands over a capture under a key the inbox holds
- **THEN** it is refused as held and nothing is written (14)

#### Scenario: A rule's drop in the capture's write

- **WHEN** the capture crate hands over a meeting's capture, its rule's own signal and `stand-ups`' drop with the reason `a stand-up`
- **THEN** one write adds `capture.rec`, `signals.rec` and the drop of `#1` appended to `moves.rec`, its target and its note `a stand-up`

#### Scenario: A rule's join makes its item in the capture's write

- **WHEN** the capture crate hands over a release's capture, its own signal and `point-releases`' join naming `point-releases-20261112T103000Z`, and the inbox holds no item of that rule
- **THEN** one write adds `capture.rec`, `signals.rec`, the join appended to `moves.rec`, and the item's `item.rec`

#### Scenario: A daily stand-up dropped as it lands

- **WHEN** a meeting's offer titled `Daily stand-up`, with no excerpt, stating its length and three attendees, is taken with C.9's policy through an inbox over its tip
- **THEN** one write adds the capture, not unread, one own signal of the title with no nature, and a drop by `rule:stand-ups`, and no reader is charged with the capture (S27)

## ADDED Requirements

### Requirement: An own signal a rule made is its capture's title

An own signal with no nature SHALL be taken only with a rule's move, and only as `<key>#1` with its excerpt and assertion its capture's title, or its key where it has none, asserted by its capture's source, at `title`, with no subjects, argument or cut; any other SHALL be refused naming the field. Its excerpt stands in its capture's offer and SHALL NOT be looked for in raw material (26, 127; core.16).

#### Scenario: A rule's signal whose excerpt is not the title

- **WHEN** the capture crate hands over a capture titled `Daily stand-up` with an own signal of no nature whose excerpt is `Weekly stand-up`, and a rule's drop
- **THEN** it is refused, naming `excerpt`, and nothing is written

#### Scenario: A signal of no nature with no rule's move

- **WHEN** the capture crate hands over a capture with an own signal of no nature and no rule's move
- **THEN** it is refused, naming `signal`, and nothing is written

### Requirement: A rule's move stands once written, in no batch

A rule's move SHALL be a join or a drop of its capture's own signal, by `rule:<name>`, at the capture's taken-at time, in no batch, and SHALL stand once written; a drop's reason SHALL be its target and its note. A rule's move with no own signal SHALL be refused naming `signal` (29 to 32, 128; core.18, core.19, core.60).

#### Scenario: A rule's move names its rule

- **WHEN** `point-releases`' join of `release/x/1.2.1#1` is written
- **THEN** its record names `by: rule:point-releases` and the capture's taken-at time, holds no `batch`, and it is the signal's standing move

#### Scenario: A rule's move with no signal

- **WHEN** the capture crate hands over a capture with no own signal and a rule's drop
- **THEN** it is refused, naming `signal`, and nothing is written
