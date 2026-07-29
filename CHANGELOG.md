# Changelog

All notable changes to this project will be documented in this file.

This project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## \[[0.700.0-rc.0](https://github.com/holochain/scaffolding/compare/v0.600.0...v0.700.0-rc.0)\] - 2026-07-29

### Features

- \[**BREAKING**\] Update scaffolded validate() codegen for Holochain 0.7.0-rc.3 by @ThetaSinner
  - Rewrites the syn/quote-based integrity zome validate() and coordinator signal_action codegen to match the hdi FlatOp/OpEntry/OpUpdate/OpDelete/ OpLink/OpRecord/OpActivity API introduced between 0.7.0-dev.23 and 0.7.0-rc.3 (FlatOp variants renamed, EntryCreationAction replaced by TypedAction<EntryCreationData>, Action becoming a {header, data} struct, link validation functions collapsed to a single TypedAction<CreateLinkData> param). Also drops the now-nonexistent "transport-iroh" Cargo feature.
  - Verified by scaffolding a fresh app (two entry types, one with a linked_from dependency, two link types) and confirming the generated integrity/coordinator zome crates compile against the real hdi=0.8.0-rc.2/ hdk=0.7.0-rc.2 crates.
  - Adds CLAUDE.md documenting the upgrade process for next time.
- Update holochain versions used in templates and required deps by @cdunster
- Add sweettest for scaffolded link-types by @jost-s in [#545](https://github.com/holochain/scaffolding/pull/545)
- Add rust integration test generation for entry types and collections by @jost-s in [#544](https://github.com/holochain/scaffolding/pull/544)
- \[**BREAKING**\] Removed support for any package manager except npm (#534) by @veeso in [#534](https://github.com/holochain/scaffolding/pull/534)
  - Feat!: removed support for any package manager except npm
  - **Breaking Change**: Remove any -p option

### Bug Fixes

- Bump holochain/hdi/hdk to rc.4/rc.3/rc.3 to fix crates.io drift by @ThetaSinner
  - Holochain=0.7.0-rc.3's own transitive deps (holochain_cascade, holochain_conductor_api) had drifted ahead on crates.io to holochain_state/holochain_conductor_api 0.7.0-rc.4, breaking a fresh `cargo generate-lockfile` for any app built against it - the coordinator zome's sweettest suite (which pulls in `holochain` as a dev-dependency) would not compile.
  - Confirmed hdi/hdk source is unchanged between rc.2 and rc.3 (only the holochain conductor crate itself changed, including a rewrite of the exact file that was failing to compile), so no codegen changes are needed here - this is purely a version bump to the point where upstream re-coordinated the release. Verified end-to-end: a fresh scaffold (two entry types, one link type) resolves and compiles cleanly with `cargo check --workspace --all-targets`, including the sweettest suite.
  - Also documents this class of issue in CLAUDE.md, including why decoupling the sweettest-only holochain version from the zome's hdi/hdk pin isn't possible (Cargo unifies the two through holochain's own test_utils -> hdk feature dependency).
- Bump holochain, hdk, hdi, ensure test app builds again by @mattyg in [#556](https://github.com/holochain/scaffolding/pull/556)

### Miscellaneous Tasks

- Trigger direnv reload if relevant files change by @cdunster
- Add missing perl package to the template's default devShell by @cdunster
- Update Rust toolchain version to 1.95.0 by @cdunster
- Nix flake update by @cdunster
- Use Rust toolchain from toolchain file in all Nix derivations by @cdunster
- Add missing perl package to this repository's own default devShell by @cdunster
- Update the CONTRIBUTING.md with shared content in [#560](https://github.com/holochain/scaffolding/pull/560)
- Clippy by @mattyg
- Rm playground by @mattyg
- Delete tryorama related code by @jost-s in [#547](https://github.com/holochain/scaffolding/pull/547)

### Build System

- Bump flake lock by @mattyg
- Bump rust by @mattyg
- Bump holochain related crate deps by @mattyg in [#550](https://github.com/holochain/scaffolding/pull/550)
- Bump hc-spin by @mattyg
- Bump flake lock by @mattyg

### CI

- Bump release workflows by @mattyg in [#555](https://github.com/holochain/scaffolding/pull/555)
- Bump prepare-release and publish-release workflows (#542) by @mattyg in [#542](https://github.com/holochain/scaffolding/pull/542)

### Refactor

- \[**BREAKING**\] Remove vanilla js template & hello-world example by @jost-s in [#568](https://github.com/holochain/scaffolding/pull/568)
- Fix clippy warnings after toolchain update by @cdunster in [#562](https://github.com/holochain/scaffolding/pull/562)
- \[**BREAKING**\] Delete react, lit and vue as selectable ui templates by @jost-s in [#549](https://github.com/holochain/scaffolding/pull/549)
  - Svelte is now the only supported UI framework. AI tools can be used to convert to other UI frameworks.
- Use build:happ in npm test command in scaffolded web-app by @jost-s

### Styling

- Fix format and lint errors in flake.nix by @cdunster

### Documentation

- Generalize CLAUDE.md, drop one-off version/feature specifics by @ThetaSinner in [#576](https://github.com/holochain/scaffolding/pull/576)
  - Removed references to this round's specific removed/renamed Cargo features and specific version-transition narrative (dev.23, rc.2, rc.3), since those won't mean anything to a future upgrade and read as noise. Kept the same guidance but phrased around what to check, not what changed this time.

### First-time Contributors

- @jost-s made their first contribution in [#568](https://github.com/holochain/scaffolding/pull/568)
- @cdunster made their first contribution in [#562](https://github.com/holochain/scaffolding/pull/562)
- @veeso made their first contribution in [#534](https://github.com/holochain/scaffolding/pull/534)

## \[[0.600.0](https://github.com/holochain/scaffolding/commits/v0.600.0)\] - 2025-11-20

### Features

- Upgrade to holochain 0.6.0 by @mattyg in [#522](https://github.com/holochain/scaffolding/pull/522)

## \[[0.600.0-dev.0](https://github.com/holochain/scaffolding/compare/v0.500.0...v0.600.0-dev.0)\] - 2025-10-16

### Features

- Remove UI references to the WebSDK by @ThetaSinner in [#512](https://github.com/holochain/scaffolding/pull/512)
- Remove support for Holo enabled scaffolding by @ThetaSinner
- Upgrade to 0.6 by @ThetaSinner

### Bug Fixes

- Repair logic to throw meaningful error if scaffolding with nix setup is attempted in an existing git repo (#516) by @matthme in [#516](https://github.com/holochain/scaffolding/pull/516)
- Set working directory before running cargo fmt (#514) by @matthme in [#514](https://github.com/holochain/scaffolding/pull/514)

### CI

- Update release actions by @ThetaSinner in [#513](https://github.com/holochain/scaffolding/pull/513)

### Miscellaneous Tasks

- Remove redundant words by @hustrust in [#510](https://github.com/holochain/scaffolding/pull/510)

<!-- generated by git-cliff -->
