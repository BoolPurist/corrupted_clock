# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Fixed

- Now paused time should be shown calculated correctly.
- Start date now is treated as the local date time and not as a direct UTC

### Added

- Showing started at, last paused at and last resumed at for stopwatch/count down in the table 
- CLI argument within "create" subcommand. Allows to specify a start date of the stopwatch or count down
- Option on subcommand "Get" and "List" to print rows with a maximum number of columns


### Added

- User can now provide durations for a count down in 3 forms
  - Only seconds
  - Only seconds and minutes
  - With seconds, minutes and hours

## [0.2.0] - 2024-04-29 

- Minimal viable product
