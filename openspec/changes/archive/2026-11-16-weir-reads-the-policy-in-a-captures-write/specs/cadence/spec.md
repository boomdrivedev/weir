## ADDED Requirements

### Requirement: A schedule gives the first time it names after an instant

A schedule SHALL give the first time it names strictly after an instant, read in the time zone given, or none where it names no later time, so a rule's item is named for its window's close (129; C.5, C.9; core.33).

#### Scenario: The first time after an instant

- **WHEN** the schedule `*/30 * * * *` is asked, in UTC, for the first time after 2026-11-12T10:05:00Z
- **THEN** it gives 2026-11-12T10:30:00Z

#### Scenario: An instant on a named time

- **WHEN** the schedule `*/30 * * * *` is asked, in UTC, for the first time after 2026-11-12T10:30:00Z
- **THEN** it gives 2026-11-12T11:00:00Z

#### Scenario: A schedule that names no later time

- **WHEN** the schedule `0 0 30 2 *` is asked for the first time after 2026-11-12T10:05:00Z
- **THEN** it gives none
