# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.14.2] - 2025-04-02

### Changed

 - Bump tokio-util from 0.7.13 to 0.7.14 in [#19](https://github.com/z0w13/tulpje/pull/19)
 - Bump chrono from 0.4.39 to 0.4.40 in [#17](https://github.com/z0w13/tulpje/pull/17)
 - Bump uuid from 1.15.1 to 1.16.0 in [#16](https://github.com/z0w13/tulpje/pull/16)

### Commit Details

<details><summary>view details</summary>

 * build(deps): bump tokio-util from 0.7.13 to 0.7.14 ([`bf5a40c`](https://github.com/z0w13/tulpje/commit/bf5a40cbcb61972300bde05ce833e12a8fcc0f6c))
 * build(deps): bump chrono from 0.4.39 to 0.4.40 ([`65a2452`](https://github.com/z0w13/tulpje/commit/65a2452173fec94286489bde93caad9af5b02ae8))
 * build(deps): bump uuid from 1.15.1 to 1.16.0 ([`7cc4bd4`](https://github.com/z0w13/tulpje/commit/7cc4bd428166d713690c8e6a6b6cc3cc77099d61))
</details>

## [0.14.1] - 2025-03-10

### Changed

 - Bump uuid from 1.13.2 to 1.15.1 in [#12](https://github.com/z0w13/tulpje/pull/12)
 - Bump serde from 1.0.216 to 1.0.219 in [#14](https://github.com/z0w13/tulpje/pull/14)
 - Bump uuid from 1.11.0 to 1.13.2 in [#2](https://github.com/z0w13/tulpje/pull/2)
 - Bump tokio from 1.42.0 to 1.43.0 in [#3](https://github.com/z0w13/tulpje/pull/3)
 - Disable chrono wasmbind feature for our code

### Commit Details

<details><summary>view details</summary>

 * build(deps): bump uuid from 1.13.2 to 1.15.1 ([`03bd122`](https://github.com/z0w13/tulpje/commit/03bd122951b391c4c56b233e7745f230925bf167))
 * build(deps): bump serde from 1.0.216 to 1.0.219 ([`2d4e975`](https://github.com/z0w13/tulpje/commit/2d4e975abe6f93c8e06ef20d63928f0156d4389f))
 * build(deps): bump uuid from 1.11.0 to 1.13.2 ([`723de3d`](https://github.com/z0w13/tulpje/commit/723de3d1e143c95a1d9c6013905a2a4d81070854))
 * build(deps): bump tokio from 1.42.0 to 1.43.0 ([`d4e6ba6`](https://github.com/z0w13/tulpje/commit/d4e6ba620d929564a901dbdba7e73796e07b33b6))
 * chore: disable chrono wasmbind feature for our code ([`429b4a4`](https://github.com/z0w13/tulpje/commit/429b4a4a6513837d5cd22ce5bcaad4ea04bab40a))
</details>

## [0.14.0] - 2025-01-16

### Breaking Changes

 - Update twilight to 0.16.0

### Removed

 - Remove unnecessary logging of unhandled events

### Commit Details

<details><summary>view details</summary>

 * fix(framework): remove unnecessary logging of unhandled events ([`0ec373f`](https://github.com/z0w13/tulpje/commit/0ec373f8bb8cc2ff5a0ae7d00a78d8e30dec981a))
 * chore!: update twilight to 0.16.0 ([`a974856`](https://github.com/z0w13/tulpje/commit/a9748566df2e386a97c6921c649cec57879fc456))
</details>

## [0.13.0] - 2025-01-12

### Breaking Changes

 - Added support for subcommands and subcommand groups

### Removed

 - Remove unused file module/module.rs

### Commit Details

<details><summary>view details</summary>

 * feat(framework)!: added support for subcommands and subcommand groups ([`007709f`](https://github.com/z0w13/tulpje/commit/007709f5a27dfa44e961653d134ece2e4820f3e1))
 * chore: remove unused file module/module.rs ([`a514ee2`](https://github.com/z0w13/tulpje/commit/a514ee27c975c5487c88f18c296fca59f7fbdff9))
</details>

## [0.11.1] - 2025-01-08

### Added

 - Add CHANGELOG.md

### Changed

 - Version tulpje-framework separately from the bot

### Commit Details

<details><summary>view details</summary>

 * docs: add CHANGELOG.md ([`0d202e7`](https://github.com/z0w13/tulpje/commit/0d202e7782b924955891541eb824b1026104be14))
 * chore: version tulpje-framework separately from the bot ([`916f1ff`](https://github.com/z0w13/tulpje/commit/916f1ff26bfd4687953f5435c61005f3cc5b444e))
</details>

## [0.11.0] - 2025-01-05

### Breaking Changes

- Move DisordEventMeta to tulpje-framework and rename it Metadata
- Remove unused InteractionHandler trait

### Added

- Add missing package metadata

### Changed

- Mark all crates as publishable

### Commit Details

<details><summary>view details</summary>

- mark all crates as publishable ([`3867bf6`](https://github.com/z0w13/tulpje/commit/3867bf60346a8391d98081d2370015ce6ec2d891))
- move DisordEventMeta to tulpje-framework and rename it Metadata ([`e1e93d7`](https://github.com/z0w13/tulpje/commit/e1e93d7903ff7a48066214ca002554ce91e4a9b3))
- remove unused InteractionHandler trait ([`641c297`](https://github.com/z0w13/tulpje/commit/641c297e65d00353d7b34147dda4e78d46114512))
- add missing package metadata ([`7a9e193`](https://github.com/z0w13/tulpje/commit/7a9e1939cf7fad86b6234612934413139d82e936))
</details>

## [0.9.0] - 2025-01-05

### Breaking Changes

- Rework sending messages into framework, and refactor Scheduler to follow similar conventions to Dispatch

### Commit Details

<details><summary>view details</summary>

- rework sending messages into framework, and refactor Scheduler to follow similar conventions to Dispatch ([`08bf914`](https://github.com/z0w13/tulpje/commit/08bf9145d5a412fffd3f489c4667f51f879ae4c1))
</details>

## [0.8.0] - 2025-01-05

### Breaking Changes

- Implement framework with main loop and shutdown functionality

### Commit Details

<details><summary>view details</summary>

- implement framework with main loop and shutdown functionality ([`891be50`](https://github.com/z0w13/tulpje/commit/891be50d55ef9869a0f10b48c1f50f0bc0181cd9))
</details>

## [0.7.0] - 2025-01-05

### Breaking Changes

- Mark builder methods as #[must_use]

### Changed

- Run `cargo fmt`
- Refactor the scheduler so we can actually enable/disable tasks, even when the scheduler isn't running yet

### Fixed

- Mark contexts/handlers as Sync + Send
- Enable clippy::option_if_let_else and fix warnings
- Enable clippy::manual_assert and fix warnings
- Enable clippy::partial_pub_fields and fix warnings
- Enable clippy::clone_on_ref_ptr and fix warnings
- Enable clippy::redundant_clone and fix warnings
- Enable clippy::needless_pass_by_value and fix warnings
- Enable clippy::semicolon_if_nothing_returned and fix warnings

### Removed

- Remove unused macros

### Commit Details

<details><summary>view details</summary>

- remove unused macros ([`5a9a1d5`](https://github.com/z0w13/tulpje/commit/5a9a1d58eed9ff9b1eb3c111aa1e02b38a9be1df))
- run `cargo fmt` ([`8e61d27`](https://github.com/z0w13/tulpje/commit/8e61d27a38b8031dc2d30c23c17b150dfd6d72ec))
- mark contexts/handlers as Sync + Send ([`d946b1d`](https://github.com/z0w13/tulpje/commit/d946b1d44f127d4550aad937be0c44a182aa9a12))
- enable clippy::option_if_let_else and fix warnings ([`bc27650`](https://github.com/z0w13/tulpje/commit/bc27650b9d073b67488039fc1ccd6322d42b4ee3))
- enable clippy::manual_assert and fix warnings ([`8b672eb`](https://github.com/z0w13/tulpje/commit/8b672eba70f3046e5d0458b02d1f1417ad88afca))
- enable clippy::partial_pub_fields and fix warnings ([`2155f3a`](https://github.com/z0w13/tulpje/commit/2155f3a4d6434c79205e6682f14bcd9c7a5e7932))
- enable clippy::clone_on_ref_ptr and fix warnings ([`b457f62`](https://github.com/z0w13/tulpje/commit/b457f624fad3e8030262d980b2879fc7ccc71fc3))
- mark builder methods as #[must_use] ([`3ec3a8a`](https://github.com/z0w13/tulpje/commit/3ec3a8a24344c5e1e780b54b132e68fc1e443383))
- enable clippy::redundant_clone and fix warnings ([`92e81e9`](https://github.com/z0w13/tulpje/commit/92e81e90362a5eb39625bef35487b06af8a20cc7))
- enable clippy::needless_pass_by_value and fix warnings ([`7e448c6`](https://github.com/z0w13/tulpje/commit/7e448c63bd57e2c3337b8dfb2618717f307ff368))
- enable clippy::semicolon_if_nothing_returned and fix warnings ([`a526faf`](https://github.com/z0w13/tulpje/commit/a526fafd635d3840b7eac26c8fe32bce923e7679))
- refactor the scheduler so we can actually enable/disable tasks, even when the scheduler isn't running yet ([`259b15a`](https://github.com/z0w13/tulpje/commit/259b15a5d0a2ef7a92fc9695525d800c01d92bd4))
</details>

## [0.6.0] - 2025-01-05

### Changed

- Don't hardcode guild module list
- Rework module system, registry, and task scheduler
- Cargo fmt

### Commit Details

<details><summary>view details</summary>

- don't hardcode guild module list ([`6b8000e`](https://github.com/z0w13/tulpje/commit/6b8000e973e6a6d333b4bf83cd7d814d79a48871))
- rework module system, registry, and task scheduler ([`ba4aae2`](https://github.com/z0w13/tulpje/commit/ba4aae287376f7040b6798c30d7be4d6c0a12ed2))
- cargo fmt ([`4a2d7d8`](https://github.com/z0w13/tulpje/commit/4a2d7d8b1f29ed55553fb7f01f73f0499600d7fd))
</details>

## [0.5.0] - 2025-01-05

### Breaking Changes

- Don't pass context in constructor
- Disallow adding tasks after starting scheduler

### Changed

- Per-guild commands

### Fixed

- InteractionRegistry::get should not be &mut

### Commit Details

<details><summary>view details</summary>

- InteractionRegistry::get should not be &mut ([`8eb9da9`](https://github.com/z0w13/tulpje/commit/8eb9da9b96af410807b9ce789aaf2a30726f4f74))
- per-guild commands ([`172d91c`](https://github.com/z0w13/tulpje/commit/172d91c8fe43e9ff7d8c46f02290712a28a7ea75))
- don't pass context in constructor ([`473c7d8`](https://github.com/z0w13/tulpje/commit/473c7d81c351f0cd2d7c16af747c30bb22d0b74c))
- disallow adding tasks after starting scheduler ([`f10212c`](https://github.com/z0w13/tulpje/commit/f10212ce1674b950033b934c08fe08834044702d))
</details>

## [0.4.0] - 2024-12-30

### Added

- Added `CommandContext::defer` helper method
- Added `CommandContext::update` method to update the current interaction's message (after defer)
- Added helper methods to get command options

### Changed

- PluralKit module
- Task scheduling using cron syntax
- Helper method to create CommandContext from base context
- Macros for making registering handlers slightly nicer
- Implemented basic command and event handling framework

### Fixed

- Thread safetey ugh headaches

### Commit Details

<details><summary>view details</summary>

- PluralKit module ([`eeb11e5`](https://github.com/z0w13/tulpje/commit/eeb11e5faf20f394a7a2e350c78706f152f85187))
- task scheduling using cron syntax ([`dbd42cb`](https://github.com/z0w13/tulpje/commit/dbd42cb547620d5c9a79b4618bcd87ac842629e6))
- thread safetey ugh headaches ([`84c6eab`](https://github.com/z0w13/tulpje/commit/84c6eab779e30ca2f84aec3360f6a74abda611aa))
- added `CommandContext::defer` helper method ([`c7ab85a`](https://github.com/z0w13/tulpje/commit/c7ab85a8ef48467a81977348ddd8bd9f2170216d))
- added `CommandContext::update` method to update the current interaction's message (after defer) ([`8c0541f`](https://github.com/z0w13/tulpje/commit/8c0541f8b76ee242aeb302b9b58210edcd91e39d))
- added helper methods to get command options ([`feec1f5`](https://github.com/z0w13/tulpje/commit/feec1f530d69957021fbc57bfd30630cecc5814d))
- helper method to create CommandContext from base context ([`02e18e4`](https://github.com/z0w13/tulpje/commit/02e18e412da53fab06507c118679bb5342427405))
- macros for making registering handlers slightly nicer ([`178e4b7`](https://github.com/z0w13/tulpje/commit/178e4b7b6c0f0a4df8469944038a2cf742a9e96a))
- implemented basic command and event handling framework ([`cde4d29`](https://github.com/z0w13/tulpje/commit/cde4d2940656156c0b1d1d5754b6de8e3139ed31))
</details>
<!-- generated by git-cliff -->
