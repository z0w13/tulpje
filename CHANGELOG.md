# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.22.0-rc.1] - 2026-03-28

### Added

 - Add workspace hack with `cargo-hakari`
 - Add a description to tulpje-shared
 - Add `hyperfine` package
 - Add more colored output
 - Add glob support to workspace member resolution in `release.py`
 - Add `--skip-slow` to `release.py` to speed up dry runs
 - Add `Deref` impl for `Color

### Changed

 - Rename `db` module to `db_id`
 - Convert `pk_guilds` to use `system_uuid` instead of `system_id` and add foreign key
 - Move migrations to repository root
 - Move Core module into `tulpje-mod-core` crate
 - Move Stats module into `tulpje-mod-stats` crate
 - Move `format_significant_duration` to `tulpje-lib`
 - Move `is_pk_proxy` to `tulpje-lib`
 - Move Emoji module into `tulpje-mod-emoji` crate
 - Split out command calls into separate function in `build.rs`
 - Move PluralKit module into `tulpje-mod-pluralkit` crate
 - Move code shared between modules into `tulpje-lib` crate
 - Rename `tulpje-shared` to `tulpje-common`
 - Move all crates into a crates/ subfolder
 - Use `wild` linker
 - Use `debug = "line-tables-only" to improve debug build times and size
 - Split log formatter into separate module
 - Turn `release.py` into a proper python package
 - Disable dependabot version updates, only use it for security
 - Set `publish` in each crate's `Cargo.toml`
 - Use the Color deref impl
 - Move `Color` struct and role colors into `tulpje-framework`
 - Point to new user guide

### Fixed

 - Properly add foreign keys to all databaes tables
 - Pass shard id to gateway queue
 - Rebuild `tulpje-handler` if migrations change
 - Fix rerun-if-changed paths in build.rs
 - Actually re-use the `cargoArtifacts` derivation
 - Don't use references to Color, it's `Copy` now
 - Derive `Copy` and `Clone` for `Color
 - Don't bubble up command error if it's already handled in the command handler

### Removed

 - Remove unused dependency `serde-json` from `tulpje-common`
 - Remove unused `tulpje-manager` crate

### Commit Details

<details><summary>view details</summary>

 * refactor(tulpje-lib): rename `db` module to `db_id` ([`7dab097`](https://github.com/tulpje/tulpje/commit/7dab097051816a811af96c60765796f8704ffae4))
 * fix(handler): properly add foreign keys to all databaes tables ([`a03bd6e`](https://github.com/tulpje/tulpje/commit/a03bd6e22728a69b7ef4b9b83927566cfec48459))
 * refactor(handler): convert `pk_guilds` to use `system_uuid` instead of `system_id` and add foreign key ([`b609dc4`](https://github.com/tulpje/tulpje/commit/b609dc458c172c3e2dc147f195cf89b35b5de167))
 * chore: move migrations to repository root ([`5d51a95`](https://github.com/tulpje/tulpje/commit/5d51a95c46fa51d55ba789f36fa851ec96ea4657))
 * fix(gateway): pass shard id to gateway queue ([`e5e9a2a`](https://github.com/tulpje/tulpje/commit/e5e9a2a677d7f951978048fd8a358964d6577090))
 * refactor: move Core module into `tulpje-mod-core` crate ([`3af4d9a`](https://github.com/tulpje/tulpje/commit/3af4d9ac77d5b99d036a59b87c9b00131d02c56a))
 * refactor: move Stats module into `tulpje-mod-stats` crate ([`2e8c598`](https://github.com/tulpje/tulpje/commit/2e8c59800b7bc2501501a316a67a8d52d1b77772))
 * chore(deps): remove unused dependency `serde-json` from `tulpje-common` ([`bb2cd8a`](https://github.com/tulpje/tulpje/commit/bb2cd8af78d7e5d07e2b4bee696b7580420af263))
 * refactor: move `format_significant_duration` to `tulpje-lib` ([`4440e1a`](https://github.com/tulpje/tulpje/commit/4440e1a0119a3e8dbdef0cadbf21e76e4a9c465c))
 * refactor: move `is_pk_proxy` to `tulpje-lib` ([`b5c9915`](https://github.com/tulpje/tulpje/commit/b5c9915018fae44eb02065bcbb639ffe5875872b))
 * refactor: move Emoji module into `tulpje-mod-emoji` crate ([`eb420b2`](https://github.com/tulpje/tulpje/commit/eb420b22cd33972ab1e4ac9315bdcf51be8bcc5b))
 * chore(build): add workspace hack with `cargo-hakari` ([`6dc409e`](https://github.com/tulpje/tulpje/commit/6dc409ed3ace68bc4e755842fae49deb3fb93baa))
 * chore: remove unused `tulpje-manager` crate ([`18ad132`](https://github.com/tulpje/tulpje/commit/18ad1322ea1dca0b82d4b75c469099d551557f82))
 * fix(build): rebuild `tulpje-handler` if migrations change ([`28740c6`](https://github.com/tulpje/tulpje/commit/28740c636741df35097da1d6ccf63157e9d88db8))
 * refactor(build): split out command calls into separate function in `build.rs` ([`30d72ac`](https://github.com/tulpje/tulpje/commit/30d72ac6fceed739c58addabbb8e1d327d17524f))
 * fix(build): fix rerun-if-changed paths in build.rs ([`f03f061`](https://github.com/tulpje/tulpje/commit/f03f0616a81cb38ad157a83c257224b86ea06929))
 * refactor: move PluralKit module into `tulpje-mod-pluralkit` crate ([`3f06556`](https://github.com/tulpje/tulpje/commit/3f06556ea7c07f6046d1f9cac529380b3d08df07))
 * refactor: move code shared between modules into `tulpje-lib` crate ([`ed80efe`](https://github.com/tulpje/tulpje/commit/ed80efeffadea4f87bbd065ddddec5d7d8dd828e))
 * refactor: rename `tulpje-shared` to `tulpje-common` ([`0a3f97d`](https://github.com/tulpje/tulpje/commit/0a3f97d11d43fc01fad73a6a594e9e0a7d7816d6))
 * chore: add a description to tulpje-shared ([`406a1a9`](https://github.com/tulpje/tulpje/commit/406a1a9e96efe36c58ef1b453b51cf6473ec171e))
 * refactor: move all crates into a crates/ subfolder ([`60c1de0`](https://github.com/tulpje/tulpje/commit/60c1de0204ef80f95e82ff897882917966f33027))
 * chore(build): use `wild` linker ([`9e4f298`](https://github.com/tulpje/tulpje/commit/9e4f298d751b9fc7ecfd8f1c7046486a23728368))
 * chore(build): use `debug = "line-tables-only" to improve debug build times and size ([`ba7d931`](https://github.com/tulpje/tulpje/commit/ba7d931d112d9a056e6b4e921f02ea4c2aec64fa))
 * fix(nix): actually re-use the `cargoArtifacts` derivation ([`98360d5`](https://github.com/tulpje/tulpje/commit/98360d5e2be635fb2062bbf66d6475e322c06a4d))
 * chore(nix): add `hyperfine` package ([`0488457`](https://github.com/tulpje/tulpje/commit/048845787d9be2d830fd41197127ca775fec20ce))
 * feat(tools/release-tulpje): add more colored output ([`b3bb48e`](https://github.com/tulpje/tulpje/commit/b3bb48e809581a4a78949a23ddffd748819b2b46))
 * refactor(tools/release-tulpje): split log formatter into separate module ([`b4d072b`](https://github.com/tulpje/tulpje/commit/b4d072bd6712d91ebcf596288c2ed62d9ea66554))
 * refactor(tools/release-tulpje): turn `release.py` into a proper python package ([`b3625fb`](https://github.com/tulpje/tulpje/commit/b3625fb8e6de115d05186b5eb11bad67afbf186f))
 * fix(build): add glob support to workspace member resolution in `release.py` ([`e42d72c`](https://github.com/tulpje/tulpje/commit/e42d72ca1cadc0af71518147033bdad013b4777f))
 * feat(build): add `--skip-slow` to `release.py` to speed up dry runs ([`cd472ec`](https://github.com/tulpje/tulpje/commit/cd472ec7df5f781c91917960f6c80061fdf9bf39))
 * chore(ci): disable dependabot version updates, only use it for security ([`6680102`](https://github.com/tulpje/tulpje/commit/66801022fbf5c18e86517b22f8fc218b6312b266))
 * chore: set `publish` in each crate's `Cargo.toml` ([`e6748e7`](https://github.com/tulpje/tulpje/commit/e6748e79d1c5e354cf4315d7cdf8361962718a84))
 * chore: use the Color deref impl ([`85639d1`](https://github.com/tulpje/tulpje/commit/85639d1b6c2f0f36a259c95a2d0e3169d66288fc))
 * fix(handler): don't use references to Color, it's `Copy` now ([`130265b`](https://github.com/tulpje/tulpje/commit/130265b6be4a47f88759e1d362dbb2aa01f3643a))
 * feat(framework): add `Deref` impl for `Color ([`f021b26`](https://github.com/tulpje/tulpje/commit/f021b26c37a31d06a51870da812a308984aeda89))
 * fix(framework): derive `Copy` and `Clone` for `Color ([`17d2796`](https://github.com/tulpje/tulpje/commit/17d2796a505ec4e68fe01b89062f5493a34a209a))
 * refactor: move `Color` struct and role colors into `tulpje-framework` ([`474807a`](https://github.com/tulpje/tulpje/commit/474807a168a3cdd502127d3da2ea0f87f48c81fd))
 * fix(framework): don't bubble up command error if it's already handled in the command handler ([`4a200b7`](https://github.com/tulpje/tulpje/commit/4a200b7acdde1be47fa2c45ee633f7986ca7b93c))
 * chore(docs): point to new user guide ([`2b8d9eb`](https://github.com/tulpje/tulpje/commit/2b8d9ebbad0e6a17476de9af44cf4ddd40c230d1))
</details>

## [0.21.5] - 2026-03-26

### Commit Details

<details><summary>view details</summary>

</details>

## [0.21.5-rc.1] - 2026-03-26

### Added

 - Add total system count

### Changed

 - Reduce sensitive information in log messages
 - Improve log message in `update_fronter_category

### Fixed

 - Only try to update fronters for systems that have notifications or a front category

### Commit Details

<details><summary>view details</summary>

 * feat(handler): add total system count ([`c91644f`](https://github.com/tulpje/tulpje/commit/c91644f81ff7d20d6761e5c93f8773e8e3ed8cda))
 * fix(handler): only try to update fronters for systems that have notifications or a front category ([`ed21560`](https://github.com/tulpje/tulpje/commit/ed215604859e568a481c0bed0abd000cdf99172b))
 * chore(handler): reduce sensitive information in log messages ([`ce8e344`](https://github.com/tulpje/tulpje/commit/ce8e3443dfb3deee1f2c24e137ca2f2af1c18ca0))
 * chore(handler): improve log message in `update_fronter_category ([`3258dc1`](https://github.com/tulpje/tulpje/commit/3258dc18deae52910247943498a53850b894632c))
</details>

## [0.21.4] - 2026-03-26

### Fixed

 - Make arguments to `just release` optional

### Commit Details

<details><summary>view details</summary>

 * fix(build): make arguments to `just release` optional ([`7653646`](https://github.com/tulpje/tulpje/commit/765364660e3c5be4f9fa074e2be70bfe542dd1ff))
</details>

## [0.21.4-rc.2] - 2026-03-25

### Added

 - Add `just release` shortcut for release script

### Fixed

 - Also store system info in `/pk setup`

### Commit Details

<details><summary>view details</summary>

 * feat(build): add `just release` shortcut for release script ([`f104e9b`](https://github.com/tulpje/tulpje/commit/f104e9b8e600e4e96f787e94d880ff676a5d866d))
 * fix(handler/pk): also store system info in `/pk setup` ([`ebf3c6c`](https://github.com/tulpje/tulpje/commit/ebf3c6c1a7294c8194be75367cd5e2c9db7e38ff))
</details>

## [0.21.4-rc.1] - 2026-03-25

### Fixed

 - Don't clean up systems that are still configured in guilds

### Commit Details

<details><summary>view details</summary>

 * fix(handler): don't clean up systems that are still configured in guilds ([`a8a1388`](https://github.com/tulpje/tulpje/commit/a8a13884a90c4229f519888dc2b957028fa68dfe))
</details>

## [0.21.3] - 2026-03-25

### Fixed

 - Run `guilds:cleanup` daily

### Commit Details

<details><summary>view details</summary>

 * fix(handler): run `guilds:cleanup` daily ([`9a18e57`](https://github.com/tulpje/tulpje/commit/9a18e57f62239adf86b5f5d74e12651272ba3cea))
</details>

## [0.21.3-rc.1] - 2026-03-25

### Changed

 - Guild cleanup
 - Register guild commands on `GuildCreate` instead of at start
 - Toggle for MESSAGE_CONTENT intent

### Fixed

 - Only start the database for sqlx-migrate/sqlx-prepare tasks
 - Track more resources we actually want from the cache

### Commit Details

<details><summary>view details</summary>

 * fix(build): only start the database for sqlx-migrate/sqlx-prepare tasks ([`22dcbb6`](https://github.com/tulpje/tulpje/commit/22dcbb6aaa734bcaeb0bd3e68b848d8847fde9cf))
 * feat(handler): guild cleanup ([`5ed26eb`](https://github.com/tulpje/tulpje/commit/5ed26ebe033d0dd3226e2a362b5fcc7ff33a4b75))
 * refactor(handler): register guild commands on `GuildCreate` instead of at start ([`3667557`](https://github.com/tulpje/tulpje/commit/366755726ea436c911735cba99ad6c994e363e2b))
 * feat: toggle for MESSAGE_CONTENT intent ([`b3c05f9`](https://github.com/tulpje/tulpje/commit/b3c05f9be64cab45fb1dbd008a9797b5c0035130))
 * fix(handler): track more resources we actually want from the cache ([`773ed59`](https://github.com/tulpje/tulpje/commit/773ed59be06e3d554733fc958d2aa700b003834e))
</details>

## [0.21.2] - 2026-03-25

### Commit Details

<details><summary>view details</summary>

</details>

## [0.21.2-rc.1] - 2026-03-25

### Changed

 - Fancier feedback message on `/pk role update`
 - Better `/pk role update` error messages
 - Better `/pk role update` success message

### Fixed

 - Also assign member roles to user
 - Fix `/pk roles update` not notifying user if system is private/not found, also fix using token

### Removed

 - Remove unnecessary GUILD_MEMBERS intent

### Commit Details

<details><summary>view details</summary>

 * chore(gateway): remove unnecessary GUILD_MEMBERS intent ([`5cf6c6c`](https://github.com/tulpje/tulpje/commit/5cf6c6c17ede4927272f8d9d2efeb15c5fd4f2e1))
 * feat(handler/pk): fancier feedback message on `/pk role update` ([`2961c62`](https://github.com/tulpje/tulpje/commit/2961c6222b01fcd30ce0f8a3fe839f4cde4f9ccc))
 * fix(handler/pk): also assign member roles to user ([`3f7f4bb`](https://github.com/tulpje/tulpje/commit/3f7f4bb4172e4b82a91db97afb4c1779dea0de89))
 * fix(handler/pk): fix `/pk roles update` not notifying user if system is private/not found, also fix using token ([`b4b493d`](https://github.com/tulpje/tulpje/commit/b4b493dd10b6e474ce3fc11eda7f10c2a715af00))
 * feat(handler/pk): better `/pk role update` error messages ([`5cf3016`](https://github.com/tulpje/tulpje/commit/5cf3016733c7714703cc024e48c8da45d7f87b79))
 * feat(handler/pk): better `/pk role update` success message ([`0993a2a`](https://github.com/tulpje/tulpje/commit/0993a2a24d3586d4ec65de676f337ea469097b0d))
</details>

## [0.21.1] - 2026-03-24

### Commit Details

<details><summary>view details</summary>

</details>

## [0.21.1-rc.1] - 2026-03-24

### Added

 - Add missing `EMBED_LINKS` permission for to the notification channel
 - Add `Permissions::CONNECT` to the permissions the bot needs on the fronter category

### Changed

 - Update actions/checkout to v6.0.2
 - Update nixbuild/nix-quick-install-action to v34
 - Update cachix/cachix-action to v17
 - Yaml file formatting

### Fixed

 - Correctly bump version on fixes
 - Fix a crash in `update_fronter_channels` and improve logging
 - Leftover reference to http-proxy

### Commit Details

<details><summary>view details</summary>

 * fix(build): correctly bump version on fixes ([`3eb1b48`](https://github.com/z0w13/tulpje/commit/3eb1b489ac40070778ea184b054c6415b4cb890a))
 * fix(handler/pk): fix a crash in `update_fronter_channels` and improve logging ([`bb83569`](https://github.com/z0w13/tulpje/commit/bb835691c11cec6ebd3e07e564f4b517a731755a))
 * fix(handler/pk): add missing `EMBED_LINKS` permission for to the notification channel ([`78c9fe9`](https://github.com/z0w13/tulpje/commit/78c9fe9c5af9d5fd19e191cebadf10038a7cd561))
 * fix(handler/pk): add `Permissions::CONNECT` to the permissions the bot needs on the fronter category ([`e9c5360`](https://github.com/z0w13/tulpje/commit/e9c53604613062b46083723da8f2d0e2089a5525))
 * chore(ci): update actions/checkout to v6.0.2 ([`dba3258`](https://github.com/z0w13/tulpje/commit/dba32587b228f1b78f38d6f57de6c53a0c8beece))
 * chore(ci): update nixbuild/nix-quick-install-action to v34 ([`2fd8bfc`](https://github.com/z0w13/tulpje/commit/2fd8bfce988a9a7cf742c58928d8bd03833a254b))
 * chore(ci): update cachix/cachix-action to v17 ([`79975a9`](https://github.com/z0w13/tulpje/commit/79975a9e0d21d5874e78574962c6f6bf1356c22f))
 * chore: yaml file formatting ([`7ae3919`](https://github.com/z0w13/tulpje/commit/7ae3919d0b35e34e3b6ffc16c808e0a642a987ec))
 * fix(ci): leftover reference to http-proxy ([`40225f5`](https://github.com/z0w13/tulpje/commit/40225f5859bbefa69fbebae6247d3ca23cb89116))
</details>

## [0.21.0] - 2026-03-24

### Commit Details

<details><summary>view details</summary>

</details>

## [0.21.0-rc.2] - 2026-03-24

### Fixed

 - Correctly update `updated_at` when front hasn't changed

### Commit Details

<details><summary>view details</summary>

 * fix(handler/pk): correctly update `updated_at` when front hasn't changed ([`cefbaa0`](https://github.com/z0w13/tulpje/commit/cefbaa05a6b2038511d3a93847c155caab38bcea))
</details>

## [0.21.0-rc.1] - 2026-03-24

### Breaking Changes

 - Switch to nirn-proxy instead of twilight-http-proxy

### Changed

 - Always create a new category when `/pk fronters setup` is used
 - Restrict requested intents
 - Document required permissions

### Fixed

 - Continue without loading secrets if directory is missing
 - Use `title` for fronter category name argument to clarify usage

### Commit Details

<details><summary>view details</summary>

 * fix(utils/secret-loader): continue without loading secrets if directory is missing ([`7a7c774`](https://github.com/z0w13/tulpje/commit/7a7c7747b05c24ff06bf2eb3e7518ee1547facd0))
 * feat!: switch to nirn-proxy instead of twilight-http-proxy ([`951749d`](https://github.com/z0w13/tulpje/commit/951749d24d987abc215581f47acf623eaa133264))
 * fix(handler/pk): use `title` for fronter category name argument to clarify usage ([`66744a1`](https://github.com/z0w13/tulpje/commit/66744a1e085356f28d7f9a62acbe0f1eba8e9069))
 * refactor(handler/pk): always create a new category when `/pk fronters setup` is used ([`a9c7256`](https://github.com/z0w13/tulpje/commit/a9c7256fb4f6a3b5afc9a546a909763671f44fe4))
 * chore(gateway): restrict requested intents ([`d054c86`](https://github.com/z0w13/tulpje/commit/d054c865eec1970e6f443063b9d7e1e208d893d0))
 * chore(docs): document required permissions ([`1e57044`](https://github.com/z0w13/tulpje/commit/1e57044921278ff2fcb013fe9b1775a1c6b99e20))
</details>

## [0.20.2] - 2026-03-23

### Commit Details

<details><summary>view details</summary>

</details>

## [0.20.2-rc.1] - 2026-03-23

### Added

 - Add a user guide
 - Add missing permission overwrites in `/pk notify setup`

### Changed

 - Use `handle_system_ref` in `/pk setup`
 - Move `handle_system_ref` to module wide utils
 - Handle 404 on `/pk setup`
 - Nicer messages in various places
 - Use new channel and permission code for `/pk fronters setup`
 - Rework and split out channel finding and permission checking logic
 - Make permission checking code generic
 - Better permission check code
 - Move reusable responses into dedicated module
 - Move some utility functions to crate root
 - Move `create_or_get_fronter_channel` next to setup command
 - Handle channel references in `/pk notify setup`
 - Error if user haven't set up a notify channel yet but they try to add a system

### Fixed

 - Make `find_channel_by_name` filter on channel type
 - Also release if we go from prerelease to full

### Removed

 - Remove old debug statement

### Commit Details

<details><summary>view details</summary>

 * chore(docs): add a user guide ([`30cfe3f`](https://github.com/z0w13/tulpje/commit/30cfe3f22ac088c7d864d7d8af9e3c1df8c1ee8f))
 * refactor(handler/pk): use `handle_system_ref` in `/pk setup` ([`81190d2`](https://github.com/z0w13/tulpje/commit/81190d2886f0e0249271effdf556d3762b067f09))
 * refactor(handler/pk): move `handle_system_ref` to module wide utils ([`73b484e`](https://github.com/z0w13/tulpje/commit/73b484ed0348bc17e7483d298b6684566b29bdaf))
 * chore(handler/pk): remove old debug statement ([`a4fb31d`](https://github.com/z0w13/tulpje/commit/a4fb31d7fded72710ac210d3c04bff9bcca8f421))
 * feat(handler/pk): handle 404 on `/pk setup` ([`0ea20bf`](https://github.com/z0w13/tulpje/commit/0ea20bfeb389476a5f0cdcd12dfec7ad3ced73f6))
 * feat(handler/pk): nicer messages in various places ([`31f73fa`](https://github.com/z0w13/tulpje/commit/31f73fa4b6ba1458f4373a20e7d5a814baceecd2))
 * fix(handler): make `find_channel_by_name` filter on channel type ([`bf3dc02`](https://github.com/z0w13/tulpje/commit/bf3dc0215dfddb89bea49741fc75afa8928031f6))
 * fix(handler/pk): add missing permission overwrites in `/pk notify setup` ([`2d7bbee`](https://github.com/z0w13/tulpje/commit/2d7bbee392ea1edba2827d11d9cfd0deebfba0ee))
 * refactor(handler/pk): use new channel and permission code for `/pk fronters setup` ([`af24809`](https://github.com/z0w13/tulpje/commit/af2480953f34ea4adb79ecb41f8db3ae366635c9))
 * refactor(handler/pk): rework and split out channel finding and permission checking logic ([`2f4950d`](https://github.com/z0w13/tulpje/commit/2f4950dffd23f4b726aa14f51b016b79823c2325))
 * refactor(handler): make permission checking code generic ([`bc28a85`](https://github.com/z0w13/tulpje/commit/bc28a859a6b8d1948ea5eb5b332fa9a4eae9a442))
 * refactor(handler): better permission check code ([`f229022`](https://github.com/z0w13/tulpje/commit/f229022f0b55f7f54389b86f920061ba3fd7ec88))
 * refactor(handler): move reusable responses into dedicated module ([`061aa52`](https://github.com/z0w13/tulpje/commit/061aa52763faa029673f690327e233870e4f426a))
 * refactor(handler): move some utility functions to crate root ([`4ddb652`](https://github.com/z0w13/tulpje/commit/4ddb652c6171245fb58cf01460a6cf219a65c19b))
 * refactor(handler/pk): move `create_or_get_fronter_channel` next to setup command ([`e90e7a7`](https://github.com/z0w13/tulpje/commit/e90e7a74439b39722d3aadba630ecf7570dadd16))
 * feat(handler/pk): handle channel references in `/pk notify setup` ([`c94ed9e`](https://github.com/z0w13/tulpje/commit/c94ed9ef4b88618ad405f3bd15e9ab6edda1d96f))
 * feat(handler/pk): error if user haven't set up a notify channel yet but they try to add a system ([`5e05b05`](https://github.com/z0w13/tulpje/commit/5e05b05200234dd93e38b2d43d261e53ddaaf5ab))
 * fix(build): also release if we go from prerelease to full ([`6172382`](https://github.com/z0w13/tulpje/commit/61723827f9ea937d6c4acfecee0c653d52aeacd9))
</details>

## [0.20.1] - 2026-03-23

### Changed

 - Clean shutdown

### Commit Details

<details><summary>view details</summary>

 * feat(handler): clean shutdown ([`6e5e297`](https://github.com/z0w13/tulpje/commit/6e5e2973ae6afff43313733d2c5928b3ab6753de))
</details>

## [0.20.1-rc.1] - 2026-03-23

### Added

 - Add missing `git-cliff` package to flake
 - Add check in `/pk roles update` for discord role limit
 - Additional comments
 - Additional comments
 - Add env var to enable log source file and line no logging
 - Add user facing errors for incorrect system references

### Changed

 - Let user know if they were/weren't already following a system
 - Implement follow limit
 - Update flake inputs to latest
 - Use a global pk client
 - Implement `Display` for `SystemRef`
 - Check front is public during front category setup
 - Track number of guilds we're in
 - Use fancy response messages in `/pk notify setup`
 - Split up `/pk notify setup` code
 - Use fancy response messages in fronter module
 - Factor out repetitive system ref parsing code
 - Implement `success_response` and `error_response` utility functions
 - Move role commands into subgroup and subcategory
 - Clippy fix
 - Split up fronter submodule more clearly
 - Move fronter command definitions into fronter module

### Fixed

 - Fix feature commit detection
 - Fix clippy warnings
 - Use `PkClient::with_token` to reuse existing client
 - Show system name in `/pk fronters setup` if available
 - Correctly handle private front checking
 - Correctly handle systems without registered switches
 - Correctly use `Span`s with async functions
 - Typo
 - Make `/pk fronters setup` show error if pk module hasn't been setup yet

### Commit Details

<details><summary>view details</summary>

 * fix(build): add missing `git-cliff` package to flake ([`049bb6b`](https://github.com/z0w13/tulpje/commit/049bb6bff327453d453d861aed9ffbe84267ab93))
 * fix(build): fix feature commit detection ([`e8b0595`](https://github.com/z0w13/tulpje/commit/e8b0595d291521b65e20a5e3f6ee52957a7324d8))
 * feat(handler/pk): let user know if they were/weren't already following a system ([`ff97974`](https://github.com/z0w13/tulpje/commit/ff97974e4c9e028b8f2c46635a8e1281156a23ec))
 * feat(handler/pk): implement follow limit ([`0f611cb`](https://github.com/z0w13/tulpje/commit/0f611cb0bc97f5d6600000c6c0b2264a4e4cbeaf))
 * feat(handler/pk): add check in `/pk roles update` for discord role limit ([`02decf4`](https://github.com/z0w13/tulpje/commit/02decf46401045035a640545e9200a1ec3970019))
 * chore(handler/pk): fix clippy warnings ([`d0d56bc`](https://github.com/z0w13/tulpje/commit/d0d56bca4e4ab066bf4a710a840beecf89c798e3))
 * fix(handler/pk): use `PkClient::with_token` to reuse existing client ([`7bcce84`](https://github.com/z0w13/tulpje/commit/7bcce84cff5fd673a21f3e2405507940e6490ddd))
 * chore(deps): update flake inputs to latest ([`9d480d8`](https://github.com/z0w13/tulpje/commit/9d480d807d22d0b37f2ccdc6ec1c61d3ce61f1c3))
 * refactor(handler): use a global pk client ([`818b7ef`](https://github.com/z0w13/tulpje/commit/818b7ef9d5ee1bcc42b52c8d724e4698740f6a51))
 * fix(handler/pk): show system name in `/pk fronters setup` if available ([`e47cdff`](https://github.com/z0w13/tulpje/commit/e47cdff782eadc12c59feba3cabe2df2dbccd340))
 * fix(handler/pk): correctly handle private front checking ([`ab0f761`](https://github.com/z0w13/tulpje/commit/ab0f761a51ae056cc7f74f1167b28e3bef01d403))
 * feat(handler/pk): implement `Display` for `SystemRef` ([`a76fc62`](https://github.com/z0w13/tulpje/commit/a76fc625956f71543db4682db7050cf2d7917e40))
 * chore(handler/pk): additional comments ([`40607f1`](https://github.com/z0w13/tulpje/commit/40607f199015a75db052295ac034c8d682dcea10))
 * fix(handler/pk): correctly handle systems without registered switches ([`9de5245`](https://github.com/z0w13/tulpje/commit/9de52454856dc82837806bd652c76aa1a83cb59b))
 * chore(handler/pk): additional comments ([`f14690d`](https://github.com/z0w13/tulpje/commit/f14690d3eb11d618a818b25aa3328f59b21d6796))
 * feat(handler/pk): check front is public during front category setup ([`df5c36c`](https://github.com/z0w13/tulpje/commit/df5c36cec45966c0096d611c80af72d0aed5ecf7))
 * fix: correctly use `Span`s with async functions ([`b99f172`](https://github.com/z0w13/tulpje/commit/b99f172eee7c140ad55a2a5378bac1f63c23fc1f))
 * feat(shared): add env var to enable log source file and line no logging ([`2b09379`](https://github.com/z0w13/tulpje/commit/2b09379525dd1ebe1505f98e195b0a4bc7258a83))
 * fix(gateway): typo ([`6b42bfb`](https://github.com/z0w13/tulpje/commit/6b42bfb3c14921a04d575b98a627835dc5c55f68))
 * feat(gateway): track number of guilds we're in ([`f238561`](https://github.com/z0w13/tulpje/commit/f238561d4961d4d11cf77a30ec1eeec569a6a97e))
 * feat(handler/pk): use fancy response messages in `/pk notify setup` ([`320f4d9`](https://github.com/z0w13/tulpje/commit/320f4d94a91d2353392fc751dcc3e0b887392329))
 * refactor(handler/pk): split up `/pk notify setup` code ([`8d973ea`](https://github.com/z0w13/tulpje/commit/8d973eaf2f35e57643dc02fc478517941bbf3f4e))
 * feat(handler/pk): use fancy response messages in fronter module ([`a8677ba`](https://github.com/z0w13/tulpje/commit/a8677bacd7ca0d5ab97d5a84f06c63fc46700385))
 * fix(handler/pk): make `/pk fronters setup` show error if pk module hasn't been setup yet ([`f3368b8`](https://github.com/z0w13/tulpje/commit/f3368b80c893d3575c38c449f437e060e992da72))
 * refactor(handler/pk): factor out repetitive system ref parsing code ([`57e1d1d`](https://github.com/z0w13/tulpje/commit/57e1d1d7360ce73b860f877fc78d5e9ec29bf24d))
 * refactor(handler/pk): implement `success_response` and `error_response` utility functions ([`d3e0900`](https://github.com/z0w13/tulpje/commit/d3e090020970d481d704e0695bcc580202ab4412))
 * refactor(handler/pk): move role commands into subgroup and subcategory ([`b6cbcd6`](https://github.com/z0w13/tulpje/commit/b6cbcd6a16d90a317132d137d1ce363e550334d8))
 * chore(shared): clippy fix ([`5afca4e`](https://github.com/z0w13/tulpje/commit/5afca4e637f133b39c550b4eedbd7b298eb808e4))
 * refactor(handler/pk): split up fronter submodule more clearly ([`a43260d`](https://github.com/z0w13/tulpje/commit/a43260d24f52abe8bec4fe5b8e66d6815be88755))
 * refactor(handler/pk): move fronter command definitions into fronter module ([`8f05f14`](https://github.com/z0w13/tulpje/commit/8f05f14ef09c39fcb3f30389a92258cbb58da74d))
 * fix(handler/pk): add user facing errors for incorrect system references ([`24ca607`](https://github.com/z0w13/tulpje/commit/24ca60755f4434520d5beeabd9f3c61e89755cbc))
</details>

## [0.20.0] - 2026-03-22

### Changed

 - Mark 0.20.0-rc.4 as reverted in changelog

### Fixed

 - Log message typo

### Commit Details

<details><summary>view details</summary>

 * chore: mark 0.20.0-rc.4 as reverted in changelog ([`89308d6`](https://github.com/z0w13/tulpje/commit/89308d6228a1b8f0364e88ebf61d435c4bcc337b))
 * fix(handler/pk): log message typo ([`418ab8e`](https://github.com/z0w13/tulpje/commit/418ab8e3b8e30000ff995a3d0227f044ab49f2e9))
</details>

## [0.20.0-rc.4] - 2026-03-22

*reverted*

## [0.20.0-rc.3] - 2026-03-22

### Changed

 - More logging in `process_system`

### Fixed

 - Fix RUST_LOG not being honoured, and spans not showing

### Commit Details

<details><summary>view details</summary>

 * fix(shared/logging): fix RUST_LOG not being honoured, and spans not showing ([`dea1dc3`](https://github.com/z0w13/tulpje/commit/dea1dc3ed96d4ef65f5666dba510b2b8db1ea896))
 * chore(module/pk): more logging in `process_system` ([`94b7f61`](https://github.com/z0w13/tulpje/commit/94b7f6110dafe7f0cc150e3a244ad1624172a6c1))
</details>

## [0.20.0-rc.2] - 2026-03-22

### Fixed

 - Mark prereleases correctly
 - Flatten event fields so VictoriaLogs can parse it

### Commit Details

<details><summary>view details</summary>

 * fix(build): mark prereleases correctly ([`d6947e8`](https://github.com/z0w13/tulpje/commit/d6947e84d30643c750c87437d73c86af4e5ff3d9))
 * fix(shared/logging): flatten event fields so VictoriaLogs can parse it ([`0088a0f`](https://github.com/z0w13/tulpje/commit/0088a0f4a2cde8db34af7c7e3ed9378576e830fb))
</details>

## [0.20.0-rc.1] - 2026-03-22

### Added

 - Add extra metrics
 - Add tracing::span to event handling
 - Add `sqlx-migrate` and `sqlx-prepare` tasks

### Changed

 - Enable globally instead of per guild
 - Configurable log format using `RUST_LOG_FORMAT` env var
 - Prettier responses when adding/removing notify systems
 - Better logging for update-member-roles
 - Update vulnerable deps
 - Staggered fronter updates
 - Use cache when updating fronters
 - Use `sqlx::query_as!` instead of manual conversion
 - Front change notifications
 - Update pkrs from 0.4.0 to 0.5.0
 - Don't save token, only optionally use it to update member roles
 - Update vulnerable deps
 - Switch to using `mod.rs` files
 - Update to rust 1.94.0

### Fixed

 - Still set `last_updated` when front is forbidden
 - Also handle systems we can't fetch from PluralKit in add/remove
 - Fix clippy warnings

### Removed

 - Delete fronters before deleting system

### Commit Details

<details><summary>view details</summary>

 * chore(module/pk): enable globally instead of per guild ([`0feaf60`](https://github.com/z0w13/tulpje/commit/0feaf601f8a284792e55c0fb23540e8b691ef221))
 * feat(handler/pk): add extra metrics ([`88e757d`](https://github.com/z0w13/tulpje/commit/88e757d503ba2092c4aba43d953a29ca330e62b8))
 * feat: configurable log format using `RUST_LOG_FORMAT` env var ([`1fe4841`](https://github.com/z0w13/tulpje/commit/1fe4841a758a9e666d70ab11a04245836a4c36a7))
 * feat(framework): add tracing::span to event handling ([`af95661`](https://github.com/z0w13/tulpje/commit/af9566109fb612ca637b1b8a2cda2d3520f2e974))
 * feat(pk): prettier responses when adding/removing notify systems ([`c91c789`](https://github.com/z0w13/tulpje/commit/c91c789f48a9d3060f9ec429fdf5aeb70f8da474))
 * feat(pk): better logging for update-member-roles ([`3b663c6`](https://github.com/z0w13/tulpje/commit/3b663c673f173d65790c76d621d506201436c71c))
 * fix(pk): still set `last_updated` when front is forbidden ([`22fe176`](https://github.com/z0w13/tulpje/commit/22fe1764b0d60feae2d0e035437541bdfde8a0f0))
 * chore(deps): update vulnerable deps ([`59030fd`](https://github.com/z0w13/tulpje/commit/59030fda04a68f7d6e9bb5cb6baf4f213a6db5b7))
 * fix(pk): also handle systems we can't fetch from PluralKit in add/remove ([`ddd036d`](https://github.com/z0w13/tulpje/commit/ddd036dcdae55c05303a5dda727218bf731fc77f))
 * fix(pk): delete fronters before deleting system ([`3ff27b3`](https://github.com/z0w13/tulpje/commit/3ff27b34bb99053b9c26b13113b034afa8887a5a))
 * feat(pk): staggered fronter updates ([`626dc08`](https://github.com/z0w13/tulpje/commit/626dc08d9619c2b48c2b720f25e092600a1abd0b))
 * refactor(handler/pk): use cache when updating fronters ([`c28d14a`](https://github.com/z0w13/tulpje/commit/c28d14aafc6384f4e1077c17a358021101f855d3))
 * refactor(handler/pk): use `sqlx::query_as!` instead of manual conversion ([`41366bf`](https://github.com/z0w13/tulpje/commit/41366bfc0a75bec6711e73d2ab18510358b8b0d2))
 * feat(justfile): add `sqlx-migrate` and `sqlx-prepare` tasks ([`4e09ecf`](https://github.com/z0w13/tulpje/commit/4e09ecfded5aabba42bdcc42d9a9ab42002a6863))
 * feat(pluralkit): front change notifications ([`09e7223`](https://github.com/z0w13/tulpje/commit/09e72236515752c1645fcb2aa7877742a8cb8f7d))
 * chore(deps): update pkrs from 0.4.0 to 0.5.0 ([`e774904`](https://github.com/z0w13/tulpje/commit/e77490452c0939cacdb47f74c006306259125187))
 * refactor(pk): don't save token, only optionally use it to update member roles ([`3c84bd7`](https://github.com/z0w13/tulpje/commit/3c84bd75f85a8da932f1f20d65a67a66b87f956b))
 * chore(deps): update vulnerable deps ([`9f4c1f2`](https://github.com/z0w13/tulpje/commit/9f4c1f226b2a7c63294c56a3c912935cc6a6589c))
 * refactor: switch to using `mod.rs` files ([`26a61a2`](https://github.com/z0w13/tulpje/commit/26a61a23019ee851f397a7721ec2eaafb638e9e5))
 * chore: fix clippy warnings ([`cce8aa8`](https://github.com/z0w13/tulpje/commit/cce8aa82664ee7b1ddd1d1a68161379dfee6e462))
 * chore: update to rust 1.94.0 ([`9a43d87`](https://github.com/z0w13/tulpje/commit/9a43d87733aab5cea01366b8539ebd730f04b6c9))
</details>

## [0.19.1] - 2026-03-20

### Added

 - Add brief setup instructions to the README

### Fixed

 - Fix version bumping logic

### Commit Details

<details><summary>view details</summary>

 * fix(build): fix version bumping logic ([`8288c71`](https://github.com/z0w13/tulpje/commit/8288c71c22b406cebd8e301e24b06fe30c8a41c9))
 * chore: add brief setup instructions to the README ([`9710e5f`](https://github.com/z0w13/tulpje/commit/9710e5f146cab6a79c28bf08793ad1392b4247b7))
</details>

## [0.19.0] - 2025-11-15

### Added

 - Support prereleases in `release.py`

### Fixed

 - Don't exclude alpha/beta/rc tags in changelog
 - Always include prereleases
 - Re-add `metrics` feature flag, was renamed not removed

### Commit Details

<details><summary>view details</summary>

 * fix(build/release): don't exclude alpha/beta/rc tags in changelog ([`40bcf5a`](https://github.com/z0w13/tulpje/commit/40bcf5a0cb590434deb22b7f3716bbd1495a386b))
 * fix(build/release): always include prereleases ([`c61cad4`](https://github.com/z0w13/tulpje/commit/c61cad40b11bca413f2d27df8c09608011795d6f))
 * feat(build): support prereleases in `release.py` ([`dd852d3`](https://github.com/z0w13/tulpje/commit/dd852d34a1e1ea66bb8e811cb1e12a4d61001468))
 * fix(http-proxy): re-add `metrics` feature flag, was renamed not removed ([`6d827d5`](https://github.com/z0w13/tulpje/commit/6d827d5d6755f2016f7134b9f9cce1c2224eed10))
</details>

## [0.19.0-rc.1] - 2025-11-15

### Breaking Changes

 - Update twilight dependencies to 0.17.0
 - Remove unused parse_task_slot

### Added

 - Add missing fields to component structs
 - Add `cargo edit` and `cargo machete`
 - Add cargo-outdated to devenv packages
 - Additional comments and cleanup
 - Additional logging in `release.py

### Changed

 - Use gateway-queue fork using twilight 0.17
 - Update to rust 1.91.0
 - Bump `tokio-util` from 0.7.16 to 0.7.17
 - Make `tokio-util` a workspace dependency
 - Bump `reqwest` from 0.12.23 to 0.12.24
 - Make `reqwest` a workspace dependency
 - Bump `amqprs` from 2.1.2 to 2.1.3
 - Bump `regex` from 1.11.3 to 1.12.2
 - Bump `metrics-process` from 2.4.1 to 2.4.2
 - Bump `tokio` from 1.47.1 to 1.48.0
 - Make `tokio` a workspace dependency
 - Configure tls correctly for 0.17
 - Update to 0.17.0
 - Update to latest version
 - Update flake inputs to latest
 - Bump `async-trait` from 0.1.86 to 0.1.89
 - Bump `sqlx` from 0.8.5 to 0.8.6
 - Bump `regex` from 1.11.3 to 1.11.5
 - Bump `serde_json` from 1.0.140 to 1.0.145
 - Bump `serde` from 1.0.219 to 1.0.228
 - Bump `reqwest` from 0.12.15 to 0.12.23
 - Bump tokio from 1.44.2 to 1.47.1
 - Bump tokio-util from 0.7.14 to 0.7.16
 - Bump chrono from 0.4.40 to 0.4.42
 - Bump `uuid` from 0.16.0 to 0.18.1
 - Bump `metrics-process from 2.4.0 to 2.4.1
 - Bump `metrics-exporter-prometheus` from 0.16.2 to 0.17.1
 - Move `metrics-exporter-prometheus` crate to workspace deps
 - Bump redis from 0.29.1 to 0.32.6
 - Move redis crate to workspace deps
 - Update to rust 1.90.0, fix lint warnings, `cargo fmt`
 - `clippy::collapsible_if`
 - `cargo fmt`
 - Rust edition 2024
 - Pass env vars to child process instead of directly setting
 - Specify edition on workspace level
 - Bump cargo feature resolver from 2 to 3
 - Move twilight-* crates to workspace deps
 - Update amqprs from 2.1.0 to 2.1.2
 - Update flake inputs to latest
 - Update to rust 1.89.0
 - Rust-like output for `release.py`

### Fixed

 - Use primary_color for member roles
 - ReadyInfo no longer needs to be dereferenced
 - Also use non-annotated git tags to determine version
 - Log invalid semver tags and skip them instead of crashing in release script
 - Bump tracing subscriber from 0.3.19 to 0.3.20
 - Hanging connections

### Removed

 - Remove unused `crate` argument from `release.py`
 - Remove clippy warning that no longer exists
 - Remove feature flag removed by upstream
 - Remove tls feature flags that got removed in twilight 0.17
 - Remove unused deps

### Commit Details

<details><summary>view details</summary>

 * fix(build): remove unused `crate` argument from `release.py` ([`ba1177b`](https://github.com/z0w13/tulpje/commit/ba1177b852e771dfde7ef56e750c5f9f9ccf82dd))
 * chore: use gateway-queue fork using twilight 0.17 ([`dd9ebf3`](https://github.com/z0w13/tulpje/commit/dd9ebf3402fa5bbfafb2020e153809a56fb13bf4))
 * fix(handler): remove clippy warning that no longer exists ([`3ce7b94`](https://github.com/z0w13/tulpje/commit/3ce7b9465ba7f10e1f51becaf55479096719e57d))
 * chore: update to rust 1.91.0 ([`e5a8e92`](https://github.com/z0w13/tulpje/commit/e5a8e929741bdb06f897d4c154056b218a47877f))
 * chore(deps): bump `tokio-util` from 0.7.16 to 0.7.17 ([`5f1931b`](https://github.com/z0w13/tulpje/commit/5f1931be04a043a8b4af0784dd33b1dd7bfda2d1))
 * chore(deps): make `tokio-util` a workspace dependency ([`ebd429b`](https://github.com/z0w13/tulpje/commit/ebd429be53f9df0477154aaad445bbcdab9f28c9))
 * chore(deps): bump `reqwest` from 0.12.23 to 0.12.24 ([`4b64090`](https://github.com/z0w13/tulpje/commit/4b640905b47fc5cb65dde7982e6cb0f77208dd5c))
 * chore(deps): make `reqwest` a workspace dependency ([`f71129b`](https://github.com/z0w13/tulpje/commit/f71129b4a40b9ca802c7e064f25fbe13355a2fb0))
 * chore(deps): bump `amqprs` from 2.1.2 to 2.1.3 ([`1dd263a`](https://github.com/z0w13/tulpje/commit/1dd263add320bd115813717509400fbc37669c72))
 * chore(deps): bump `regex` from 1.11.3 to 1.12.2 ([`7e5e016`](https://github.com/z0w13/tulpje/commit/7e5e016bf1ae5b7b2fa298badb04964d9277cc9e))
 * chore(deps): bump `metrics-process` from 2.4.1 to 2.4.2 ([`20432f3`](https://github.com/z0w13/tulpje/commit/20432f3b9e7c9d23e4de28146c8e4569003b653e))
 * chore(deps): bump `tokio` from 1.47.1 to 1.48.0 ([`2d56289`](https://github.com/z0w13/tulpje/commit/2d562894b83375484d4593f1d986c52ad7a2eb5d))
 * chore(deps): make `tokio` a workspace dependency ([`1a2f46a`](https://github.com/z0w13/tulpje/commit/1a2f46aecb624069c466738891ba905e446a1637))
 * fix(handler/emoji): add missing fields to component structs ([`bdff86f`](https://github.com/z0w13/tulpje/commit/bdff86f88ecae71f5b2acf9c226705685998ccb2))
 * chore: configure tls correctly for 0.17 ([`76d856f`](https://github.com/z0w13/tulpje/commit/76d856f79ba957985b8f7151c7109d643a00cf65))
 * fix(http-proxy): remove feature flag removed by upstream ([`ee6fb5f`](https://github.com/z0w13/tulpje/commit/ee6fb5f159731828e6753c2524cceec20eb3c080))
 * chore(http-proxy): update to 0.17.0 ([`41db799`](https://github.com/z0w13/tulpje/commit/41db7990639ab5b6b17ce17459efeb3a4b260265))
 * chore(gateway-queue): update to latest version ([`85d077d`](https://github.com/z0w13/tulpje/commit/85d077d9f16b50d2529cae4e4746a1b3ffca85e6))
 * fix(handler/pk): use primary_color for member roles ([`7b4dd11`](https://github.com/z0w13/tulpje/commit/7b4dd11cf79d9fae2c531c407f244070090eefa9))
 * fix: remove tls feature flags that got removed in twilight 0.17 ([`1c1bf24`](https://github.com/z0w13/tulpje/commit/1c1bf24dc2048e2089e0a4ef87493a7a55872460))
 * fix(gateway): ReadyInfo no longer needs to be dereferenced ([`387dba7`](https://github.com/z0w13/tulpje/commit/387dba70fed68eb0ac715f59f0bffdad192ab1ed))
 * chore!: update twilight dependencies to 0.17.0 ([`2f57061`](https://github.com/z0w13/tulpje/commit/2f570610a5c012d91a99ea3ed30c5ffb1d168fd0))
 * chore(deps): update flake inputs to latest ([`25d1cf2`](https://github.com/z0w13/tulpje/commit/25d1cf205fe9e996e6095a8a760a9b8775454ad4))
 * chore(deps): bump `async-trait` from 0.1.86 to 0.1.89 ([`0a60b47`](https://github.com/z0w13/tulpje/commit/0a60b47fca218c9e150987b146afecb0c0064f9e))
 * chore(deps): bump `sqlx` from 0.8.5 to 0.8.6 ([`1cd6d49`](https://github.com/z0w13/tulpje/commit/1cd6d497a81047232b7af1bf879ed407386db883))
 * chore(deps): bump `regex` from 1.11.3 to 1.11.5 ([`e1b01bc`](https://github.com/z0w13/tulpje/commit/e1b01bc03267a277782b07ea286157ffeef25c77))
 * chore(deps): bump `serde_json` from 1.0.140 to 1.0.145 ([`0dd1cd8`](https://github.com/z0w13/tulpje/commit/0dd1cd8340faf6afc223ad6e1e5b1c5e3162285a))
 * chore(deps): bump `serde` from 1.0.219 to 1.0.228 ([`5b1f399`](https://github.com/z0w13/tulpje/commit/5b1f39919a293e04bb34b12a6e843b044f7ebe2d))
 * chore(deps): bump `reqwest` from 0.12.15 to 0.12.23 ([`4ba8a1c`](https://github.com/z0w13/tulpje/commit/4ba8a1ce1fa9d1ff64a33ef850571e3be330816c))
 * build(deps): bump tokio from 1.44.2 to 1.47.1 ([`44d256f`](https://github.com/z0w13/tulpje/commit/44d256fe7de24db17b823dd5f67176e278e7f154))
 * build(deps): bump tokio-util from 0.7.14 to 0.7.16 ([`b084877`](https://github.com/z0w13/tulpje/commit/b084877909230c68857b633a8581ce425a45f67a))
 * build(deps): bump chrono from 0.4.40 to 0.4.42 ([`df640ec`](https://github.com/z0w13/tulpje/commit/df640ec19e6b7ffcb735170d5e605d0143ac53a0))
 * chore(deps): bump `uuid` from 0.16.0 to 0.18.1 ([`bb59882`](https://github.com/z0w13/tulpje/commit/bb5988295d34695f9356211a9f86439e44388877))
 * chore(deps): bump `metrics-process from 2.4.0 to 2.4.1 ([`c48771d`](https://github.com/z0w13/tulpje/commit/c48771d77618900132af16d98332bee6b534da08))
 * chore(deps): bump `metrics-exporter-prometheus` from 0.16.2 to 0.17.1 ([`ccdcfb7`](https://github.com/z0w13/tulpje/commit/ccdcfb7021b2ac8ff5cb63411786c52e4a96f592))
 * chore(deps): move `metrics-exporter-prometheus` crate to workspace deps ([`0fa597b`](https://github.com/z0w13/tulpje/commit/0fa597b69a9bfdf82f59af027803718a194585e1))
 * chore(deps): bump redis from 0.29.1 to 0.32.6 ([`c5cf3fb`](https://github.com/z0w13/tulpje/commit/c5cf3fbdd854a4f81fb18e3a287b351527fac0c7))
 * chore(deps): move redis crate to workspace deps ([`b597314`](https://github.com/z0w13/tulpje/commit/b59731451f1a88e1839675542d0aec6b998a346e))
 * feat(build): add `cargo edit` and `cargo machete` ([`a573309`](https://github.com/z0w13/tulpje/commit/a57330947da6ec62d8b89cb1996bceb58d73d417))
 * chore(deps): remove unused deps ([`98fe91c`](https://github.com/z0w13/tulpje/commit/98fe91c111fd54a34e23afdd117695e52c2f81a3))
 * chore(build): update to rust 1.90.0, fix lint warnings, `cargo fmt` ([`4a93c3b`](https://github.com/z0w13/tulpje/commit/4a93c3be063b99cbf6f4cd773e4b6fcf60f0b9bc))
 * fix(build): also use non-annotated git tags to determine version ([`45398fd`](https://github.com/z0w13/tulpje/commit/45398fd68c3d3c1b8b08a59839393ec79f45dd16))
 * chore(lint): `clippy::collapsible_if` ([`5c9d89e`](https://github.com/z0w13/tulpje/commit/5c9d89e3def56d8672cfa5399ced073f28884e99))
 * chore: `cargo fmt` ([`e76d893`](https://github.com/z0w13/tulpje/commit/e76d893b5102eca310144ab258e79553cb5b2f41))
 * chore(build): rust edition 2024 ([`31fc0c6`](https://github.com/z0w13/tulpje/commit/31fc0c6d2ad5102d7d48f0115bcce37e815dd601))
 * refactor!: remove unused parse_task_slot ([`19d8abd`](https://github.com/z0w13/tulpje/commit/19d8abd3afa41c48a7498e5b5602f64db8478c3b))
 * refactor(secret-loader): pass env vars to child process instead of directly setting ([`b37c463`](https://github.com/z0w13/tulpje/commit/b37c463cf8e7cadb8613e57d22467bc7f1a47fdd))
 * refactor(build): specify edition on workspace level ([`f5b7a79`](https://github.com/z0w13/tulpje/commit/f5b7a79c4d5c5051e9dc3cc8b0def19fe22c63a6))
 * chore(build): bump cargo feature resolver from 2 to 3 ([`832bc9a`](https://github.com/z0w13/tulpje/commit/832bc9a1fb45aadb3288cf3a74265d63b3d317f9))
 * feat(build): add cargo-outdated to devenv packages ([`edb7020`](https://github.com/z0w13/tulpje/commit/edb70202d6da9088aa1d1b7dc34202840d2ec4a7))
 * chore(deps): move twilight-* crates to workspace deps ([`f23e379`](https://github.com/z0w13/tulpje/commit/f23e37987e31d52258c6f6a5a4b856b33e2756ef))
 * chore(reconnecting-amqp/deps): update amqprs from 2.1.0 to 2.1.2 ([`7c76ee2`](https://github.com/z0w13/tulpje/commit/7c76ee2b256b1919b560b19bd4eb073ccb4f81f1))
 * chore(deps): update flake inputs to latest ([`fc5afe4`](https://github.com/z0w13/tulpje/commit/fc5afe49df00fd5c8da945d3e6c131b31acdcee0))
 * fix(build): log invalid semver tags and skip them instead of crashing in release script ([`dcc2d9a`](https://github.com/z0w13/tulpje/commit/dcc2d9a281fb89809558f07e125ee4c9f7d78a48))
 * chore(deps): update to rust 1.89.0 ([`0d6a462`](https://github.com/z0w13/tulpje/commit/0d6a462e00505fcafb7cf479201932e0e728a697))
 * chore(nix): additional comments and cleanup ([`8952461`](https://github.com/z0w13/tulpje/commit/8952461cd04036552efee97620aba9e089ca8eeb))
 * fix(deps): bump tracing subscriber from 0.3.19 to 0.3.20 ([`f569250`](https://github.com/z0w13/tulpje/commit/f569250d1425a5920560b10ba213c0791df44728))
 * fix(gateway): hanging connections ([`dc44437`](https://github.com/z0w13/tulpje/commit/dc44437096a4792f9782ccc219ce2f29d7c221c3))
 * feat(build): additional logging in `release.py ([`c8ba08f`](https://github.com/z0w13/tulpje/commit/c8ba08ff395932f217543ad7c7351cc344a4bfac))
 * feat(build): rust-like output for `release.py` ([`926fdd9`](https://github.com/z0w13/tulpje/commit/926fdd9b811be62ca9af2277b02611750e5e3034))
</details>

## [0.18.0] - 2025-04-22

### Breaking Changes

 - Split `reconnecting-amqp` into separate crate

### Added

 - Add logging to `contrib/release.py`
 - Add start up messages containing version to gateway/handler
 - Add type safety to state transitions
 - Add `AmqpHandle::wait_start` that waits for amqp to connect

### Changed

 - Bump metrics from 0.24.1 to 0.24.2 in [#29](https://github.com/z0w13/tulpje/pull/29)
 - Format `release.py` using `ruff`
 - Use python-semver in `release.py`
 - Use uv for `release.py`
 - Bump sqlx from 0.8.4 to 0.8.5
 - Use state machine, only reopen channel if channel closed
 - Rewrite using an event handler loop
 - Implement reconnection logic for amqp
 - Bump sqlx from 0.8.3 to 0.8.4 in [#28](https://github.com/z0w13/tulpje/pull/28)
 - Make metrics listen address configurable
 - Move shared amqp logic into tulpje-shared

### Fixed

 - Fix changelog generation
 - Fix skopeo command

### Commit Details

<details><summary>view details</summary>

 * build: fix changelog generation ([`06a35d0`](https://github.com/z0w13/tulpje/commit/06a35d04ad4e30120e13e779dd43af9b2490c7a0))
 * build: add logging to `contrib/release.py` ([`cace833`](https://github.com/z0w13/tulpje/commit/cace83338ce42762ff424ab11f6e602807f2b4b0))
 * feat: add start up messages containing version to gateway/handler ([`e5e8545`](https://github.com/z0w13/tulpje/commit/e5e8545ac890b458dbb7d8a443cc9a0da569bbfa))
 * build(deps): bump metrics from 0.24.1 to 0.24.2 ([`990a8f8`](https://github.com/z0w13/tulpje/commit/990a8f8139cfe368257a09284a627569ed223ccb))
 * refactor!: split `reconnecting-amqp` into separate crate ([`9ea6bee`](https://github.com/z0w13/tulpje/commit/9ea6beeed957ee35f3184cdde44f96e604a323cc))
 * feat(shared/amqp): add type safety to state transitions ([`977a5d0`](https://github.com/z0w13/tulpje/commit/977a5d06ccde385cc38a38278011b7aa550c6648))
 * chore(style): format `release.py` using `ruff` ([`eec41fa`](https://github.com/z0w13/tulpje/commit/eec41fa0d610f02f30144ec3be075ba54de469e9))
 * refactor(build): use python-semver in `release.py` ([`29a1b99`](https://github.com/z0w13/tulpje/commit/29a1b9984ec0cdbfa47a75c5e91460bba70a4aa6))
 * feat(build): use uv for `release.py` ([`8889b09`](https://github.com/z0w13/tulpje/commit/8889b09938bb646a680cda021fba4b2ccda85d62))
 * build(deps): bump sqlx from 0.8.4 to 0.8.5 ([`e4e02e1`](https://github.com/z0w13/tulpje/commit/e4e02e152ffe7c81a1e3aa78427bdfcc7bb75ca7))
 * feat(shared/amqp): add `AmqpHandle::wait_start` that waits for amqp to connect ([`cb75c4f`](https://github.com/z0w13/tulpje/commit/cb75c4f039ae12224a69e8c39879ff6212493a1a))
 * refactor(shared/amqp): use state machine, only reopen channel if channel closed ([`e725af6`](https://github.com/z0w13/tulpje/commit/e725af616257e236db6422e16a334e777e790b36))
 * refactor(shared/amqp): rewrite using an event handler loop ([`450dee0`](https://github.com/z0w13/tulpje/commit/450dee07ec798e7ea246e351bd91f18b68a38950))
 * feat(shared): implement reconnection logic for amqp ([`366ddd6`](https://github.com/z0w13/tulpje/commit/366ddd6bafae04b46394c2c3f4c02d61a50cdbe9))
 * build(deps): bump sqlx from 0.8.3 to 0.8.4 ([`52ff5b9`](https://github.com/z0w13/tulpje/commit/52ff5b9bca58d903f4b53792172a02a69dc275f2))
 * feat(shared): make metrics listen address configurable ([`aa9101a`](https://github.com/z0w13/tulpje/commit/aa9101a97a58d80a6564f614eabbf76df2fdedda))
 * refactor: move shared amqp logic into tulpje-shared ([`afc8d3d`](https://github.com/z0w13/tulpje/commit/afc8d3d49213872878a9eab1ef2ccdaf00486876))
 * build(github): fix skopeo command ([`56dd090`](https://github.com/z0w13/tulpje/commit/56dd09098e4675431224286f2097282c50b824d2))
</details>

## [0.17.1] - 2025-04-07

### Added

 - Add cachix-action
 - Add nix-community substituter
 - Add github ci and docker image workflows

### Changed

 - Use personal cachix cache
 - Bump tokio from 1.43.0 to 1.44.2
 - Get local development with docker working again
 - Update twilight-gateway-queue to 5f5e0c1

### Fixed

 - Make dependabot target dev branch
 - Don't show double : in image name
 - Fix ability to run `docker compose up`

### Commit Details

<details><summary>view details</summary>

 * build(github): add cachix-action ([`152016f`](https://github.com/z0w13/tulpje/commit/152016f14904166b22c0ff2014a2a019d8ea0df6))
 * build: use personal cachix cache ([`beef3d0`](https://github.com/z0w13/tulpje/commit/beef3d069a28174f12c3a9560d2c7dd9a8cf5b22))
 * fix(build): add nix-community substituter ([`c89cbea`](https://github.com/z0w13/tulpje/commit/c89cbeae1bfbad9fdcfb1d9f925e648d098e05ea))
 * feat(ci): add github ci and docker image workflows ([`cd150ab`](https://github.com/z0w13/tulpje/commit/cd150ab5a79d3c25d4a3c0aea74831be79ce3026))
 * build(deps): bump tokio from 1.43.0 to 1.44.2 ([`7bb5707`](https://github.com/z0w13/tulpje/commit/7bb5707683ebb1de6fd89d84a6e1ed447eb197ca))
 * fix(ci): make dependabot target dev branch ([`c8209fa`](https://github.com/z0w13/tulpje/commit/c8209fad24f17348fbb002b229da7dbdf8d172ea))
 * build: get local development with docker working again ([`1206529`](https://github.com/z0w13/tulpje/commit/12065291b3ec449bd23fe11c7c5a5e0d76420e1f))
 * chore: update twilight-gateway-queue to 5f5e0c1 ([`48d0763`](https://github.com/z0w13/tulpje/commit/48d07634508ac8285fd12f4d85fb64489c1e97c4))
 * fix(build/push): don't show double : in image name ([`e7375a0`](https://github.com/z0w13/tulpje/commit/e7375a019b3f270fb6fd7aec54b0cf9bbe40dd32))
 * wip: fix ability to run `docker compose up` ([`82e1f2a`](https://github.com/z0w13/tulpje/commit/82e1f2ad4a90f8f5dc41b6c0159b65910dba8b19))
</details>

## [0.17.0] - 2025-04-02

### Breaking Changes

 - Switch to nix based images and devenv
 - Remove dotenvy
 - Use figment instead of serde_envfile

### Added

 - Add rust-toolchain.toml

### Changed

 - Move TASK_SLOT parsing into shared crate
 - Use single build.rs for both handler & gateway
 - Replace vergen_gitcl with simple code doing the same thing
 - Skip hidden files (dotfiles)
 - Make secret path configurable using SECRET_LOADER_PATH env var
 - Bump tokio-util from 0.7.13 to 0.7.14 in [#19](https://github.com/z0w13/tulpje/pull/19)
 - Bump chrono from 0.4.39 to 0.4.40 in [#17](https://github.com/z0w13/tulpje/pull/17)
 - Bump uuid from 1.15.1 to 1.16.0 in [#16](https://github.com/z0w13/tulpje/pull/16)
 - Bump serde_json from 1.0.138 to 1.0.140 in [#15](https://github.com/z0w13/tulpje/pull/15)
 - Bump reqwest from 0.12.9 to 0.12.15 in [#20](https://github.com/z0w13/tulpje/pull/20)

### Fixed

 - Don't need init on the twilight containers
 - Don't error with custom IMAGE_TAG
 - Redis should have feature `tokio-comp` not `aio`

### Commit Details

<details><summary>view details</summary>

 * feat!: switch to nix based images and devenv ([`17bdc9c`](https://github.com/z0w13/tulpje/commit/17bdc9c9b0695f03ee84848f738022bc2102e407))
 * fix(compose): don't need init on the twilight containers ([`dab326d`](https://github.com/z0w13/tulpje/commit/dab326da33bf43bb91f0afb4ce6fe0036a0983e0))
 * refactor: move TASK_SLOT parsing into shared crate ([`40ba656`](https://github.com/z0w13/tulpje/commit/40ba656449294f26e3c268a0c75708967093f3a3))
 * refactor!: remove dotenvy ([`abb674e`](https://github.com/z0w13/tulpje/commit/abb674e15db05f744b2f899700bfbef4c3f81e9e))
 * refactor!: use figment instead of serde_envfile ([`88b0993`](https://github.com/z0w13/tulpje/commit/88b09931468ff14d977c841cda9edf9bd724135c))
 * refactor: use single build.rs for both handler & gateway ([`41245cd`](https://github.com/z0w13/tulpje/commit/41245cd86896a5f0ed5780ddd7f1d172d7a34f18))
 * refactor: replace vergen_gitcl with simple code doing the same thing ([`47a8608`](https://github.com/z0w13/tulpje/commit/47a8608ae89b094df18d1d9f31fa80bd4a402be3))
 * feat(utils/secret-loader): skip hidden files (dotfiles) ([`8b0a50a`](https://github.com/z0w13/tulpje/commit/8b0a50afca3d4cc1d623f5c7e6ce69f92dedc78d))
 * feat(utils/secret-loader): make secret path configurable using SECRET_LOADER_PATH env var ([`cb52886`](https://github.com/z0w13/tulpje/commit/cb528866dc8183b93167224154f841452e09bc96))
 * chore(build): add rust-toolchain.toml ([`7c275c0`](https://github.com/z0w13/tulpje/commit/7c275c0420434b93aab1447be725d29800e6c594))
 * fix(build/push): don't error with custom IMAGE_TAG ([`17adfa9`](https://github.com/z0w13/tulpje/commit/17adfa96adddb8642e10389e17e8d28888ea081b))
 * fix(handler): redis should have feature `tokio-comp` not `aio` ([`89b3522`](https://github.com/z0w13/tulpje/commit/89b35222f4bc99d8a03baceb6ee66d0da80ed4e6))
 * build(deps): bump tokio-util from 0.7.13 to 0.7.14 ([`bf5a40c`](https://github.com/z0w13/tulpje/commit/bf5a40cbcb61972300bde05ce833e12a8fcc0f6c))
 * build(deps): bump chrono from 0.4.39 to 0.4.40 ([`65a2452`](https://github.com/z0w13/tulpje/commit/65a2452173fec94286489bde93caad9af5b02ae8))
 * build(deps): bump uuid from 1.15.1 to 1.16.0 ([`7cc4bd4`](https://github.com/z0w13/tulpje/commit/7cc4bd428166d713690c8e6a6b6cc3cc77099d61))
 * build(deps): bump serde_json from 1.0.138 to 1.0.140 ([`551fc09`](https://github.com/z0w13/tulpje/commit/551fc09bce0e23187a2271c076726170f0de3170))
 * build(deps): bump reqwest from 0.12.9 to 0.12.15 ([`d076ae8`](https://github.com/z0w13/tulpje/commit/d076ae8aae986397a86efde3a1d80a3761bb9790))
</details>

## [0.16.0] - 2025-03-10

### Breaking Changes

 - Use redis-rs directly instead of through bb8 pool

### Changed

 - Bump vergen-gitcl from 1.0.2 to 1.0.5 in [#6](https://github.com/z0w13/tulpje/pull/6)
 - Bump redis from 0.28.2 to 0.29.1 in [#11](https://github.com/z0w13/tulpje/pull/11)
 - Bump sqlx from 0.8.2 to 0.8.3 in [#9](https://github.com/z0w13/tulpje/pull/9)
 - Bump uuid from 1.13.2 to 1.15.1 in [#12](https://github.com/z0w13/tulpje/pull/12)
 - Bump ring from 0.17.8 to 0.17.13 in [#13](https://github.com/z0w13/tulpje/pull/13)
 - Bump serde from 1.0.216 to 1.0.219 in [#14](https://github.com/z0w13/tulpje/pull/14)
 - Bump uuid from 1.11.0 to 1.13.2 in [#2](https://github.com/z0w13/tulpje/pull/2)
 - Bump async-trait from 0.1.83 to 0.1.86 in [#4](https://github.com/z0w13/tulpje/pull/4)
 - Bump serde_json from 1.0.133 to 1.0.138 in [#1](https://github.com/z0w13/tulpje/pull/1)
 - Bump metrics-exporter-prometheus from 0.16.0 to 0.16.2 in [#5](https://github.com/z0w13/tulpje/pull/5)
 - Bump tokio from 1.42.0 to 1.43.0 in [#3](https://github.com/z0w13/tulpje/pull/3)
 - Disable chrono wasmbind feature for our code
 - Enable dependabot

### Fixed

 - Update tokio-websockets to v0.11.3
 - Don't use a subshell while parsing .env into env vars

### Removed

 - Remove indirect dependency on aws-lc-rs

### Commit Details

<details><summary>view details</summary>

 * build(deps): bump vergen-gitcl from 1.0.2 to 1.0.5 ([`1d815db`](https://github.com/z0w13/tulpje/commit/1d815dbc37c5fc122a0fd8f559990a42a473454d))
 * build(deps): bump redis from 0.28.2 to 0.29.1 ([`0c7fea0`](https://github.com/z0w13/tulpje/commit/0c7fea0b667bb7dd32bf6f0aa9212c5b630a0568))
 * build(deps): bump sqlx from 0.8.2 to 0.8.3 ([`0f98dae`](https://github.com/z0w13/tulpje/commit/0f98dae9dfcf24e792bb65b9dc0fd9366f9b838f))
 * build(deps): bump uuid from 1.13.2 to 1.15.1 ([`03bd122`](https://github.com/z0w13/tulpje/commit/03bd122951b391c4c56b233e7745f230925bf167))
 * build(deps): bump ring from 0.17.8 to 0.17.13 ([`0bf283a`](https://github.com/z0w13/tulpje/commit/0bf283a31c38d8c8e1600ea25ecd50ace16bf7c5))
 * build(deps): bump serde from 1.0.216 to 1.0.219 ([`2d4e975`](https://github.com/z0w13/tulpje/commit/2d4e975abe6f93c8e06ef20d63928f0156d4389f))
 * build(deps): bump uuid from 1.11.0 to 1.13.2 ([`723de3d`](https://github.com/z0w13/tulpje/commit/723de3d1e143c95a1d9c6013905a2a4d81070854))
 * build(deps): bump async-trait from 0.1.83 to 0.1.86 ([`da99ce7`](https://github.com/z0w13/tulpje/commit/da99ce7c4242dd76762caec46565805827286fcd))
 * build(deps): bump serde_json from 1.0.133 to 1.0.138 ([`66da8bd`](https://github.com/z0w13/tulpje/commit/66da8bdc2851a0e5ca287742bb11c455b8258976))
 * build(deps): bump metrics-exporter-prometheus from 0.16.0 to 0.16.2 ([`372fc08`](https://github.com/z0w13/tulpje/commit/372fc08f2a7ada13fd7c6f5ed200369b5be5f3f6))
 * build(deps): bump tokio from 1.42.0 to 1.43.0 ([`d4e6ba6`](https://github.com/z0w13/tulpje/commit/d4e6ba620d929564a901dbdba7e73796e07b33b6))
 * chore: disable chrono wasmbind feature for our code ([`429b4a4`](https://github.com/z0w13/tulpje/commit/429b4a4a6513837d5cd22ce5bcaad4ea04bab40a))
 * fix: update tokio-websockets to v0.11.3 ([`429eb6c`](https://github.com/z0w13/tulpje/commit/429eb6cd91b700f5a7931af85e29a6595d6d10a1))
 * build: enable dependabot ([`fcf5461`](https://github.com/z0w13/tulpje/commit/fcf54611022eab07fe64a3246a9262af7dd56eaf))
 * fix(build): don't use a subshell while parsing .env into env vars ([`3033cd1`](https://github.com/z0w13/tulpje/commit/3033cd197851cdd69cf17928bfb54a76bea245a1))
 * refactor!: use redis-rs directly instead of through bb8 pool ([`61a548a`](https://github.com/z0w13/tulpje/commit/61a548abb36de63ee410cfa8a662e221478f14a8))
 * fix(shared): remove indirect dependency on aws-lc-rs ([`55a3bbf`](https://github.com/z0w13/tulpje/commit/55a3bbff4ed263437d01d90bb56fd63d2e728ef1))
</details>

## [0.15.0] - 2025-01-16

### Breaking Changes

 - Use env var for RUST_LOG instead of secret
 - Update twilight to 0.16.0

### Changed

 - Reduce log level for gateway messages to trace
 - Reduce amqp message logging level to trace
 - Use cache for checking if emojis belong to a guild

### Fixed

 - Add length limit to fronter category name
 - Update references to PluralKit command names in error messages

### Commit Details

<details><summary>view details</summary>

 * build(release): correctly tag independent crates without a release ([`f9ee0b6`](https://github.com/z0w13/tulpje/commit/f9ee0b6bdae27c28aa75403b75ba0c449bcf3232))
 * build(release): correctly detect tags for independent crates ([`2ad38f0`](https://github.com/z0w13/tulpje/commit/2ad38f006274e6ec9d65de083856c963aa80294b))
 * build(compose)!: use env var for RUST_LOG instead of secret ([`36b59e7`](https://github.com/z0w13/tulpje/commit/36b59e75f657bfa151959c45a5b963d9d6e7487f))
 * build(compose): remove unnecessary env var expansion ([`5942a66`](https://github.com/z0w13/tulpje/commit/5942a66e9e6b2e1098ec413fbd41149a9ea4f273))
 * fix(handler/pk): add length limit to fronter category name ([`acb0f5a`](https://github.com/z0w13/tulpje/commit/acb0f5aaf207fccc135a9d5d860405b893c27b4c))
 * fix(handler/pk): update references to PluralKit command names in error messages ([`3b102da`](https://github.com/z0w13/tulpje/commit/3b102da825bff01dc4b7acea0d71b2f462911de2))
 * chore(gateway): reduce log level for gateway messages to trace ([`d77205e`](https://github.com/z0w13/tulpje/commit/d77205e023730a0ec3d50c0a70ea68f23e080aae))
 * chore: reduce amqp message logging level to trace ([`4b9a0b7`](https://github.com/z0w13/tulpje/commit/4b9a0b7bedf7dc4869386d7b3fa5e9083bfffc52))
 * feat(handler/emoji): use cache for checking if emojis belong to a guild ([`a5dc9d1`](https://github.com/z0w13/tulpje/commit/a5dc9d1098553eec7f7daebb62257fd2aa5e60a7))
 * feat: implemented tulpje-cache, a redis based caching library ([`6710502`](https://github.com/z0w13/tulpje/commit/6710502612beb7e00fd5324502f6ea55bd4b0ea7))
 * chore!: update twilight to 0.16.0 ([`a974856`](https://github.com/z0w13/tulpje/commit/a9748566df2e386a97c6921c649cec57879fc456))
</details>

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
