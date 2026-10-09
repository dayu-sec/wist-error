# Changelog

All notable changes to `wist-error` are documented here. The format follows
[Keep a Changelog](https://keepachangelog.com/en/1.1.0/) and this project adheres to
[Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.2.0] - 2026-10-09

### Changed

- Bump `orion-error` from 0.8 to 0.9 to align on its error-identity traits
  (`OperationContext` is now pure data; automatic lifecycle logging moved to the
  non-`Clone` `AutoLogGuard`). The reason vocabulary is unchanged, but consumers must now
  use `orion-error` 0.9 as well, so the dependency graph does not carry two `DomainReason`
  versions side by side.
