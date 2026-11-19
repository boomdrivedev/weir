# Tasks

## 1. A package of either format

- [x] 1.1 Give `Package` a `PackageKind`, `Simple` or `Statechart`, in `crates/weir-kinds/src/package.rs`, exported from the crate, and let `read` give a package whose one kind is a statechart kind, its identity checked as a simple kind's is (89, 90, 92). In `crates/weir-kinds/tests/package.rs`, `a_package_of_a_statechart_kind` reads it, and `a_package_of_a_statechart_kind_held_at_another_version` and `a_package_of_a_statechart_kind_held_at_another_name` refuse it; the tests that read a simple kind's package read it through `PackageKind::simple`. True when `devenv shell -- moon run kinds:test` passes them, holding the kind-packages scenarios of those names.
