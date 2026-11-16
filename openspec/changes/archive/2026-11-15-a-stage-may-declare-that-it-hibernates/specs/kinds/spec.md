## ADDED Requirements

### Requirement: A stage reads whether it hibernates
A stage's `hibernate` SHALL read with the stage, `true` or `false` as
written, and SHALL be returned left out when the stage leaves it out,
with no value standing in for it, and never among the stage's sections
(53, 56, 58 and C.4 as amended at blueprints 30dce35).

#### Scenario: A stage reads whether it hibernates
- **WHEN** a kind's three stages declare `hibernate = true`, `hibernate = false` and no `hibernate`
- **THEN** the first reads hibernating, the second not hibernating, and the third with `hibernate` left out, no value standing in for it, and none of the three holds `hibernate` among its sections

### Requirement: Any stage may declare that it hibernates
Any stage MAY declare `hibernate`, a conversation or a stage with a
deliverable, one that waits or repeats or not, and none SHALL be refused
for it but by its shape. What hibernating means, and what a stage that
leaves it out does, SHALL be left to the reader (53, 58; core.32).

#### Scenario: Any stage may hibernate
- **WHEN** a conversation that waits, and a stage delivering `tasks` with `repeat = "per-task-group"`, each declare `hibernate = true`
- **THEN** the declaration reads, both stages hibernating, the first waiting and the second repeating per task group

### Requirement: A hibernate on a kind's table is a section
On a kind's own table `hibernate` is no field the format names: it SHALL
be returned among the kind's sections and SHALL never be read as its
stages' (53, 56).

#### Scenario: A hibernate on a kind's table is a section
- **WHEN** a kind's table declares `hibernate = true` beside its one stage, which declares no `hibernate`
- **THEN** the kind reads with `hibernate` returned unread among its sections, and its stage with `hibernate` left out

### Requirement: A stage's hibernate that is not a bool is refused
A stage's `hibernate` that is not a bool SHALL be refused naming that
stage's `hibernate` field, with the reason that it must be true or
false, as a field of the wrong shape is, and the declaration SHALL
return no kind (57, C.4 as amended at blueprints 30dce35).

#### Scenario: A hibernate of the wrong shape
- **WHEN** three stages of one kind declare `hibernate = "yes"`, `hibernate = 1` and `hibernate = [true]`
- **THEN** the declaration is refused naming each stage's `hibernate` field in stage order, each reason `must be true or false`, and no kind is returned

### Requirement: The pack's kinds cases hold a stage that hibernates
Among its kinds cases citing 51 to 58, the pack SHALL hold a kind whose
stages declare `hibernate = true`, `hibernate = false` and none, read
with each as written (53, 58 and C.4 as amended at blueprints 30dce35;
C.7).

#### Scenario: A stage that hibernates is a case of its own
- **WHEN** a case's declaration gives `talk` the stages `draft`, a conversation that waits with `hibernate = true`, `code`, delivering commits with `hibernate = false`, and `review`, a conversation declaring no `hibernate`
- **THEN** the case cites 53 and 58 and expects `draft` hibernating, `code` not hibernating and `review` with `hibernate` left out, and the crate gives each

### Requirement: A case whose stage differs in its hibernate is named
A stage a declaration's kinds case expects with `hibernate` left out
SHALL be one read with it left out, and a case whose stage differs from
the stage read in its `hibernate` SHALL be named, with the stages read
and the stages the case expects (C.4 as amended at blueprints 30dce35;
C.7).

#### Scenario: A case whose stage differs in its hibernate is named
- **WHEN** a case expects `talk`'s one stage `draft`, a conversation, with `hibernate = true`, and the stage read leaves `hibernate` out
- **THEN** the crate's tests give one difference naming the case, the stages read with `draft`'s `hibernate` left out and the stages the case expects with it `true`

## MODIFIED Requirements

### Requirement: A declaration's kinds case gives each kind's version, stages and landing fix

Each kind a declaration's case expects SHALL give its version and its
stages in order, none where it lands with none, and its landing fix and
how many it runs where it names them, and neither where it does not.
Each stage SHALL give its name, its deliverable or that it is a
conversation, its `repeat` where it repeats, and its `hibernate`, `true`
or `false`, where it declares one (C.4 as amended at blueprints
30dce35; C.7).

#### Scenario: A kind that names its fix, and one with stages that may not
- **WHEN** one case's declaration gives `train` `lands = true`, `stages = []`, `landing_fix = "acme/fix@1"` and `max_landing_fixes = 2`, a second gives `bump` one stage, `lands = true`, `landing_fix = "acme/fix@1"` and `max_landing_fixes = 2`, and a third holds a package whose one file is a `kind.toml` declaring `train` as the first does, held at `acme/train@1`
- **THEN** the first expects `train` at version 1 with no stages, `landing_fix` `acme/fix@1` and `max_landing_fixes` 2; the second the refusal naming `kinds.bump.landing_fix` and then `kinds.bump.max_landing_fixes`; the third the identity `acme/train@1`, the hash of that one file and `train` with `landing_fix` `acme/fix@1` and `max_landing_fixes` 2; and the crate gives each
