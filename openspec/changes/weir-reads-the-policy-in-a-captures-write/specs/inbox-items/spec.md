## ADDED Requirements

### Requirement: A rule's item names its rule and its priority

A rule's item SHALL be recorded as `weir-item/1` made by a join, named by `rule:<name>` at its making, with `rule` and `priority`, the priority's word, after `named_at`, and `closed_at` once its window closes. `rule` without `priority` or the converse, either on an item a conclusion made, or `closed_at` on an item no rule made, SHALL be refused naming the field (128; C.2; core.58).

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

A rule's join SHALL go into the rule's current item at the tip it lands on, the item naming the rule with no `closed_at`, and only where the rule has none SHALL make an item under the name handed; a name an item of the inbox has had SHALL then be refused naming it. A replay on a new tip SHALL find its current item again (35, 46, 128; core.26, core.58).

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
