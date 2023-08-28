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
