# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.3.2] - 2021-07-30
### Changed
- Update dependencies.  [#61], [#63], [#64], [#66], [#67], [#73], [#77], [#82], [#87], [#88]
- Update docs to recommend installing with `--locked`.  [#79]
- Add license check to CI runner.  [#74]

## [0.3.1] - 2021-05-13
### Changed
- Add backoff-retry behavior to coldsnap uploads.  [#56]
- Update dependencies.  [#48], [#50], [#51], [#54], [#55], [#58], [#60]
- Fix clippy warnings for Rust 1.52.  [#57]

## [0.3.0] - 2021-02-25
### Breaking Changes
- Updated tokio to v1, this is a breaking change when using coldsnap as a library. [#39]

### Changed
- Fix an issue with download filepaths [#40]

## [0.2.0] - 2020-11-11
### Changed
- Added Cargo.toml features to switch between rusoto native-tls and rustls. [#18]

## [0.1.0] - 2020-08-05
### Added
- Everything!

[0.3.2]: https://github.com/awslabs/coldsnap/compare/v0.3.1...v0.3.2
[0.3.1]: https://github.com/awslabs/coldsnap/compare/v0.3.0...v0.3.1
[0.3.0]: https://github.com/awslabs/coldsnap/compare/v0.2.0...v0.3.0
[0.2.0]: https://github.com/awslabs/coldsnap/compare/v0.1.0...v0.2.0
[0.1.0]: https://github.com/awslabs/coldsnap/releases/tag/v0.1.0

[#18]: https://github.com/awslabs/coldsnap/pull/18
[#39]: https://github.com/awslabs/coldsnap/pull/39
[#40]: https://github.com/awslabs/coldsnap/pull/40
[#48]: https://github.com/awslabs/coldsnap/pull/48
[#50]: https://github.com/awslabs/coldsnap/pull/50
[#51]: https://github.com/awslabs/coldsnap/pull/51
[#54]: https://github.com/awslabs/coldsnap/pull/54
[#55]: https://github.com/awslabs/coldsnap/pull/55
[#56]: https://github.com/awslabs/coldsnap/pull/56
[#57]: https://github.com/awslabs/coldsnap/pull/57
[#58]: https://github.com/awslabs/coldsnap/pull/58
[#60]: https://github.com/awslabs/coldsnap/pull/60
[#61]: https://github.com/awslabs/coldsnap/pull/61
[#63]: https://github.com/awslabs/coldsnap/pull/63
[#64]: https://github.com/awslabs/coldsnap/pull/64
[#66]: https://github.com/awslabs/coldsnap/pull/66
[#67]: https://github.com/awslabs/coldsnap/pull/67
[#73]: https://github.com/awslabs/coldsnap/pull/73
[#74]: https://github.com/awslabs/coldsnap/pull/74
[#77]: https://github.com/awslabs/coldsnap/pull/77
[#79]: https://github.com/awslabs/coldsnap/pull/79
[#82]: https://github.com/awslabs/coldsnap/pull/82
[#87]: https://github.com/awslabs/coldsnap/pull/87
[#88]: https://github.com/awslabs/coldsnap/pull/88
