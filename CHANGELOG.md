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
