# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.2.0] - 2025-11-15

### Changed

 - Make `tokio-util` a workspace dependency
 - Bump `amqprs` from 2.1.2 to 2.1.3
 - Make `tokio` a workspace dependency
 - Bump `async-trait` from 0.1.86 to 0.1.89
 - Bump tokio from 1.44.2 to 1.47.1
 - Bump tokio-util from 0.7.14 to 0.7.16
 - Update to rust 1.90.0, fix lint warnings, `cargo fmt`
 - `clippy::collapsible_if`
 - `cargo fmt`
 - Specify edition on workspace level
 - Update amqprs from 2.1.0 to 2.1.2

### Fixed

 - Pin amqprs version due to trait changes

### Commit Details

<details><summary>view details</summary>

 * chore(deps): make `tokio-util` a workspace dependency ([`ebd429b`](https://github.com/z0w13/tulpje/commit/ebd429be53f9df0477154aaad445bbcdab9f28c9))
 * chore(deps): bump `amqprs` from 2.1.2 to 2.1.3 ([`1dd263a`](https://github.com/z0w13/tulpje/commit/1dd263add320bd115813717509400fbc37669c72))
 * chore(deps): make `tokio` a workspace dependency ([`1a2f46a`](https://github.com/z0w13/tulpje/commit/1a2f46aecb624069c466738891ba905e446a1637))
 * chore(deps): bump `async-trait` from 0.1.86 to 0.1.89 ([`0a60b47`](https://github.com/z0w13/tulpje/commit/0a60b47fca218c9e150987b146afecb0c0064f9e))
 * build(deps): bump tokio from 1.44.2 to 1.47.1 ([`44d256f`](https://github.com/z0w13/tulpje/commit/44d256fe7de24db17b823dd5f67176e278e7f154))
 * build(deps): bump tokio-util from 0.7.14 to 0.7.16 ([`b084877`](https://github.com/z0w13/tulpje/commit/b084877909230c68857b633a8581ce425a45f67a))
 * chore(build): update to rust 1.90.0, fix lint warnings, `cargo fmt` ([`4a93c3b`](https://github.com/z0w13/tulpje/commit/4a93c3be063b99cbf6f4cd773e4b6fcf60f0b9bc))
 * chore(lint): `clippy::collapsible_if` ([`5c9d89e`](https://github.com/z0w13/tulpje/commit/5c9d89e3def56d8672cfa5399ced073f28884e99))
 * chore: `cargo fmt` ([`e76d893`](https://github.com/z0w13/tulpje/commit/e76d893b5102eca310144ab258e79553cb5b2f41))
 * refactor(build): specify edition on workspace level ([`f5b7a79`](https://github.com/z0w13/tulpje/commit/f5b7a79c4d5c5051e9dc3cc8b0def19fe22c63a6))
 * chore(reconnecting-amqp/deps): update amqprs from 2.1.0 to 2.1.2 ([`7c76ee2`](https://github.com/z0w13/tulpje/commit/7c76ee2b256b1919b560b19bd4eb073ccb4f81f1))
 * fix(reconnecting-amqp/deps): pin amqprs version due to trait changes ([`cc94756`](https://github.com/z0w13/tulpje/commit/cc947560839a21f92fee0dd241bc9552ee6d15d9))
</details>

## [0.1.0] - 2025-04-21

### Changed

- Split `reconnecting-amqp` into separate crate

### Commit Details

<details><summary>view details</summary>

- split `reconnecting-amqp` into separate crate ([`9ea6bee`](https://github.com/z0w13/tulpje/commit/9ea6beeed957ee35f3184cdde44f96e604a323cc))
</details>
