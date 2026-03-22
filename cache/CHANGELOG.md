# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.5.0-rc.1] - 2026-03-22

### Changed

 - Front change notifications
 - Switch to using `mod.rs` files

### Commit Details

<details><summary>view details</summary>

 * feat(pluralkit): front change notifications ([`09e7223`](https://github.com/z0w13/tulpje/commit/09e72236515752c1645fcb2aa7877742a8cb8f7d))
 * refactor: switch to using `mod.rs` files ([`26a61a2`](https://github.com/z0w13/tulpje/commit/26a61a23019ee851f397a7721ec2eaafb638e9e5))
</details>

## [0.4.1] - 2026-03-20

### Changed

 - Update `twilight-cache-inmemory`, `twilight-gateway`, `twilight-http` and `twilight-model` from 0.17.0 to 0.17.1
 - Make `serde` and `serde_json` workspace dependencies

### Removed

 - Remove catch-all for events, add missing ignored events

### Commit Details

<details><summary>view details</summary>

 * chore(deps): update `twilight-cache-inmemory`, `twilight-gateway`, `twilight-http` and `twilight-model` from 0.17.0 to 0.17.1 ([`a79d238`](https://github.com/z0w13/tulpje/commit/a79d238af1c8672ec596be73b72f3b2974de3311))
 * fix(cache): remove catch-all for events, add missing ignored events ([`14a70f5`](https://github.com/z0w13/tulpje/commit/14a70f5b0000754be8d626438369d5bb72572d4f))
 * chore(deps): make `serde` and `serde_json` workspace dependencies ([`46e7099`](https://github.com/z0w13/tulpje/commit/46e7099adb6eb81d51d7011d42fde7e0fd2f5be6))
</details>

## [0.4.0] - 2025-11-15

### Added

 - Add `avatar_decoration_data` and `banner` fields to user

### Changed

 - Bump `serde_json` from 1.0.140 to 1.0.145
 - Bump `serde` from 1.0.219 to 1.0.228
 - Move redis crate to workspace deps
 - Update to rust 1.90.0, fix lint warnings, `cargo fmt`
 - `clippy::collapsible_if`
 - `cargo fmt`
 - Specify edition on workspace level
 - Move twilight-* crates to workspace deps

### Fixed

 - Update User on MemberUpdate
 - Hanging connections

### Removed

 - Remove deref

### Commit Details

<details><summary>view details</summary>

 * fix(cache): add `avatar_decoration_data` and `banner` fields to user ([`4366dec`](https://github.com/z0w13/tulpje/commit/4366dec57ab1bb7643e1a8c79bc9f03e3d22ee73))
 * fix(cache): remove deref ([`6121d0b`](https://github.com/z0w13/tulpje/commit/6121d0b3c82e11286bf9082ada2deec20b4d945e))
 * fix(cache): update User on MemberUpdate ([`d5a8737`](https://github.com/z0w13/tulpje/commit/d5a87377821d7c5e733d779d97fd39c55467e58b))
 * chore(deps): bump `serde_json` from 1.0.140 to 1.0.145 ([`0dd1cd8`](https://github.com/z0w13/tulpje/commit/0dd1cd8340faf6afc223ad6e1e5b1c5e3162285a))
 * chore(deps): bump `serde` from 1.0.219 to 1.0.228 ([`5b1f399`](https://github.com/z0w13/tulpje/commit/5b1f39919a293e04bb34b12a6e843b044f7ebe2d))
 * chore(deps): move redis crate to workspace deps ([`b597314`](https://github.com/z0w13/tulpje/commit/b59731451f1a88e1839675542d0aec6b998a346e))
 * chore(build): update to rust 1.90.0, fix lint warnings, `cargo fmt` ([`4a93c3b`](https://github.com/z0w13/tulpje/commit/4a93c3be063b99cbf6f4cd773e4b6fcf60f0b9bc))
 * chore(lint): `clippy::collapsible_if` ([`5c9d89e`](https://github.com/z0w13/tulpje/commit/5c9d89e3def56d8672cfa5399ced073f28884e99))
 * chore: `cargo fmt` ([`e76d893`](https://github.com/z0w13/tulpje/commit/e76d893b5102eca310144ab258e79553cb5b2f41))
 * refactor(build): specify edition on workspace level ([`f5b7a79`](https://github.com/z0w13/tulpje/commit/f5b7a79c4d5c5051e9dc3cc8b0def19fe22c63a6))
 * chore(deps): move twilight-* crates to workspace deps ([`f23e379`](https://github.com/z0w13/tulpje/commit/f23e37987e31d52258c6f6a5a4b856b33e2756ef))
 * fix(gateway): hanging connections ([`dc44437`](https://github.com/z0w13/tulpje/commit/dc44437096a4792f9782ccc219ce2f29d7c221c3))
</details>

## [0.3.0] - 2025-04-02

### Changed

 - Bump serde_json from 1.0.138 to 1.0.140 in [#15](https://github.com/z0w13/tulpje/pull/15)

### Fixed

 - Redis should have feature `tokio-comp` not `aio`

### Commit Details

<details><summary>view details</summary>

 * fix(handler): redis should have feature `tokio-comp` not `aio` ([`89b3522`](https://github.com/z0w13/tulpje/commit/89b35222f4bc99d8a03baceb6ee66d0da80ed4e6))
 * build(deps): bump serde_json from 1.0.138 to 1.0.140 ([`551fc09`](https://github.com/z0w13/tulpje/commit/551fc09bce0e23187a2271c076726170f0de3170))
</details>

## [0.2.0] - 2025-03-10

### Breaking Changes

 - Use redis-rs directly instead of through bb8 pool

### Changed

 - Bump redis from 0.28.2 to 0.29.1 in [#11](https://github.com/z0w13/tulpje/pull/11)
 - Bump serde from 1.0.216 to 1.0.219 in [#14](https://github.com/z0w13/tulpje/pull/14)
 - Bump serde_json from 1.0.133 to 1.0.138 in [#1](https://github.com/z0w13/tulpje/pull/1)

### Commit Details

<details><summary>view details</summary>

 * build(deps): bump redis from 0.28.2 to 0.29.1 ([`0c7fea0`](https://github.com/z0w13/tulpje/commit/0c7fea0b667bb7dd32bf6f0aa9212c5b630a0568))
 * build(deps): bump serde from 1.0.216 to 1.0.219 ([`2d4e975`](https://github.com/z0w13/tulpje/commit/2d4e975abe6f93c8e06ef20d63928f0156d4389f))
 * build(deps): bump serde_json from 1.0.133 to 1.0.138 ([`66da8bd`](https://github.com/z0w13/tulpje/commit/66da8bdc2851a0e5ca287742bb11c455b8258976))
 * refactor!: use redis-rs directly instead of through bb8 pool ([`61a548a`](https://github.com/z0w13/tulpje/commit/61a548abb36de63ee410cfa8a662e221478f14a8))
</details>

## [0.1.0] - 2025-01-16

### Changed

 - Implemented tulpje-cache, a redis based caching library

### Commit Details

<details><summary>view details</summary>

 * feat: implemented tulpje-cache, a redis based caching library ([`6710502`](https://github.com/z0w13/tulpje/commit/6710502612beb7e00fd5324502f6ea55bd4b0ea7))
</details>
<!-- generated by git-cliff -->
