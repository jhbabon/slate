# Change Log
All notable changes to this project will be documented in this file.
This project adheres to [Semantic Versioning](http://semver.org/).

## [Unreleased]

## [1.4.0] - 2017-11-08
### Removed
- `exec` command
- `snippet` command

### Fixed
- Don't change the env variables when running Config tests.

### Changed
- Unify code formatting with `rustfmt`.

## [1.3.0] - 2016-08-17
### Added
- Now you can use your own slate file by setting up the `SLATE_FILEPATH`
  env var. This must be a valid path. If the file doesn't exist
  it will be created by the program.

## [1.2.0] - 2016-08-16
### Changed
- Internal changes on how the result and errors of different functions are handled.
  Now internal IO errors, for example, will be displayed when the program fails.

## [1.1.1] - 2016-08-15
### Changed
- Use `std::process::Command` lib in the exec subcommand instead of the `exec` crate.

## [1.1.0] - 2016-06-30
### Added
- Add new option to get command, -n. It tells the command to not add
  a trailing newline character to the value extracted, as in the `echo`
  utility.

## [1.0.2] - 2016-05-12
### Fixed
- Fix install instructions. Use correct git repo url.

### Added
- Add more meta information in Cargo.toml

## [1.0.1] - 2016-05-11
### Added
- MIT License.

## 1.0.0 - 2016-05-11
### Added
- The subcommand `set` to save new values.
- The subcommand `get` to read existing values.
- The subcommand `list` to see all available keys.
- The subcommand `rename` to change keys' names.
- The subcommand `remove` to destroy existing keys or all of them.
- The subcommand `exec` to execute values as commands.
- The subcommand `snippet` to use values with changing content.
- The option `--help` message and one per subcommand.
- Tests for the main `Slate` structure.
- Use `~/.slate` file as main storage point.

[Unreleased]: https://github.com/jhbabon/slate/compare/v1.4.0...HEAD
[1.4.0]: https://github.com/jhbabon/slate/compare/v1.3.0...v1.4.0
[1.3.0]: https://github.com/jhbabon/slate/compare/v1.2.0...v1.3.0
[1.2.0]: https://github.com/jhbabon/slate/compare/v1.1.1...v1.2.0
[1.1.1]: https://github.com/jhbabon/slate/compare/v1.1.0...v1.1.1
[1.1.0]: https://github.com/jhbabon/slate/compare/v1.0.2...v1.1.0
[1.0.2]: https://github.com/jhbabon/slate/compare/v1.0.1...v1.0.2
[1.0.1]: https://github.com/jhbabon/slate/compare/v1.0.0...v1.0.1
