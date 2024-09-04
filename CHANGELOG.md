# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Changed

- Router display now uses different characters to represent root and matchable nodes.

## [0.2.1] - 2024-09-04

### Changed

- Added OCI example.

### Fixed

- Router display no longer relies on generic being displayable.

## [0.2.0] - 2024-08-29

### Changed

- Search method now returns a Result.

### Removed

- Removed all usages of unsafe code.
- Removed smallvec dependency.

### Security

- Resolved incorrect unsafe usage in parameter extraction.

## [0.1.0] - 2024-08-29

### Added

- Initial implementation of `wayfind`.

[unreleased]: https://github.com/DuskSystems/wayfind/compare/v0.2.1...HEAD
[0.2.1]: https://github.com/DuskSystems/wayfind/compare/v0.2.0...v0.2.1
[0.2.0]: https://github.com/DuskSystems/wayfind/compare/v0.1.0...v0.2.0
[0.1.0]: https://github.com/DuskSystems/wayfind/releases/tag/v0.1.0
