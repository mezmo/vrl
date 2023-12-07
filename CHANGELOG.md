# [0.6.0](https://github.com/answerbook/vrl/compare/v0.5.1...v0.6.0) (2023-11-21)


### Features

* Expose fallible functions mezmo_to_object() and mezmo_to_array() [5afe456](https://github.com/answerbook/vrl/commit/5afe4564a8aa7b8feda0ba19d3a950f1c5a82552) - Jorge Bay [LOG-17868](https://logdna.atlassian.net/browse/LOG-17868)

## [0.5.1](https://github.com/answerbook/vrl/compare/v0.5.0...v0.5.1) (2023-09-12)


### Bug Fixes

* use `$crate` with the `value` macro so it works when re-exported (#216) [8abd049](https://github.com/answerbook/vrl/commit/8abd0492ae51e0d8ba869fc22896570deb74681c) - GitHub


### Chores

* add `arbitrary` and `lua` features (#220) [a0b9892](https://github.com/answerbook/vrl/commit/a0b9892ab90e6b081ce720c22134d71d207eed73) - GitHub
* **ci**: bump actions/add-to-project from 0.4.1 to 0.5.0 (#184) [4d0fda2](https://github.com/answerbook/vrl/commit/4d0fda2325d6a36a11f3477b37381ed85b313643) - GitHub
* crate cleanup (#190) [566456d](https://github.com/answerbook/vrl/commit/566456d90d1ae4cc80e8127bb230cde384cf48dc) - GitHub
* migrate path logic for `parse_grok` (#202) [d42fe98](https://github.com/answerbook/vrl/commit/d42fe98a5e65ea59e028151b56180c54f5a31046) - GitHub
* Migrate stdlib `get` function to new path code (#200) [c1414e2](https://github.com/answerbook/vrl/commit/c1414e2bb1e9141723d1134bb0c656170333126f) - GitHub
* move prelude from `stdlib` to `compiler` and re-export it from `vrl` (#218) [4b98f54](https://github.com/answerbook/vrl/commit/4b98f5491bdce4c4784d9ec45c1eb8eb90308d64) - GitHub
* Move some functionality from `core` to `compiler` (#217) [2d877ad](https://github.com/answerbook/vrl/commit/2d877ade58bb4e27d18ee91a73b02c703deb24db) - GitHub
* re-export all sub-crates in `vrl` (#213) [efa13f1](https://github.com/answerbook/vrl/commit/efa13f14ecd6e2b7985a10d37daf42e5eb5c8212) - GitHub
* remove `Value::get_by_path` and `Value::insert_by_path` (#204) [00369f4](https://github.com/answerbook/vrl/commit/00369f4a02fdd7f4d7601f5994f0020c525e062c) - GitHub
* remove `vrl` as a dependency of the `stdlib` crate (#209) [0e05c24](https://github.com/answerbook/vrl/commit/0e05c2440487397600b594d0d99cf9cb7b88a1a9) - GitHub
* remove `vrl` as a dependency of the `tests` crate (#208) [247384a](https://github.com/answerbook/vrl/commit/247384a047db91ce3bf4f40e42d3b7f5f8b3dc56) - GitHub
* remove `vrl` as a dev-dependency of the `stdlib` crate (#210) [c3447c3](https://github.com/answerbook/vrl/commit/c3447c351cef26995d68ccf40f6dc01a61d7b4d3) - GitHub
* remove deprecated `FieldBuf` from `Field` (#199) [a23ad33](https://github.com/answerbook/vrl/commit/a23ad33340d765c42f9bac43d4edbcf4d5468ad7) - GitHub
* remove lookup v1 (#207) [e723f96](https://github.com/answerbook/vrl/commit/e723f96c9305d77c853d8f95a1955ff393c8aff1) - GitHub
* remove web playground (#185) [6505478](https://github.com/answerbook/vrl/commit/650547870a16c66dcfab01ec382cfdc23415d85b) - GitHub
* rename the `lookup` crate to `path` (#212) [b623b61](https://github.com/answerbook/vrl/commit/b623b615a6ca69c2466dc09a7cac394545cde597) - GitHub


### Miscellaneous

* Merge pull request #33 from update upstream to v0.3.0 [00d4830](https://github.com/answerbook/vrl/commit/00d4830c7317decd57eab04f5cec62a9fb09b0c3) - GitHub
* Merge branch 'main' [7ad96ac](https://github.com/answerbook/vrl/commit/7ad96ac8ae9554fda14280a3d159a05bf861d8b8) - Jorge Bay
* Merge tag 'v0.3.0' from upstream [1062d7d](https://github.com/answerbook/vrl/commit/1062d7d5260cb5535f015ef79eb14b72296bf202) - Jorge Bay
* Merge branch 'main' of github.com:vectordotdev/vrl [113005b](https://github.com/answerbook/vrl/commit/113005bcee6cd7b5ea0a53a7db2fc45ba4bc4125) - Nathan Fox
* save (#221) [0da83a0](https://github.com/answerbook/vrl/commit/0da83a0ba7df2065b272895280b9cac86fb8044d) - GitHub
* save [bc8b290](https://github.com/answerbook/vrl/commit/bc8b290725403977ede81923c86accf535eaac9f) - Nathan Fox
* save (#214) [b933b1f](https://github.com/answerbook/vrl/commit/b933b1fedc73d67334210da93113b60d217a78a3) - GitHub
* save (#205) [42c1764](https://github.com/answerbook/vrl/commit/42c17644a06ca955b11258b0ce211b163aa91205) - GitHub
* save (#201) [3ba6a85](https://github.com/answerbook/vrl/commit/3ba6a8572d1de5ec0ff98f5d925336cb5c54b389) - GitHub
* save (#196) [1299360](https://github.com/answerbook/vrl/commit/129936074cfe114672042f4296d90cdb2533f547) - GitHub

# [0.5.0](https://github.com/answerbook/vrl/compare/v0.4.3...v0.5.0) (2023-09-08)


### Features

* New fallible concat or add function [6cb9a0c](https://github.com/answerbook/vrl/commit/6cb9a0c1c67686efecec2da79e256d7fd40a47ec) - Jorge Bay [LOG-17858](https://logdna.atlassian.net/browse/LOG-17858)

## [0.4.3](https://github.com/answerbook/vrl/compare/v0.4.2...v0.4.3) (2023-08-28)


### Bug Fixes

* **build**: bump rust image version to latest (#31) [77c4e70](https://github.com/answerbook/vrl/commit/77c4e7050052d17d0279dfb054917bcb00a5ca47) - GitHub [LOG-17986](https://logdna.atlassian.net/browse/LOG-17986)

## [0.4.2](https://github.com/answerbook/vrl/compare/v0.4.1...v0.4.2) (2023-07-27)


### Bug Fixes

* Narrow return types for mezmo_concat_or_add() (#30) [4cbe11e](https://github.com/answerbook/vrl/commit/4cbe11e3ef317efd7e9cebe4719732a34a544f88) - GitHub [LOG-17443](https://logdna.atlassian.net/browse/LOG-17443)

## [0.4.1](https://github.com/answerbook/vrl/compare/v0.4.0...v0.4.1) (2023-07-26)


### Bug Fixes

* Invalid return types for mezmo index funcs (#29) [d0d84f5](https://github.com/answerbook/vrl/commit/d0d84f51482a8d64acbbd019b5f340b14caa4e63) - GitHub [LOG-17641](https://logdna.atlassian.net/browse/LOG-17641)

# [0.4.0](https://github.com/answerbook/vrl/compare/v0.3.3...v0.4.0) (2023-07-26)


### Features

* Add functions to coerce int and float types (#28) [093162f](https://github.com/answerbook/vrl/commit/093162fb96fad1a53ec62c6072e8d51803d7119b) - GitHub [LOG-17443](https://logdna.atlassian.net/browse/LOG-17443)

## [0.3.3](https://github.com/answerbook/vrl/compare/v0.3.2...v0.3.3) (2023-07-12)


### Bug Fixes

* lookup segment handling when quoting is redundant (#25) [1cb9515](https://github.com/answerbook/vrl/commit/1cb9515d45a8a17e3b5b8affb6f9d0348577635f) - GitHub [LOG-17092](https://logdna.atlassian.net/browse/LOG-17092)

## [0.3.2](https://github.com/answerbook/vrl/compare/v0.3.1...v0.3.2) (2023-06-30)


### Bug Fixes

* Avoid panicking on regex error in parse_grok() function [9a96163](https://github.com/answerbook/vrl/commit/9a961632e3e4f2183e19a32e43437622786dcfc9) - GitHub [LOG-17425](https://logdna.atlassian.net/browse/LOG-17425)

## [0.3.1](https://github.com/answerbook/vrl/compare/v0.3.0...v0.3.1) (2023-06-30)


### Continuous Integration

* Setup repo in Jenkins [a5f877a](https://github.com/answerbook/vrl/commit/a5f877a5c8e68784d859bb1c4ae314803f0b5cb5) - Dan Hable [LOG-16869](https://logdna.atlassian.net/browse/LOG-16869)


### Miscellaneous

* Merge pull request #21 from answerbook/dhable/LOG-16869 [61b2f3c](https://github.com/answerbook/vrl/commit/61b2f3c2260fb061d1c943fa809c0517beeb26da) - GitHub [LOG-16869](https://logdna.atlassian.net/browse/LOG-16869)

# Changelog

## unreleased

## `0.7.0` (2023-09-25)

#### Bug Fixes
- `parse_nginx_log` doesn't fail if the values of key-value pairs in error logs is missing (https://github.com/vectordotdev/vrl/pull/442)
- `encode_gzip` and `encode_zlib` now correctly check the compression level (preventing a panic) (https://github.com/vectordotdev/vrl/pull/393)
- fix the type definition of array/object literal expressions where one of the values is undefined (https://github.com/vectordotdev/vrl/pull/401)
- `parse_aws_vpc_flow_log` now handles account-id value as a string, avoiding loss of leading zeros and case where value is `unknown` (https://github.com/vectordotdev/vrl/issues/263) 

#### Features
- `parse_key_value` can now parse values enclosed in single quote characters (https://github.com/vectordotdev/vrl/pull/382)
- added `pretty` parameter for `encode_json` vrl function to produce pretty-printed JSON string (https://github.com/vectordotdev/vrl/pull/370)
- added `community_id` function for generation of [V1 Community IDs](https://github.com/corelight/community-id-spec) (https://github.com/vectordotdev/vrl/pull/360)
- updated aws vpc flow log parsing to include version 5 fields (https://github.com/vectordotdev/vrl/issues/227)
- removed deprecated `to_timestamp` function (https://github.com/vectordotdev/vrl/pull/452)

## `0.6.0` (2023-08-02)

#### Bug Fixes

- enquote values containing `=` in `encode_logfmt` vrl function (https://github.com/vectordotdev/vector/issues/17855)
- breaking change to `parse_nginx_log()` to make it compatible to more unstandardized events (https://github.com/vectordotdev/vrl/pull/249)

#### Features

- deprecated `to_timestamp` vrl function (https://github.com/vectordotdev/vrl/pull/285)
- add support for chacha20poly1305, xchacha20poly1305, xsalsa20poly1305 algorithms for encryption/decryption (https://github.com/vectordotdev/vrl/pull/293)
- add support for resolving variables to `Expr::resolve_constant` (https://github.com/vectordotdev/vrl/pull/304)
- introduce new encryption/decryption algorithm options (`"AES-*-CTR-BE"`, `"AES-*-CTR-LE"`) https://github.com/vectordotdev/vrl/pull/299

## `0.5.0` (2023-06-28)
- added \0 (null) character literal to lex parser (https://github.com/vectordotdev/vrl/pull/259)
- added the `timezone` argument to the `format_timestamp` vrl function. (https://github.com/vectordotdev/vrl/pull/247)
- removed feature flags for each individual VRL function. (https://github.com/vectordotdev/vrl/pull/251)
- fixed a panic when arithmetic overflows. It now always wraps (only in debug builds). (https://github.com/vectordotdev/vrl/pull/252)
- `ingress_upstreaminfo` log format has been added to `parse_nginx_log` function (https://github.com/vectordotdev/vrl/pull/193)
- fixed type definitions for side-effects inside of queries (https://github.com/vectordotdev/vrl/pull/258)
- replaced `Program::final_type_state` with `Program::final_type_info` to give access to the type definitions of both the target and program result (https://github.com/vectordotdev/vrl/pull/262)
- added `from_unix_timestamp` vrl function (https://github.com/vectordotdev/vrl/pull/277)

## `0.4.0` (2023-05-11)
- consolidated all crates into the root `vrl` crate. The external API stayed the same, with the exception of macros, which are now all exported at the root of the `vrl` crate.
- published VRL to crates.io. Standard crate versioning will now be used instead of git tags.

## `0.3.0` (2023-05-05)
- fixed a type definition bug for assignments where the right-hand side of the assignment expression resolved to the `never` type
- removed the deprecated `FieldBuf` from `Field`
- removed the lookup v1 code
- renamed the `lookup` crate to `path`
- re-exported all sub-crates in the root `vrl` crate
- fix the `value` macro so it works when re-exported

## `0.2.0` (2023-04-03)
- added guard for the `limit` param of the `split` function to ensure it's not negative
- renamed `Expression::as_value` to `Expression::resolve_constant`
- `match` function now precompiles static regular expressions
- enabled the `encrypt` and `decrypt` VRL functions on the WASM playground
- update default branch to `main`
- the following VRL functions now compile on WASM (but abort at runtime)
  - `get_hostname`
  - `log'
  - `reverse_dns'
  - `parse_grok`
  - `parse_groks`

## `0.1.0` (2023-03-27)
- VRL was split from the Vector repo
