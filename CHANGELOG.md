# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.14.1] - 2025-01-13

### Changed

 - Clean up emoji stats on GuildCreate and GuildEmojisUpdate events

### Fixed

 - Validate the emoji stats embed
 - Don't show pagination/sorting when emoji stats are empty

### Removed

 - Remove emoji stats cleanup task, handled on event now

### Commit Details

<details><summary>view details</summary>

 * fix(handler): validate the emoji stats embed ([`f5eb65d`](https://github.com/z0w13/tulpje/commit/f5eb65d4ff2fac3b5ef4cd81b971ff19f83b17bf))
 * fix(handler): don't show pagination/sorting when emoji stats are empty ([`ff91d8b`](https://github.com/z0w13/tulpje/commit/ff91d8b1c1cb8bc8870410b07d761f55297cd5fb))
 * chore(handler): remove emoji stats cleanup task, handled on event now ([`a7cf7ea`](https://github.com/z0w13/tulpje/commit/a7cf7ea8afad98ac8b77ad2a96e1787c21a62386))
 * feat(handler): clean up emoji stats on GuildCreate and GuildEmojisUpdate events ([`3b5b3a9`](https://github.com/z0w13/tulpje/commit/3b5b3a9cd27e7569edc07698a4985022fcb4ebac))
</details>

## [0.14.0] - 2025-01-13

### Added

 - Added manual and automatic removal of emoji stats for deleted emojis
 - Add missing commas in RELEASE_FILENAME_MATCHLIST_WORKSPACE

### Changed

 - Implement pagination for `/emoji stats`
 - Split modules::stats into multiple files
 - Implement fallback for /stats when we can't get stats from redis
 - Split core module into multiple files

### Fixed

 - Always source .env from project root
 - Don't hardcode independent crates in RELEASE_FILENAME_MATCHLIST_WORKSPACE
 - Use latest main tag (vX.Y.Z) in push.sh

### Commit Details

<details><summary>view details</summary>

 * fix(build): always source .env from project root ([`ce72ec8`](https://github.com/z0w13/tulpje/commit/ce72ec846e332ce3d1d9c6cc2ecd202d2cd41eee))
 * feat(handler): added manual and automatic removal of emoji stats for deleted emojis ([`79dc31c`](https://github.com/z0w13/tulpje/commit/79dc31ceb9d16e38ee20da519a574fa0f1db774e))
 * feat(handler): implement pagination for `/emoji stats` ([`f6e120a`](https://github.com/z0w13/tulpje/commit/f6e120a6733c08c4699baaef58cf139c1812468c))
 * refactor(handler): split modules::stats into multiple files ([`ab4da15`](https://github.com/z0w13/tulpje/commit/ab4da1535c6d82d47c8a4bac6ef3a647941538c7))
 * feat(handler): implement fallback for /stats when we can't get stats from redis ([`6dca9e7`](https://github.com/z0w13/tulpje/commit/6dca9e704c7342772e9c98055bbd1638c31803c6))
 * refactor(handler): split core module into multiple files ([`6315247`](https://github.com/z0w13/tulpje/commit/6315247b33664ec520e0ba770fea5da3b31aa70f))
 * fix(build): don't hardcode independent crates in RELEASE_FILENAME_MATCHLIST_WORKSPACE ([`d2e36d9`](https://github.com/z0w13/tulpje/commit/d2e36d9dba95bd6b18ba13c935442a5f2a772bd3))
 * fix(build): add missing commas in RELEASE_FILENAME_MATCHLIST_WORKSPACE ([`fbbe9c3`](https://github.com/z0w13/tulpje/commit/fbbe9c375f80c0031160542bf1c3c0fd424c3266))
 * fix(build): use latest main tag (vX.Y.Z) in push.sh ([`13c3495`](https://github.com/z0w13/tulpje/commit/13c34955c5c67c72473262d8f6b5e9a14ed2e53e))
</details>

## [0.13.0] - 2025-01-12

### Breaking Changes

 - Use subcommands and subcommand groups
 - Added support for subcommands and subcommand groups

### Changed

 - Specify GitHub release title

### Fixed

 - Reset minor/patch levels when bumping versions

### Commit Details

<details><summary>view details</summary>

 * fix(build): reset minor/patch levels when bumping versions ([`cc7bc4d`](https://github.com/z0w13/tulpje/commit/cc7bc4d9c884ec3ab3aee937d33e09dc5290f244))
 * feat(handler)!: use subcommands and subcommand groups ([`254c83e`](https://github.com/z0w13/tulpje/commit/254c83e6fbfa9d906145c1f5199c365adde84710))
 * feat(framework)!: added support for subcommands and subcommand groups ([`007709f`](https://github.com/z0w13/tulpje/commit/007709f5a27dfa44e961653d134ece2e4820f3e1))
 * build: specify GitHub release title ([`3198eea`](https://github.com/z0w13/tulpje/commit/3198eeaa2eecee28c1bef92f0f33b59d61b7edc5))
</details>

## [0.12.1] - 2025-01-12

### Changed

 - Don't use \`cross\` for compiling to x86_64-unknown-linux-musl

### Fixed

 - Exit when we receive an empty message from the shard
 - Fix should_release not being taken into account when releasing
 - Version bump didn't take `cargo semver-checks` into account
 - Create_changelog_update accepts Iterable[CrateInfo]

### Commit Details

<details><summary>view details</summary>

 * fix(gateway): exit when we receive an empty message from the shard ([`3cbf899`](https://github.com/z0w13/tulpje/commit/3cbf899320c251ff6fa3c75f81fd52f83e5e1267))
 * build: don't use \`cross\` for compiling to x86_64-unknown-linux-musl ([`ff5d84c`](https://github.com/z0w13/tulpje/commit/ff5d84c6ae34969b36f5201487f450b7cb72b4d5))
 * fix(build): fix should_release not being taken into account when releasing ([`73a3e9e`](https://github.com/z0w13/tulpje/commit/73a3e9e62ef479ecb0ff0fe0ccc2a9f5631b24a4))
 * fix(build): version bump didn't take `cargo semver-checks` into account ([`a008a8c`](https://github.com/z0w13/tulpje/commit/a008a8cc4895f540969730fb806f4de429690944))
 * fix(build): create_changelog_update accepts Iterable[CrateInfo] ([`c7e2632`](https://github.com/z0w13/tulpje/commit/c7e26329a51691c889652bd67d46a6709b694de2))
</details>

## [0.12.0] - 2025-01-08

### Breaking Changes

 - Move sqlx data to the tulpje-handler crate as they're part of that anyway

### Added

 - Add release tooling
 - Add CHANGELOG.md

### Fixed

 - Set a default for HANDLER_COUNT and don't override SHARD_COUNT from .env
 - Revert "don't clear target/release, unneeded after removal of amqp feature"
 - Fix crash when unable to parse gateway payload, log error instead
 - Use fork of pkrs that's actually published to crates.io

### Commit Details

<details><summary>view details</summary>

 * build: add release tooling ([`417cbcf`](https://github.com/z0w13/tulpje/commit/417cbcf0fbd08b3c05a4014ca7e883b9dce7cf54))
 * docs: add CHANGELOG.md ([`0d202e7`](https://github.com/z0w13/tulpje/commit/0d202e7782b924955891541eb824b1026104be14))
 * fix(build): set a default for HANDLER_COUNT and don't override SHARD_COUNT from .env ([`67612cc`](https://github.com/z0w13/tulpje/commit/67612cc65241772bcdd745b1797d1ca3b4b409fa))
 * fix: revert "don't clear target/release, unneeded after removal of amqp feature" ([`1043929`](https://github.com/z0w13/tulpje/commit/1043929ac418d522212a067397a33317cee7ae5b))
 * fix(handler): fix crash when unable to parse gateway payload, log error instead ([`ca5d4ff`](https://github.com/z0w13/tulpje/commit/ca5d4ffe5e28f555db02f3328856ff1acd2f3db6))
 * chore!: move sqlx data to the tulpje-handler crate as they're part of that anyway ([`bf3bdb5`](https://github.com/z0w13/tulpje/commit/bf3bdb57c3d2b7361a64bd7a1a2a2c617016d9c0))
 * fix(handler): use fork of pkrs that's actually published to crates.io ([`b94cac6`](https://github.com/z0w13/tulpje/commit/b94cac6280e85f3c8fa10ee28fa3a79d6c1cc6aa))
</details>

## [0.11.0] - 2025-01-05

### Breaking Changes

- Move DisordEventMeta to tulpje-framework and rename it Metadata
- Rewrite the deploy and push scripts to use bash
- Remove features to choose amqp implementation, just use amqprs

### Added

- Add version constraints to workspace dependencies
- Add missing package metadata

### Changed

- Mark all crates as publishable
- Don't make main() return Result, use .expect() to add info to errors
- Implement additional metrics and show them in /processes
- Implement version!() macro to get version from vergen env vars
- Simplify MetricsManager and move the tokio::spawn call outside of it
- Implement and use ToRedisArgs/FromRedisValue for ShardState and Metrics
- Allow specifying image tag using IMAGE_TAG= in push.sh
- Don't clear target/release, unneeded after removal of amqp feature

### Fixed

- Don't rebuild if migrations change
- Check mutually-exclusive features in build.rs

### Removed

- Remove redlight from RUST_LOG in .example.env
- Remove remnants of never implemented cache feature

### Commit Details

<details><summary>view details</summary>

- add version constraints to workspace dependencies ([`7fe31e8`](https://github.com/z0w13/tulpje/commit/7fe31e8008da07e0ee9ce47ef813a0002c3ff049))
- mark all crates as publishable ([`3867bf6`](https://github.com/z0w13/tulpje/commit/3867bf60346a8391d98081d2370015ce6ec2d891))
- don't make main() return Result, use .expect() to add info to errors ([`7fc1eb0`](https://github.com/z0w13/tulpje/commit/7fc1eb02c7ef9db4e037c9ebc0e83a1744b45ece))
- move DisordEventMeta to tulpje-framework and rename it Metadata ([`e1e93d7`](https://github.com/z0w13/tulpje/commit/e1e93d7903ff7a48066214ca002554ce91e4a9b3))
- add missing package metadata ([`7a9e193`](https://github.com/z0w13/tulpje/commit/7a9e1939cf7fad86b6234612934413139d82e936))
- implement additional metrics and show them in /processes ([`d4c9863`](https://github.com/z0w13/tulpje/commit/d4c986380f0ff01e101c46db8ed14e11e23cb869))
- implement version!() macro to get version from vergen env vars ([`b4d8ecc`](https://github.com/z0w13/tulpje/commit/b4d8ecccc76c446cdd2134c7d0901baf4c9f5b36))
- simplify MetricsManager and move the tokio::spawn call outside of it ([`4445dac`](https://github.com/z0w13/tulpje/commit/4445dace20c41e630fc4477c23c2689bb46e151a))
- implement and use ToRedisArgs/FromRedisValue for ShardState and Metrics ([`c0a020f`](https://github.com/z0w13/tulpje/commit/c0a020f68672467c78e6d10cce2d43756ce1b303))
- allow specifying image tag using IMAGE_TAG= in push.sh ([`233c7ee`](https://github.com/z0w13/tulpje/commit/233c7eef0d8805f6a87f2ca98c336a44252e28cc))
- remove redlight from RUST_LOG in .example.env ([`c91c0a2`](https://github.com/z0w13/tulpje/commit/c91c0a2cb0afb4dbb9b30ce174beeca680cd00d1))
- rewrite the deploy and push scripts to use bash ([`efbf1ab`](https://github.com/z0w13/tulpje/commit/efbf1ab7cc453e2e480d639b30cdf574e486d813))
- don't clear target/release, unneeded after removal of amqp feature ([`1a9671c`](https://github.com/z0w13/tulpje/commit/1a9671c59f2702f019ae8df253a4702efe966e8b))
- remove remnants of never implemented cache feature ([`7124405`](https://github.com/z0w13/tulpje/commit/71244050bd6a2977b5c92b07a8f177bad95624f8))
- remove features to choose amqp implementation, just use amqprs ([`872bbca`](https://github.com/z0w13/tulpje/commit/872bbcaed7e671d7e12d434502410c51bb143690))
- don't rebuild if migrations change ([`69e7cd1`](https://github.com/z0w13/tulpje/commit/69e7cd11694e7a30c32580f02ef4a4dec3ba066f))
- check mutually-exclusive features in build.rs ([`ecc9550`](https://github.com/z0w13/tulpje/commit/ecc9550605aea771cd3717b1c5d55142c1269575))
</details>

## [0.10.0] - 2025-01-05

### Fixed

- Don't source SHARD_COUNT/HANDLER_COUNT from .env
- Only run tasks on the "primary" handler (handler_id=0)
- Only register commands on the "primary" handler (handler_id=0)
- SHARD_ID and HANDLER_ID should actually be 0 not 1 by default
- Better error messages in framework setup function

### Commit Details

<details><summary>view details</summary>

- don't source SHARD_COUNT/HANDLER_COUNT from .env ([`8f1360a`](https://github.com/z0w13/tulpje/commit/8f1360ac4cd9ed2aa3c9867bbb39d24a31cfb4c6))
- only run tasks on the "primary" handler (handler_id=0) ([`743d701`](https://github.com/z0w13/tulpje/commit/743d701b3c4681e81ced0820f76cd5863dfb4d6e))
- only register commands on the "primary" handler (handler_id=0) ([`c6e4fb5`](https://github.com/z0w13/tulpje/commit/c6e4fb59d58104371c501e0d351ef9b017ca4319))
- SHARD_ID and HANDLER_ID should actually be 0 not 1 by default ([`053221e`](https://github.com/z0w13/tulpje/commit/053221edeb1a24e0224ab8b9c3ae498a6b433c3c))
- better error messages in framework setup function ([`f006ca1`](https://github.com/z0w13/tulpje/commit/f006ca1256e71fa371b1bc87f9035d72ee6424ca))
</details>

## [0.9.0] - 2025-01-05

### Breaking Changes

- Rework sending messages into framework, and refactor Scheduler to follow similar conventions to Dispatch

### Added

- Add error message if .current_user_application() fails

### Fixed

- Use .expect() instead of ? in main for extra error info
- Set SHARD_ID and HANDLER_ID to 1 in run-local.sh
- Don't fetch process metrics twice, and correctly calculate cpu/mem usage

### Commit Details

<details><summary>view details</summary>

- use .expect() instead of ? in main for extra error info ([`99b4999`](https://github.com/z0w13/tulpje/commit/99b4999ca8026bfd27705c2aeffe0655d8a014c8))
- rework sending messages into framework, and refactor Scheduler to follow similar conventions to Dispatch ([`08bf914`](https://github.com/z0w13/tulpje/commit/08bf9145d5a412fffd3f489c4667f51f879ae4c1))
- set SHARD_ID and HANDLER_ID to 1 in run-local.sh ([`c3ed97b`](https://github.com/z0w13/tulpje/commit/c3ed97bbf37318e7e4427ac71e18278ac4ea7d2a))
- add error message if .current_user_application() fails ([`a4710eb`](https://github.com/z0w13/tulpje/commit/a4710eb06d0d05f86773ce4251392271018fecad))
- don't fetch process metrics twice, and correctly calculate cpu/mem usage ([`056b691`](https://github.com/z0w13/tulpje/commit/056b6917f6389f507c7c216c00c4c44288dd0750))
</details>

## [0.8.0] - 2025-01-05

### Breaking Changes

- Implement framework with main loop and shutdown functionality

### Fixed

- Allow configuring handler count in deploy script

### Commit Details

<details><summary>view details</summary>

- allow configuring handler count in deploy script ([`fe607ff`](https://github.com/z0w13/tulpje/commit/fe607ffe5d63de792c02d6138a54aa26c84b5530))
- implement framework with main loop and shutdown functionality ([`891be50`](https://github.com/z0w13/tulpje/commit/891be50d55ef9869a0f10b48c1f50f0bc0181cd9))
</details>

## [0.7.0] - 2025-01-05

### Added

- Add explicit scaling support and store the handler count/id

### Changed

- Implement /processes for bot process stats and re-implement cpu/mem usage for /stats
- Implement basic memory/cpu usage tracking for bot processes
- Enable clippy::missing_assert_message
- Enable clippy::mod_module_files

### Fixed

- SHARD_ID env var should be uppercase
- Enable clippy::explicit_iter_loop clippy::explicit_into_iter_loop and fix warnings
- Wrap Registry in an Arc to avoid expensive `clone` operations
- Enable clippy::redundant_closure and fix warnings
- Enable clippy::or_fun_call and fix warnings
- Enable clippy::option_if_let_else and fix warnings
- Enable clippy::if_then_some_else_none and fix warnings
- Enable clippy::match_bool and fix warnings
- Enable clippy::indexing_slicing and fix warnings
- Enable clippy::manual_assert and fix warnings
- Enable clippy::redundant_else and fix warnings
- Enable clippy::partial_pub_fields and fix warnings
- Enable clippy::unwrap_in_result and fix warnings
- Enable clippy::cast_lossless and clippy::cast_precision_loss and fix warnings
- Enable clippy::integer_division and fix warnings
- Enable clippy::unneeded_field_pattern and fix warnings
- Enable clippy::get_unwrap and fix warnings
- Enable clippy::ignored_unit_patterns and fix warnings
- Enable clippy::clone_on_ref_ptr and fix warnings
- Enable clippy::needless_for_each and fix warnings
- Enable clippy::redundant_clone and fix warnings
- Enable clippy::renamed_function_params and fix warnings
- Enable clippy::use_self and fix warnings
- Enable clippy::needless_pass_by_value and fix warnings
- Enable clippy::from_iter_instead_of_collect and fix warnings
- Enable clippy::manual_string_new and fix warnings
- Enable clippy::allow_attributes and fix warnings
- Enable clippy::implicit_clone and fix warnings
- Enable clippy::unused_trait_names and fix warnings
- Use assert! instead of assert_eq! if checking for true/false
- Enable clippy::semicolon_if_nothing_returned and fix warnings

### Removed

- Remove outdated comment

### Commit Details

<details><summary>view details</summary>

- implement /processes for bot process stats and re-implement cpu/mem usage for /stats ([`e8dd738`](https://github.com/z0w13/tulpje/commit/e8dd738745c4ca3b37ba3b540c962ac5495328dd))
- implement basic memory/cpu usage tracking for bot processes ([`529805a`](https://github.com/z0w13/tulpje/commit/529805a9a01a28f6e9fdee07c2c72467389c111e))
- add explicit scaling support and store the handler count/id ([`3263933`](https://github.com/z0w13/tulpje/commit/32639337609e7cb776c5c2458a2e376444b3f130))
- SHARD_ID env var should be uppercase ([`4892373`](https://github.com/z0w13/tulpje/commit/48923734af123dce5ab2cee14698b23ec1cf7e54))
- enable clippy::explicit_iter_loop clippy::explicit_into_iter_loop and fix warnings ([`604f7bc`](https://github.com/z0w13/tulpje/commit/604f7bc411afdcdd2ccfa9be85379e110a92392c))
- wrap Registry in an Arc to avoid expensive `clone` operations ([`0622cd4`](https://github.com/z0w13/tulpje/commit/0622cd42582fb61741bb7572597de6a410a2f1f1))
- remove outdated comment ([`505c6c8`](https://github.com/z0w13/tulpje/commit/505c6c854013635860f75d08f42ad8694bface76))
- enable clippy::redundant_closure and fix warnings ([`6b941e6`](https://github.com/z0w13/tulpje/commit/6b941e61b069778159db01f847cbb0dc97863ec9))
- enable clippy::or_fun_call and fix warnings ([`7565a8a`](https://github.com/z0w13/tulpje/commit/7565a8a18f13c586cfff27a850085236b68be526))
- enable clippy::option_if_let_else and fix warnings ([`bc27650`](https://github.com/z0w13/tulpje/commit/bc27650b9d073b67488039fc1ccd6322d42b4ee3))
- enable clippy::if_then_some_else_none and fix warnings ([`794c6e1`](https://github.com/z0w13/tulpje/commit/794c6e11ca1b8f8b99915e198f8b8827968e467b))
- enable clippy::match_bool and fix warnings ([`6a6aa13`](https://github.com/z0w13/tulpje/commit/6a6aa13df964abba7d2aca804e01545dc982f833))
- enable clippy::indexing_slicing and fix warnings ([`6c5392b`](https://github.com/z0w13/tulpje/commit/6c5392b0ed443c5bf0678800e684b0d5e75040f7))
- enable clippy::missing_assert_message ([`58f1d6f`](https://github.com/z0w13/tulpje/commit/58f1d6fad78bfe418a1422e218c15db02b83b70e))
- enable clippy::manual_assert and fix warnings ([`8b672eb`](https://github.com/z0w13/tulpje/commit/8b672eba70f3046e5d0458b02d1f1417ad88afca))
- enable clippy::redundant_else and fix warnings ([`8e621a3`](https://github.com/z0w13/tulpje/commit/8e621a3dd4c76130a32d54c163eb05a68012f04a))
- enable clippy::partial_pub_fields and fix warnings ([`2155f3a`](https://github.com/z0w13/tulpje/commit/2155f3a4d6434c79205e6682f14bcd9c7a5e7932))
- enable clippy::unwrap_in_result and fix warnings ([`78e012a`](https://github.com/z0w13/tulpje/commit/78e012ac4d591e5cbc7934cdc0da2e1267d545da))
- enable clippy::cast_lossless and clippy::cast_precision_loss and fix warnings ([`c44fef5`](https://github.com/z0w13/tulpje/commit/c44fef5eca922c40bcee17c5463d6b4bd6e287b3))
- enable clippy::integer_division and fix warnings ([`52ff935`](https://github.com/z0w13/tulpje/commit/52ff935039759806b750a954b0b18d346fcad82b))
- enable clippy::unneeded_field_pattern and fix warnings ([`8fa41ee`](https://github.com/z0w13/tulpje/commit/8fa41eeb06e5149d7668f22e2dbe21faef8f4f51))
- enable clippy::get_unwrap and fix warnings ([`c5a1853`](https://github.com/z0w13/tulpje/commit/c5a1853355fb1839a49af3f37701f14e5ef490d8))
- enable clippy::ignored_unit_patterns and fix warnings ([`7002443`](https://github.com/z0w13/tulpje/commit/7002443123d98c5e723d13ad7bb35d68e55910bd))
- enable clippy::clone_on_ref_ptr and fix warnings ([`b457f62`](https://github.com/z0w13/tulpje/commit/b457f624fad3e8030262d980b2879fc7ccc71fc3))
- enable clippy::needless_for_each and fix warnings ([`a58c116`](https://github.com/z0w13/tulpje/commit/a58c116b1ed1ea3c5bff91f915c8ef61c5b02d91))
- enable clippy::redundant_clone and fix warnings ([`92e81e9`](https://github.com/z0w13/tulpje/commit/92e81e90362a5eb39625bef35487b06af8a20cc7))
- enable clippy::mod_module_files ([`d86c1cb`](https://github.com/z0w13/tulpje/commit/d86c1cb074e9cc79de1f915ac5a805c1dbfe8a02))
- enable clippy::renamed_function_params and fix warnings ([`25e49c6`](https://github.com/z0w13/tulpje/commit/25e49c6587b1b8f75424cabc292f99935cda90c0))
- enable clippy::use_self and fix warnings ([`07fb1dd`](https://github.com/z0w13/tulpje/commit/07fb1dd338208b3deb12b2162150504e8184751e))
- enable clippy::needless_pass_by_value and fix warnings ([`7e448c6`](https://github.com/z0w13/tulpje/commit/7e448c63bd57e2c3337b8dfb2618717f307ff368))
- enable clippy::from_iter_instead_of_collect and fix warnings ([`b1e2bc3`](https://github.com/z0w13/tulpje/commit/b1e2bc3bc32f909e7eb77de20edefd75f5aeb9ca))
- enable clippy::manual_string_new and fix warnings ([`ca680eb`](https://github.com/z0w13/tulpje/commit/ca680ebba3898e5e1c5890ab711d168ad4688347))
- enable clippy::allow_attributes and fix warnings ([`ed71f0f`](https://github.com/z0w13/tulpje/commit/ed71f0f24e6e188a8fbcce17921d2c261228ced7))
- enable clippy::implicit_clone and fix warnings ([`ac6aadb`](https://github.com/z0w13/tulpje/commit/ac6aadbe706f08c39a602cdfa69bdcae70eeb7df))
- enable clippy::unused_trait_names and fix warnings ([`d45c4c5`](https://github.com/z0w13/tulpje/commit/d45c4c541810dada3048441ca6635f0a5f9c83ef))
- use assert! instead of assert_eq! if checking for true/false ([`f8c6700`](https://github.com/z0w13/tulpje/commit/f8c6700a5231d17066ba5b3d127ee3b75dbf5e7c))
- enable clippy::semicolon_if_nothing_returned and fix warnings ([`a526faf`](https://github.com/z0w13/tulpje/commit/a526fafd635d3840b7eac26c8fe32bce923e7679))
</details>

## [0.6.0] - 2025-01-05

### Changed

- Don't hardcode guild module list
- Rework module system, registry, and task scheduler
- Cargo fmt

### Fixed

- Don't update fronters for guilds that don't have the pluralkit module enabled

### Commit Details

<details><summary>view details</summary>

- don't hardcode guild module list ([`6b8000e`](https://github.com/z0w13/tulpje/commit/6b8000e973e6a6d333b4bf83cd7d814d79a48871))
- don't update fronters for guilds that don't have the pluralkit module enabled ([`5e27447`](https://github.com/z0w13/tulpje/commit/5e27447b8422cff823a5e65d7adb6c4ad65079ee))
- rework module system, registry, and task scheduler ([`ba4aae2`](https://github.com/z0w13/tulpje/commit/ba4aae287376f7040b6798c30d7be4d6c0a12ed2))
- cargo fmt ([`4a2d7d8`](https://github.com/z0w13/tulpje/commit/4a2d7d8b1f29ed55553fb7f01f73f0499600d7fd))
</details>

## [0.5.0] - 2025-01-05

### Breaking Changes

- Don't pass context in constructor

### Changed

- Per-guild commands

### Fixed

- After defer we should use ctx.update
- Actually send user errors back to the user

### Commit Details

<details><summary>view details</summary>

- after defer we should use ctx.update ([`657d946`](https://github.com/z0w13/tulpje/commit/657d9467c231cc8190dc0c5a5bd8a2b7ac70c069))
- actually send user errors back to the user ([`367aa7f`](https://github.com/z0w13/tulpje/commit/367aa7f038a41512cd5f6ba8f5d21a8c41478544))
- per-guild commands ([`172d91c`](https://github.com/z0w13/tulpje/commit/172d91c8fe43e9ff7d8c46f02290712a28a7ea75))
- don't pass context in constructor ([`473c7d8`](https://github.com/z0w13/tulpje/commit/473c7d81c351f0cd2d7c16af747c30bb22d0b74c))
</details>

## [0.4.2] - 2025-01-05

### Added

- Add targets and feature permutations to contrib/check.sh
- Add metrics to gateway and handler

### Changed

- Implement emoji cloning on right-click menu
- Use futures_util instead of futures
- Use custom IdentifyProperties identifying as tulpje
- Use Config::presence to set the presence, instead of manually on first ready
- Update README with additional info

### Fixed

- Guild IDs can't be 0, so dummy ID should be 1 at least
- Initialise cache before redis connection to avoid borrow issue

### Removed

- Remove Cargo.lock from crate, should only be in root

### Commit Details

<details><summary>view details</summary>

- implement emoji cloning on right-click menu ([`d774d01`](https://github.com/z0w13/tulpje/commit/d774d01c74ab71faef9c5d737391e023d3c7ae2d))
- guild IDs can't be 0, so dummy ID should be 1 at least ([`f25d504`](https://github.com/z0w13/tulpje/commit/f25d50460be954a24fbf88f64d10888e5a8a5d27))
- feat!(gateway): remove incomplete/broken cache feature ([`117f61b`](https://github.com/z0w13/tulpje/commit/117f61b99929be42be3a845e832a4fb1340496af))
- add targets and feature permutations to contrib/check.sh ([`1b50bd4`](https://github.com/z0w13/tulpje/commit/1b50bd4af82a6f784646fe9a84fc849422c4d4be))
- initialise cache before redis connection to avoid borrow issue ([`1406a39`](https://github.com/z0w13/tulpje/commit/1406a398417675b6f782394c77e510a6555e07a4))
- add metrics to gateway and handler ([`c42eab0`](https://github.com/z0w13/tulpje/commit/c42eab0c887c8e9b9410108e0d34237f37b78477))
- use futures_util instead of futures ([`f3b92d9`](https://github.com/z0w13/tulpje/commit/f3b92d9d776c16ab404bf1da0a29a82a852f86ec))
- use custom IdentifyProperties identifying as tulpje ([`46b4f74`](https://github.com/z0w13/tulpje/commit/46b4f7440ca1839f61bd3652bd5feddfb8dae82d))
- use Config::presence to set the presence, instead of manually on first ready ([`f865329`](https://github.com/z0w13/tulpje/commit/f8653291916e7df12b61d883c4aabdb42fb8a467))
- remove Cargo.lock from crate, should only be in root ([`b6a8ce9`](https://github.com/z0w13/tulpje/commit/b6a8ce9e1d41663a9e9a59b67a4a937e1e652c5e))
- update README with additional info ([`cc0efb8`](https://github.com/z0w13/tulpje/commit/cc0efb8a5267274059056b343b6487962097730c))
</details>

## [0.4.1] - 2025-01-05

### Added

- Added script to build and push images
- Add RUSTSEC-2024-0384 to cargo-audit ignore list

### Changed

- Script to run through a bunch of checks, useful before tag/deploy/etc
- Features to pick between lapin and amqprs for amqp implementation
- Update miniz_oxide to 0.8.2 version as 0.8.1 was yanked
- Rename docker-compose.yml to compose.yml
- Pass around twilight_model::id::Id instead of raw u64 values
- We're just called Tulpje now

### Fixed

- Ignore rsa advisory as we don't use it
- Specify name in docker-compose.yml otherwise some scripts will break if folder has a different name

### Commit Details

<details><summary>view details</summary>

- added script to build and push images ([`9525c8b`](https://github.com/z0w13/tulpje/commit/9525c8b9fd09ad471a1ed4d99b7de3f3ad32e14f))
- script to run through a bunch of checks, useful before tag/deploy/etc ([`f9ead9b`](https://github.com/z0w13/tulpje/commit/f9ead9b81d841b2595592a25501eb64f035d5cae))
- features to pick between lapin and amqprs for amqp implementation ([`511f588`](https://github.com/z0w13/tulpje/commit/511f588da01f9c286d920f936ed801bdfbc99b4a))
- add RUSTSEC-2024-0384 to cargo-audit ignore list ([`57c1982`](https://github.com/z0w13/tulpje/commit/57c198229f900d11a67d2942a9034a5cdb08585a))
- ignore rsa advisory as we don't use it ([`d9f291a`](https://github.com/z0w13/tulpje/commit/d9f291a984d41863f6144e193bdeb7687b564789))
- update miniz_oxide to 0.8.2 version as 0.8.1 was yanked ([`8ce0162`](https://github.com/z0w13/tulpje/commit/8ce01622d9040dccc2cf513ba586401e34e5a943))
- rename docker-compose.yml to compose.yml ([`9f4c9fd`](https://github.com/z0w13/tulpje/commit/9f4c9fd4e044c048406188a5f9f32cd0361d2b32))
- pass around twilight_model::id::Id instead of raw u64 values ([`264702a`](https://github.com/z0w13/tulpje/commit/264702aa4dca7fb04c5cf34edc796145494e4b7d))
- specify name in docker-compose.yml otherwise some scripts will break if folder has a different name ([`f077d22`](https://github.com/z0w13/tulpje/commit/f077d2219d626a3d982ec0b11a35705539cc26d8))
- we're just called Tulpje now ([`881d77b`](https://github.com/z0w13/tulpje/commit/881d77b0b7f4efff556fa5b9b033ec53247aa86d))
</details>

## [0.4.0] - 2024-12-30

### Changed

- PluralKit module
- Task scheduling using cron syntax
- Suppress clippy::single_match warning
- Implement emoji cloning
- Macros for making registering handlers slightly nicer
- Implemented basic command and event handling framework

### Fixed

- Don't specify a version for workspace packages, they're in sync anyway
- Don't create a twilight_http::Client that we never use
- Thread safetey ugh headaches

### Commit Details

<details><summary>view details</summary>

- don't specify a version for workspace packages, they're in sync anyway ([`8c37a35`](https://github.com/z0w13/tulpje/commit/8c37a35531f97f3cc09b593f369d43c277f762d3))
- PluralKit module ([`eeb11e5`](https://github.com/z0w13/tulpje/commit/eeb11e5faf20f394a7a2e350c78706f152f85187))
- task scheduling using cron syntax ([`dbd42cb`](https://github.com/z0w13/tulpje/commit/dbd42cb547620d5c9a79b4618bcd87ac842629e6))
- don't create a twilight_http::Client that we never use ([`6e237d9`](https://github.com/z0w13/tulpje/commit/6e237d94c72ea13ffc019119c8d5eb21e3f553c6))
- suppress clippy::single_match warning ([`5982423`](https://github.com/z0w13/tulpje/commit/598242303275dbf915f2c3a8fd224bc6b837de12))
- thread safetey ugh headaches ([`84c6eab`](https://github.com/z0w13/tulpje/commit/84c6eab779e30ca2f84aec3360f6a74abda611aa))
- implement emoji cloning ([`ea0d6a0`](https://github.com/z0w13/tulpje/commit/ea0d6a09bdacc4b40f218255eba332e461f69bb6))
- macros for making registering handlers slightly nicer ([`178e4b7`](https://github.com/z0w13/tulpje/commit/178e4b7b6c0f0a4df8469944038a2cf742a9e96a))
- implemented basic command and event handling framework ([`cde4d29`](https://github.com/z0w13/tulpje/commit/cde4d2940656156c0b1d1d5754b6de8e3139ed31))
</details>

## [0.3.0] - 2024-12-20

### Breaking Changes

- Split stats commands into its own module

### Changed

- Bump version to 0.3.0
- Expand env vars when creating secrets
- Implement emoji usage tracking and /emoji-stats
- Update to twilight 0.16.0-rc.1

### Fixed

- Clippy warnings

### Commit Details

<details><summary>view details</summary>

- bump version to 0.3.0 ([`9f1e681`](https://github.com/z0w13/tulpje/commit/9f1e681aae1909bb7e6a1fd75c9e4b16f084c6e4))
- expand env vars when creating secrets ([`645e8ad`](https://github.com/z0w13/tulpje/commit/645e8addd05cc8df78f964bcf55ea492ea81be7b))
- implement emoji usage tracking and /emoji-stats ([`f524787`](https://github.com/z0w13/tulpje/commit/f524787a3a8ef5a640826a2496ea43ca54d9afab))
- update to twilight 0.16.0-rc.1 ([`7c4f0ac`](https://github.com/z0w13/tulpje/commit/7c4f0ac0652d12345cd47b07e1c81cfbf52023fa))
- split stats commands into its own module ([`a7e80f0`](https://github.com/z0w13/tulpje/commit/a7e80f0f96469ac9bbed00149a0bb5d922cc1856))
- clippy warnings ([`e82a3b3`](https://github.com/z0w13/tulpje/commit/e82a3b3b439404ab60f3a57703b14ca33bad6f09))
</details>

## [0.2.0] - 2024-12-20

### Added

- Add heartbeat_interval in ShardState, add ShardState::is_up() for better determining if shard is up
- Add .env.example

### Changed

- Bump version to 0.2.0
- Inherit package version from workspace
- Implement tests for ShardState::is_up()
- Use ShardState::is_up() to display whether shards are up
- Show per-shard guild count in /shards
- Show guild count in /stats
- Track guilds shard is in, and store count in ShardState
- Implement /stats and /shards commands
- Store shard state in redis
- Implement docker swarm deployment
- Contrib/run-local.sh utility that sets service IPs correctly for local dev (not in container)
- Run in scratch containers
- Store latency info in redis
- Handle Ready event and setting presence correctly
- Get shard_id/shard_count from env vars
- Also track shard_id in DiscordEvent
- Use serde_envfile and dotenvy for config and env parsing
- Use upstream rabbitmq image
- Implement using twilight-rs/gateway-queue for session rate limiting
- Improve comments and logging
- Use serde_envfile and dotenvy for config and env parsing
- Implement rudimentary gateway and handler processes
- Define services we depend on and their healthchecks

### Fixed

- Use base rabbitmq image and don't expose management port
- Specify user in pg_isready in postgres healthcheck

### Removed

- Remove unused import

### Commit Details

<details><summary>view details</summary>

- bump version to 0.2.0 ([`2278321`](https://github.com/z0w13/tulpje/commit/22783218d286d12c8b5c71202b02d79a9324bf34))
- inherit package version from workspace ([`ad71c47`](https://github.com/z0w13/tulpje/commit/ad71c47031cb6cf69122639a652df39a614dedb1))
- implement tests for ShardState::is_up() ([`d28088e`](https://github.com/z0w13/tulpje/commit/d28088eed8b3e72893dff30640c26a4dcb4f5733))
- use ShardState::is_up() to display whether shards are up ([`66855e1`](https://github.com/z0w13/tulpje/commit/66855e1528e83a966f4729bc762db96a92b75638))
- add heartbeat_interval in ShardState, add ShardState::is_up() for better determining if shard is up ([`f029fb4`](https://github.com/z0w13/tulpje/commit/f029fb4a9366f4fe512148401cf4785ed03ce07e))
- show per-shard guild count in /shards ([`b717710`](https://github.com/z0w13/tulpje/commit/b71771094d2a98bd96831b637a483d281797396c))
- show guild count in /stats ([`37de1b1`](https://github.com/z0w13/tulpje/commit/37de1b15a55362b205bc4b7e517c12d8480ad028))
- track guilds shard is in, and store count in ShardState ([`2e570f7`](https://github.com/z0w13/tulpje/commit/2e570f75d056c672f9581b37b94d4f8c17223084))
- implement /stats and /shards commands ([`e2ca481`](https://github.com/z0w13/tulpje/commit/e2ca481606f70cbc689c8d7522a97b42fe151944))
- store shard state in redis ([`41873d3`](https://github.com/z0w13/tulpje/commit/41873d3c04116d8af558029b7e7c60d07d3e6e0b))
- add .env.example ([`23af052`](https://github.com/z0w13/tulpje/commit/23af0524d352f063759fc3e8a7da5a7dee8e3253))
- implement docker swarm deployment ([`6549a08`](https://github.com/z0w13/tulpje/commit/6549a08fca5c8ab66d5cef9fbfcc9b3f831dabea))
- contrib/run-local.sh utility that sets service IPs correctly for local dev (not in container) ([`a58847f`](https://github.com/z0w13/tulpje/commit/a58847f667cdeaf2b04c33e6773a864cf239dc94))
- run in scratch containers ([`6bf243d`](https://github.com/z0w13/tulpje/commit/6bf243db1c820b2dc76caaba3b3b5b04861e2ce5))
- use base rabbitmq image and don't expose management port ([`193697a`](https://github.com/z0w13/tulpje/commit/193697a370bbe50ff9fcd054fd2bca454ace6cfb))
- specify user in pg_isready in postgres healthcheck ([`2f5b19f`](https://github.com/z0w13/tulpje/commit/2f5b19fe6b87c21524a19c689058916735f9fd8d))
- store latency info in redis ([`54b610b`](https://github.com/z0w13/tulpje/commit/54b610b916081966cebf76938b1e1532f16d1d3e))
- handle Ready event and setting presence correctly ([`ab9b0d9`](https://github.com/z0w13/tulpje/commit/ab9b0d927fa52cf817a6f43e340837f86ebbb641))
- get shard_id/shard_count from env vars ([`150d8fd`](https://github.com/z0w13/tulpje/commit/150d8fd4ff62a7790347b9bcba56047f2c8560c8))
- also track shard_id in DiscordEvent ([`71be200`](https://github.com/z0w13/tulpje/commit/71be200aa507d1c9d8fa524ae3ecff6357cbc2a7))
- use serde_envfile and dotenvy for config and env parsing ([`41e369e`](https://github.com/z0w13/tulpje/commit/41e369e66877b1d2b4bc51f8b977a27192d41f05))
- use upstream rabbitmq image ([`1eb70cb`](https://github.com/z0w13/tulpje/commit/1eb70cbbce9a03c4976f5313053ee0c7d01d386a))
- implement using twilight-rs/gateway-queue for session rate limiting ([`dc2d867`](https://github.com/z0w13/tulpje/commit/dc2d8677b9f39ff2f0a0c4d1a4775c4cc3af786e))
- improve comments and logging ([`9e3e651`](https://github.com/z0w13/tulpje/commit/9e3e65160f2b3229ba8d5eb2f49342386b4f326e))
- remove unused import ([`bdd901d`](https://github.com/z0w13/tulpje/commit/bdd901d39f089f1f8f6cacf1b8657d51c436447a))
- use serde_envfile and dotenvy for config and env parsing ([`416e27b`](https://github.com/z0w13/tulpje/commit/416e27be73cdeb9aa4fd6cad954e3dc7715b1774))
- implement rudimentary gateway and handler processes ([`d719997`](https://github.com/z0w13/tulpje/commit/d7199978de6d2fcbd7915c50faae5bf14a514318))
- define services we depend on and their healthchecks ([`4d0d8ea`](https://github.com/z0w13/tulpje/commit/4d0d8ea85ef0e2057b6806ab75dc4b3501372a79))
</details>
<!-- generated by git-cliff -->
