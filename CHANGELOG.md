# [0.7.0](https://github.com/answerbook/vrl/compare/v0.6.0...v0.7.0) (2023-12-18)


### Bug Fixes

* arithmetic overflow panic (#252) [1ff642d](https://github.com/answerbook/vrl/commit/1ff642dc940c7d07ea29627b010806086f968953) - GitHub


### Chores

* add badges to README (#236) [43df337](https://github.com/answerbook/vrl/commit/43df3373c03125d53ee1638909227ce9eca66955) - GitHub
* add simple example (#231) [e007923](https://github.com/answerbook/vrl/commit/e00792337b8dce414fa8ca8c535753d5536e6df7) - GitHub
* consolidate internal crates (#235) [8f13cd2](https://github.com/answerbook/vrl/commit/8f13cd241565e7e85eb541ca3be483b1192cbb5c) - GitHub
* **deps**: Bump rust-toolchain.toml to 1.69.0 (#253) [a1ec917](https://github.com/answerbook/vrl/commit/a1ec91715140a510086e83fba0b880bf20a3588e) - GitHub
* **deps**: update afl requirement from 0.12.17 to 0.13.0 (#256) [c5fa9ce](https://github.com/answerbook/vrl/commit/c5fa9ce726c69cb12a461fa05a6bdf7d1f3ca648) - GitHub
* **deps**: update criterion requirement from 0.4 to 0.5 (#255) [c7c95b8](https://github.com/answerbook/vrl/commit/c7c95b855f74e0f9e9ce35b06587712ccaed9050) - GitHub
* **deps**: update dns-lookup requirement from 1.0.8 to 2.0.1 (#224) [6500539](https://github.com/answerbook/vrl/commit/6500539808cff2f21d5e9bd80e5e85cb4e2f560a) - GitHub
* **deps**: update indexmap requirement from ~1.9.2 to ~2.0.0 (#279) [078f47a](https://github.com/answerbook/vrl/commit/078f47aac45d1cdddf76c7ffbdf239c55e14376c) - GitHub
* **deps**: update itertools requirement from 0.10.5 to 0.11.0 (#276) [c7572ff](https://github.com/answerbook/vrl/commit/c7572ff0597b86d8c4232eddb43ebcea60b7f484) - GitHub
* **deps**: update lalrpop requirement from 0.19.8 to 0.20.0 (#225) [04941e7](https://github.com/answerbook/vrl/commit/04941e71487166c9746a504bf6fcddefd198b13d) - GitHub
* **deps**: update lalrpop-util requirement from 0.19 to 0.20 (#223) [3ac0149](https://github.com/answerbook/vrl/commit/3ac0149cf5e853a309eb7887f7356c44e54b6314) - GitHub
* **deps**: update quoted_printable requirement from 0.4.7 to 0.5.0 (#275) [b322236](https://github.com/answerbook/vrl/commit/b322236a35fd5a957b17c6c18acaf75f1ca86a50) - GitHub
* **deps**: update rustyline requirement from 11 to 12 (#278) [72315d6](https://github.com/answerbook/vrl/commit/72315d674b02f4246c736bb446bffbff601fd76a) - GitHub
* prepare for crates.io release (#234) [0de9b0c](https://github.com/answerbook/vrl/commit/0de9b0c3cad5392a0c8a1e41d70ef6ad5af265e5) - GitHub
* Regenerate license inventory (#248) [ff35551](https://github.com/answerbook/vrl/commit/ff35551f22034d567c3a6a1b1355ace63be4cd00) - GitHub
* remove `getrandom` dependency (#237) [d9fa124](https://github.com/answerbook/vrl/commit/d9fa124f29002568e49e03667ebd30df8fdf4f6b) - GitHub
* Set up 3rd party license list file (#222) [cfa9aad](https://github.com/answerbook/vrl/commit/cfa9aadbd19770def92cfefcf2853810efbb955b) - GitHub
* Update README (#230) [ecfb31e](https://github.com/answerbook/vrl/commit/ecfb31e7bae6143814fa727b7a256c5cc0215567) - GitHub


### Features

* add from_unix_timestamp function (#277) [5739fdd](https://github.com/answerbook/vrl/commit/5739fddd20b6c8f4ed76890da2c49a5df8109c7f) - GitHub
* add fuzzer (#245) [935e2ab](https://github.com/answerbook/vrl/commit/935e2ab91d2c3a9f536e499459121361e8eefe28) - GitHub
* give access to the program result type definition (#262) [2bbe672](https://github.com/answerbook/vrl/commit/2bbe6728fc0a9d0c20b6ee8634346bdbf28f78d8) - GitHub
* **stdlib**: Add `ingress_upstreaminfo` log format to `parse_nginx_log` function (#193) [64a5cac](https://github.com/answerbook/vrl/commit/64a5cac56c8e80d18c17a4b1b06110d49790446f) - GitHub
* **stdlib**: Add timezone argument to format_timestamp (#247) [52959f8](https://github.com/answerbook/vrl/commit/52959f8c24d5524c6e0a4eaef83962049fbfe8f6) - GitHub


### Miscellaneous

* Merge pull request #35 from answerbook/darinspivey/LOG-18807 [34285e3](https://github.com/answerbook/vrl/commit/34285e38b19bd425f8d2d231c7484a08ed0b2db5) - GitHub [LOG-18807](https://logdna.atlassian.net/browse/LOG-18807)
* Merge branch 'from-upstream-0.5.0' into upstream-0.5.0 [73716ee](https://github.com/answerbook/vrl/commit/73716ee7050ed9865fcab64877b8a26060f115a8) - Darin Spivey
* prepare for 0.5.0 release (#281) [e82e7eb](https://github.com/answerbook/vrl/commit/e82e7ebfca7981c6570b03de093cb52974e0a1b2) - GitHub
* Added null escape to lexer (#259) [bab67a5](https://github.com/answerbook/vrl/commit/bab67a5d832eef396c7612510b539e24c479f3f1) - GitHub
* changelog (#260) [c5c607f](https://github.com/answerbook/vrl/commit/c5c607f748302f8ab5f62501384b5655d445e485) - GitHub
* fix typedefs with query side-effects (#258) [d2c4d84](https://github.com/answerbook/vrl/commit/d2c4d841296f7c9e5fd735684f0c536b6708f148) - GitHub
* remove stdlib function features (#251) [b308189](https://github.com/answerbook/vrl/commit/b308189e6966df1e61416f9b7a4e5d36e211fc80) - GitHub

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
