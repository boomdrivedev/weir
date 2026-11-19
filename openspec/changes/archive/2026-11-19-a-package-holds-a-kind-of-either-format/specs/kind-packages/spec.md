## RENAMED Requirements

- FROM: `### Requirement: A package of a statechart kind is refused naming its kind`
- TO: `### Requirement: A package of a statechart kind gives its kind`

## MODIFIED Requirements

### Requirement: A package of a statechart kind gives its kind

`read-package` SHALL count a statechart kind its `kind.toml` declares among the package's kinds, so a `kind.toml` declaring a simple kind and a statechart kind declares two and is refused as any such package is (89). A package whose one kind is a statechart kind SHALL give that statechart kind, at the identity it is held at, with its content hash over every regular file the package holds, and SHALL be refused naming `kinds.<kind>`, or `kinds.<kind>.version`, where the identity it is held at is not its declaration's name and version, as a simple kind's package is (89, 90, 92).

#### Scenario: A package of a statechart kind
- **WHEN** a package holds only `kind.toml`, declaring the statechart kind `persona` titled `Persona test` with one region, and is read held at `acme/persona@1`
- **THEN** it reads, its identity `acme/persona@1`, its kind the statechart kind `persona` at version 1 with the title `Persona test` and no simple kind, and its content hash the hash of that one file

#### Scenario: A package of a statechart kind held at another version
- **WHEN** the same package is read held at `acme/persona@2`
- **THEN** it is refused naming `kinds.persona.version`, saying it makes the package `acme/persona@1` but is held at `acme/persona@2`

#### Scenario: A package of a statechart kind held at another name
- **WHEN** the same package is read held at `acme/review@1`
- **THEN** it is refused naming `kinds.persona`

#### Scenario: A simple kind and a statechart kind in one package
- **WHEN** a package's `kind.toml` declares the simple kind `review` and the statechart kind `persona`
- **THEN** the package is refused naming `kinds`, the reason saying it declares 2 kinds, `persona` and `review`, and a package declares exactly one kind
