## ADDED Requirements

### Requirement: owed reads each record once

`owed` SHALL read each record its caller passes once, and SHALL find a delivery's confirmations and hand-overs by its key and reader, so its cost grows with the records and not with their square; and it SHALL give each delivery's hand-overs to a reader in the order added (103, 111; core.51).

#### Scenario: Hand-overs of two deliveries interleaved

- **WHEN** `owed` is called over two deliveries, `a` to `planner` and `reviewer` and `b` to `planner`, whose hand-overs by `notice` and then by `wake` interleave
- **THEN** it gives `planner` `a` then `b`, each with its own two hand-overs, `notice` then `wake`, and `reviewer` `a` with its one
