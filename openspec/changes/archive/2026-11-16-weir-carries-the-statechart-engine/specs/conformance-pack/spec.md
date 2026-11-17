## MODIFIED Requirements

### Requirement: Every case lies in one folder under its shape

The pack SHALL hold its cases at `pack/<shape>/<case>/`, one case per folder, each with a `case.toml` naming the clauses it exercises in `satisfies` (67; C.7; core.35). The shapes SHALL be exactly `host`, `adapter`, `cadence`, `branch`, `inbox`, `kinds`, `tick` and `delivery` (87; C.7). A clause is cited by its number, a whole number such as `14`, or a number with its letter written as a string, such as `"75a"`.

#### Scenario: A folder of no shape
- **WHEN** the pack holds `pack/hosts/held-key/case.toml`
- **THEN** the pack's check fails naming `pack/hosts` as no shape of the pack

#### Scenario: A case that cites nothing
- **WHEN** a case's `case.toml` has no `satisfies`, or an empty one
- **THEN** the pack's check fails naming that case

#### Scenario: A clause with a letter
- **WHEN** an import case names `satisfies = [37, "75a"]`
- **THEN** the check reads both as clauses and finds both in the statement

#### Scenario: A delivery case is a case of the pack
- **WHEN** the pack holds `pack/delivery/a-second-confirmation/case.toml` citing 102
- **THEN** the pack's check reads it as a case of the `delivery` shape

#### Scenario: A tick case is a case of the pack
- **WHEN** the pack holds `pack/tick/a-response-applied-once/case.toml` citing 97
- **THEN** the pack's check reads it as a case of the `tick` shape

### Requirement: The pack keeps an index of the statement's clauses

The pack SHALL hold, at `pack/clauses.toml`, the clause numbers of the statement in boomdrivedev/blueprints `design/weir/requirements.md`, each under the section it stands in (A.1 to A.14, B.1 to B.7), naming the commit of the statement it was read from, blueprints 006fc34. It SHALL hold every number of that commit: 94 and 95, statechart kinds, under A.8 with the kind format's other clauses; 96, 97 and 98, the engine, under A.13; and 99, the tick, under B.4.

#### Scenario: The index read
- **WHEN** the index is read
- **THEN** it names the statement's commit, every section of Parts A and B, and under each the numbers that section holds: 94 and 95 under A.8, 96 to 98 under A.13, 99 under B.4, 75a under B.2, 100 to 107 under A.14, and 108 to 113 under B.7 among them

### Requirement: The pack's check fails on a counted clause no case cites

The pack's check SHALL count the clauses of A.1 to A.10, of A.13, of A.14 and 71, and SHALL fail naming each counted clause that no case of any shape cites (67; S16). It SHALL count none of 65, 67, 68, 69 and 70, nor any clause of Part B, and a case citing one of them SHALL not fail the check for it.

#### Scenario: A clause of Part A cited by no case (S16)
- **WHEN** no case of any shape cites 33
- **THEN** the check fails naming `33`

#### Scenario: A clause of A.14 cited by no case
- **WHEN** no case of any shape cites 106
- **THEN** the check fails naming `106`

#### Scenario: A clause of A.13 cited by no case
- **WHEN** no case of any shape cites 97
- **THEN** the check fails naming `97`

#### Scenario: A clause the check does not count
- **WHEN** no case cites 69, and a case cites 87
- **THEN** the check neither fails for 69 nor for 87

#### Scenario: A cadence or branch case counts as any other
- **WHEN** 60 is cited only by a case under `pack/cadence/`, and 48 only by a case under `pack/branch/`
- **THEN** the check counts both as cited

#### Scenario: The sections the check counts
- **WHEN** the check's counted sections are read
- **THEN** they are A.1 to A.10, A.13 and A.14, with 71 beside them

### Requirement: weir's pack passes its own check

The pack as weir's repository holds it SHALL pass the check: every case cites at least one clause, every cited number names a clause, and every clause of A.1 to A.10, of A.13, of A.14 and 71 is cited by some case (67; core.35).

#### Scenario: The repository's pack
- **WHEN** the check is run over `pack/` and `pack/clauses.toml`
- **THEN** it passes, naming how many cases of each shape it read, `tick` and `delivery` among them

### Requirement: The index holds numbers alone and follows the statement

The index SHALL hold numbers and sections only, never a clause's words (core.37). A clause the statement adds, removes or renumbers SHALL be changed in the index in the change that follows the statement.

#### Scenario: The index read
- **WHEN** the index is read
- **THEN** it names the statement's commit, every section of Parts A and B, and under each the numbers that section holds: 96 to 98 under A.13, 99 under B.4, 75a under B.2, 100 to 107 under A.14, and 108 to 113 under B.7 among them
