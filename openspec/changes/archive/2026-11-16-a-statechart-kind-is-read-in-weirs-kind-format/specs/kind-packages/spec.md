## ADDED Requirements

### Requirement: A package of a statechart kind is refused naming its kind

`read-package` SHALL count a statechart kind its `kind.toml` declares among the package's kinds, so a `kind.toml` declaring a simple kind and a statechart kind declares two and is refused as any such package is (89). A package whose one kind is a statechart kind SHALL be refused naming `kinds.<kind>`, its reason saying it is a statechart kind and that `read-package` reads a package of a simple kind alone (92; design 12).

#### Scenario: A package of a statechart kind
- **WHEN** a folder holds only `kind.toml`, declaring the statechart kind `persona` with a title and one region, and is read held at `acme/persona@1`
- **THEN** the package is refused naming `kinds.persona`, its reason holding `is a statechart kind`

#### Scenario: A simple kind and a statechart kind in one package
- **WHEN** a package's `kind.toml` declares the simple kind `review` and the statechart kind `persona`
- **THEN** the package is refused naming `kinds`, the reason saying it declares 2 kinds, `persona` and `review`, and a package declares exactly one kind
