# Proposal

## Why

The statement gives a package exactly one kind, of either format, and
read-package gives that kind with its identity and content hash (89,
92). The change that read statechart kinds left a package of one
refused, since `Package.kind` was a simple kind and carrying a
statechart kind changes `Package` for every reader of packages (design
12 of `a-statechart-kind-is-read-in-weirs-kind-format`). flume's own
kinds are one package per version, each in its version's format, and a
host's source may hold a package of a statechart kind, trusted, locked
and resolved as a simple kind's package is (flume 154 to 156, C.2). So
read-package now reads a package of either format.

## What Changes

- `Package.kind` is a `PackageKind`: `Simple(Kind)` or
  `Statechart(StatechartKind)`, with its `name()`, `version()`,
  `simple()` and `statechart()`.
- A package whose one kind is a statechart kind reads, its identity
  checked against the declaration's name and version as a simple
  kind's is, and its content hash over every regular file as every
  package's is (89, 90, 92).
- A `kind.toml` declaring a simple kind and a statechart kind still
  declares two and is refused as before (89).
- Every part a statechart kind's states name in their `params` is its
  reader's to read, since the stage template's parameters beside weir's
  are the reader's (95, 98); weir checks the parts a simple kind's
  stages name, as before.

## Capabilities

### Modified Capabilities

- `kind-packages`: a package of a statechart kind gives its kind.
