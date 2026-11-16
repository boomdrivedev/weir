# inbox-items Specification

## Purpose
Records the units of work curation makes of signals: an item, the
signals and weight read from its standing moves, its brief, and each
send of it with what became of it (34 to 36; C.2).

## Requirements

### Requirement: An item's signals are read from the standing moves that name it

An item's signals SHALL be exactly the signals whose standing join or attach names it, read from `moves.rec` whenever asked, and SHALL NOT be recorded in the item's record (34; core.20).

#### Scenario: A join replaced

- **WHEN** three signals are joined into `shim-timeout` and the join of one is then replaced by a drop
- **THEN** the item's signals are the other two, and its `item.rec` is unchanged

#### Scenario: An attach counts

- **WHEN** a signal is attached to the sent item `shim-timeout`
- **THEN** it is among the item's signals

### Requirement: An item's weight is computed, never stored

An item's weight SHALL be computed from its signals: how many, the sources of their captures, and the earliest and latest of their captures' event times; no record SHALL hold a weight (34; core.8, core.20).

#### Scenario: Weight from two sources

- **WHEN** an item holds three signals, two from release captures of 2026-10-01 and 2026-10-10 and one from a meeting capture of 2026-10-05
- **THEN** its weight is three signals, from `meeting` and `release`, spanning 2026-10-01 to 2026-10-10 as their event times give them

### Requirement: An item is made by a join or from an accepted conclusion

An item SHALL be recorded as `weir-item/1` with its name, whether a join or a conclusion made it, the conclusion where one did, and who named it and when (34; C.2). An item made from a conclusion SHALL name that conclusion; one with no conclusion named SHALL be refused.

#### Scenario: An item from a conclusion

- **WHEN** an item `pin-mise` is proposed from the conclusion `inquiry/toolchain#3`
- **THEN** the write adds `inbox/items/pin-mise/item.rec`, made by a conclusion and naming it, and the item is proposed with no signals

### Requirement: An item's name is one its inbox has never had

An item's name SHALL be one no item of the inbox has had, SHALL hold no `/`, and SHALL be neither empty, `.` nor `..`; any other SHALL be refused naming the name (35; core.20). A join naming an existing item grows it rather than making it.

#### Scenario: A name already had

- **WHEN** an item is proposed from a conclusion under the name of an item the inbox holds
- **THEN** it is refused, naming the name

#### Scenario: A nested name

- **WHEN** a join names the item `shim/timeout`
- **THEN** it is refused, naming the name

### Requirement: An item is proposed until first sent

An item SHALL be proposed while it has no send and sent once it has one (glossary; 36).

#### Scenario: The first send

- **WHEN** a proposed item is sent
- **THEN** it reads as sent from that tip on

### Requirement: An item's brief is the text handed for it, rewritten in place

An item's brief SHALL be `inbox/items/<name>/brief.md`, its first line naming `weir-brief/1` and the rest the text handed, unchanged; writing it again SHALL rewrite that file alone (36, 41; core.21). The crate SHALL compose no brief: it records the text curation or the operator hands (core.20).

#### Scenario: A brief written and rewritten

- **WHEN** a brief is written for `shim-timeout` and later written again with other text
- **THEN** `brief.md` holds the later text after its format line, and no other file changes

#### Scenario: A brief for no item

- **WHEN** a brief is written for an item the inbox does not hold
- **THEN** it is refused, naming the item

### Requirement: A send keeps the brief as handed

A send SHALL append to the `sends.rec` of each item it hands a `weir-send/1` record with the item, the target, the brief as it stood when sent, when it was sent, and the outcome `open`; the brief it keeps SHALL not change when `brief.md` later does (36; core.20). An item with no brief SHALL not be sent.

#### Scenario: A brief rewritten after a send

- **WHEN** `shim-timeout` is sent with one brief and its brief is then rewritten
- **THEN** its send still holds the brief as it was sent

#### Scenario: Several items to one target

- **WHEN** two items are sent together to one target
- **THEN** one write appends a send to each item's `sends.rec`, both with that target and time

#### Scenario: An item with no brief

- **WHEN** an item with no `brief.md` is sent
- **THEN** it is refused, naming the item

### Requirement: What became of a send is added beside it

A send's outcome SHALL be added as a record after it in the same `sends.rec`, naming the send by its target and when it was sent, and giving the outcome: a landing, a conclusion, or abandoned (36, 41; C.2). The send itself SHALL stay as written. An outcome for no send, or for a send already settled, SHALL be refused.

#### Scenario: A send that landed

- **WHEN** the send of `shim-timeout` lands as a commit
- **THEN** `sends.rec` gains a record naming that send with the outcome a landing and what landed, and the send's own record is unchanged

#### Scenario: An outcome for a settled send

- **WHEN** an outcome is added for a send whose outcome is already abandoned
- **THEN** it is refused, naming the send and its outcome

### Requirement: A rule's item names its rule and its priority

A rule's item SHALL be recorded as `weir-item/1` made by a join, named by `rule:<name>` at its making, with `rule` and `priority`, the priority's word, after `named_at`, and `closed_at` once its window closes. `rule` without `priority` or the converse, either on an item a conclusion made, or `closed_at` on an item no rule made, SHALL be refused naming the field (128; C.2; core.57).

#### Scenario: A rule's item record

- **WHEN** `point-releases`' join makes `point-releases-20261112T103000Z`
- **THEN** its `item.rec` holds `made_by: join`, `named_by: rule:point-releases`, `rule: point-releases` and `priority: low`, and reads back as written

#### Scenario: A rule's item with a rule and no priority

- **WHEN** a tip's `item.rec` holds `rule: point-releases` and no `priority`
- **THEN** reading the inbox is refused, naming `priority`

#### Scenario: A closed_at on an item no rule made

- **WHEN** a tip's `item.rec` holds `closed_at` and no `rule`
- **THEN** reading the inbox is refused, naming `closed_at`

### Requirement: A rule's join goes into its current item

A rule's join SHALL go into the rule's current item at the tip it lands on, the item naming the rule with no `closed_at`, and only where the rule has none SHALL make an item under the name handed; a name an item of the inbox has had SHALL then be refused naming it. A replay on a new tip SHALL find its current item again (35, 46, 128; core.26, core.57).

#### Scenario: A patch release then another ten minutes later

- **WHEN** with C.9's policy, a patch release is taken at 2026-11-12T10:05:00Z and a second at 10:15:00Z, each through an inbox over its tip
- **THEN** the first makes `point-releases-20261112T103000Z` with its rule and priority, and the second joins it, its `item.rec` unchanged and its signals the two (S26)

#### Scenario: A closed item takes no more joins

- **WHEN** the tip's item of `point-releases` holds `closed_at`, and a join naming `point-releases-20261112T110000Z` is handed over
- **THEN** the write makes `point-releases-20261112T110000Z`, and the closed item holds the signals it held

#### Scenario: Two writers of one rule meet one item

- **WHEN** a join naming `point-releases-20261112T110000Z` is formed on a tip where another writer has made `point-releases-20261112T103000Z`, not closed
- **THEN** the join goes into `point-releases-20261112T103000Z` and no item is made

#### Scenario: A rule's item under a used name

- **WHEN** the rule has no current item, and a join naming an item the inbox has had is handed over
- **THEN** it is refused, naming the item, and nothing is written
