## ADDED Requirements

### Requirement: A rule's item closes once, with its brief

Closing a rule's item SHALL form one write: its `item.rec` again with `closed_at` added and nothing else changed, and, where it holds signals, its `brief.md`, their excerpts as they stand, in the order of their captures' event times and then of their names, each after a blank line from the one before, with no word added (36, 128, 129; C.2). Whether its window has come SHALL be its caller's to read.

#### Scenario: A rule's item closes with its brief

- **WHEN** `point-releases-20261112T103000Z` holds two releases' signals, the later-released taken first, and it is closed
- **THEN** its `item.rec` holds `closed_at`, its `brief.md` holds the earlier release's excerpt, a blank line, then the later's, and the rule has no current item

#### Scenario: A closed item's rule makes its next item

- **WHEN** the rule's next release is taken after the close
- **THEN** its join makes `point-releases-20261112T110000Z`, and the closed item holds the two signals it held

### Requirement: Only a rule's open item closes

The check SHALL take a replacement of an `item.rec` that adds `closed_at` alone to a rule's item not closed yet, and refuse every other. A close of an item not held, of one no rule made, of one closed already, or at a text that is not an RFC 3339 time SHALL be refused naming the item and, where there is one, the field (128; core.21).

#### Scenario: An item closes once

- **WHEN** a closed item is closed again
- **THEN** it is refused, naming `closed_at`

#### Scenario: An item no rule made does not close

- **WHEN** an item made from a conclusion is closed
- **THEN** it is refused, naming `rule`

#### Scenario: A close at no time

- **WHEN** a rule's item is closed at `half past ten`
- **THEN** it is refused, naming `closed_at`

#### Scenario: An item record changed otherwise is refused

- **WHEN** a write replaces a rule's `item.rec` with `closed_at` added and its priority changed
- **THEN** the check refuses it, naming the path
