## MODIFIED Requirements

### Requirement: weir check reads a declaration
`weir check <declaration>` SHALL read the file it names as a
declaration, the elaboration sections included. When the declaration
reads, it SHALL list each kind of either format in name order with its
version, a simple kind with its stages, each with its deliverable or
that it is a conversation, and exit 0. When the declaration is refused,
it SHALL name each failing field with its reason on standard error and
exit non-zero. It SHALL leave the file as it was.

#### Scenario: A declaration that reads
- **WHEN** `weir check` is given a file declaring `fix` and `change`
- **THEN** it prints `fix@1` and `change@1` with their stages and deliverables, and exits 0

#### Scenario: A kind that lands with no stages is listed
- **WHEN** `weir check` is given a file declaring `train` with `lands = true` and `stages = []`
- **THEN** it prints `train@1` and then `  lands with no stages`, and exits 0

#### Scenario: A kind that names its fix is listed
- **WHEN** `weir check` is given a file declaring `train` with `lands = true`, `stages = []`, `landing_fix = "acme/fix@1"` and `max_landing_fixes = 2`
- **THEN** it prints `train@1`, `  lands with no stages`, `  landing fix acme/fix@1` and `  landing fixes at most 2`, in that order, and exits 0

#### Scenario: A declaration that is refused
- **WHEN** `weir check` is given a file whose stage declares two deliverables
- **THEN** it prints that stage's `deliverables` field with its reason on standard error, prints no kind, and exits non-zero

#### Scenario: Checking changes nothing
- **WHEN** `weir check` reads a file
- **THEN** the file's bytes after the check are the bytes before it

## ADDED Requirements

### Requirement: weir check lists a statechart kind
When the declaration it reads holds a statechart kind, `weir check`
SHALL list it among the kinds in name order with its version, its
title, its landing fix and its bound where it names them, and each
region with its initial state and its states.

#### Scenario: A statechart kind is listed
- **WHEN** `weir check` is given a file declaring the simple kind `bump` and C.4's statechart kind `persona`
- **THEN** it prints `bump@2` with its stages, then `persona@1`, `  title Persona test` and `  region stages from build: build, dropped (final), passed (final), stopped, test`, in that order, and exits 0
