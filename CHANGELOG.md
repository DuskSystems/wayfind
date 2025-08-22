# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

- Support `no-std` builds.
- `Router::get` and `Router::get_mut` methods to access data from pre-existing templates.

### Changed

- Raised MSRV from 1.63 to 1.85.
- Switch to Rust 2024 edition.
- Switched internal data store to slot storage.
- Syntax for parameters has changed from `{name}` to `<name>`
- Parameter characters `<` and `>` can no longer be escaped.

### Removed

- Removed OCI example.
- Constraints are no longer supported.
- Optional groups are no longer supported.

## [0.8.1] - 2025-01-07

### Fixed

- Remove required lifetime from router.
- Improved detection for structural conflicts.

## [0.8.0] - 2025-01-07

### Changed

- The router no longer performs percent-decoding.
- Searches no longer error on any invalid UTF-8 parameters, instead acting as if the path doesn't match instead.
- Error messages are now more consistent.
- The term 'route' has been replaced with 'template'.
- Lowered MSRV from 1.66 to 1.63.

### Removed

- The `Routable` wrapper is no longer needed. The router uses strings for inserts.
- The `Path` wrapper is no longer needed. The router uses strings for searches.
- `EncodingError` has been removed.
- `PathError` has been removed.
- `RoutableError` has been removed.

### Fixed

- Priority logic now correctly prefers depth, then length, of templates.

## [0.7.0] - 2024-11-12

### Added

- Support builder pattern for constructing routables.

### Changed

- `Router` now requires an explicit lifetime.
- All routes now require a leading `/`.
- Router display no longer shows internal root node. Matchable nodes are now denoted by `[*]`.
- Route expander now converts empty routes `""` to `"/"`.
- `Parameters` lists make use of `smallvec` now.

### Removed

- Individual parameters are returned as tuples, rather than `Parameter`.

### Fixed

- Touching parameters are now correctly rejected.

## [0.6.0] - 2024-10-27

### Added

- Added support for optional groups in routes.
- Added support for inline wildcards.

### Changed

- Inserts and deletes should be much more efficient now.
- Syntax for escaping parameters has changed to use the `\` character.
- Route errors now have more consistent error messages.
- Duplicate route error now shows which route caused the conflict.
- Added priority ranking to routes.

### Removed

- Optional parameters have been replaced with optional groups.
- Optional trailing slashes have been replaced with optional groups.

## [0.5.0] - 2024-09-11

### Fixed

- Ensure routes can only be deleted via the exact same inserted route.

### Changed

- Router display is more compact now.

## [0.4.0] - 2024-09-11

### Added

- Optional trailing slashes are now supported.

### Fixed

- Route error messages are now correctly indented.

## [0.3.0] - 2024-09-11

### Added

- Optional parameters are now supported in routes.

### Changed

- Successful matches now return a flattened representation of node data.
- Route encoding errors now live in the encoding error enum.
- Router display now uses different characters to represent root and matchable nodes.

### Fixed

- Router delete method now checks for encoded routes.
- Be consistent with the use of terms "path" and "route".

## [0.2.1] - 2024-09-04

### Changed

- Added OCI example.

### Fixed

- Router display no longer relies on generic being displayable.

## [0.2.0] - 2024-08-29

### Changed

- Search method now returns a result.

### Removed

- Removed all usages of unsafe code.
- Removed smallvec dependency.

### Security

- Resolved incorrect unsafe usage in parameter extraction.

## [0.1.0] - 2024-08-29

### Added

- Initial implementation of `wayfind`.

[unreleased]: https://github.com/DuskSystems/wayfind/compare/v0.8.1...HEAD
[0.8.1]: https://github.com/DuskSystems/wayfind/compare/v0.8.0...v0.8.1
[0.8.0]: https://github.com/DuskSystems/wayfind/compare/v0.7.0...v0.8.0
[0.7.0]: https://github.com/DuskSystems/wayfind/compare/v0.6.0...v0.7.0
[0.6.0]: https://github.com/DuskSystems/wayfind/compare/v0.5.0...v0.6.0
[0.5.0]: https://github.com/DuskSystems/wayfind/compare/v0.4.0...v0.5.0
[0.4.0]: https://github.com/DuskSystems/wayfind/compare/v0.3.0...v0.4.0
[0.3.0]: https://github.com/DuskSystems/wayfind/compare/v0.2.1...v0.3.0
[0.2.1]: https://github.com/DuskSystems/wayfind/compare/v0.2.0...v0.2.1
[0.2.0]: https://github.com/DuskSystems/wayfind/compare/v0.1.0...v0.2.0
[0.1.0]: https://github.com/DuskSystems/wayfind/releases/tag/v0.1.0
