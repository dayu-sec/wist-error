# wist-error

Shared structured error vocabulary for the `wist` server side.

[![crates.io](https://img.shields.io/crates/v/wist-error.svg)](https://crates.io/crates/wist-error)
[![docs.rs](https://img.shields.io/docsrs/wist-error/latest.svg)](https://docs.rs/wist-error)
[![Downloads](https://img.shields.io/crates/d/wist-error.svg)](https://crates.io/crates/wist-error)
[![CI](https://github.com/dayu-sec/wist-error/actions/workflows/ci.yml/badge.svg)](https://github.com/dayu-sec/wist-error/actions/workflows/ci.yml)
[![codecov](https://codecov.io/gh/dayu-sec/wist-error/branch/main/graph/badge.svg)](https://codecov.io/gh/dayu-sec/wist-error)
[![dependency status](https://deps.rs/repo/github/dayu-sec/wist-error/status.svg)](https://deps.rs/repo/github/dayu-sec/wist-error)
[![License: Apache-2.0](https://img.shields.io/badge/license-Apache--2.0-blue.svg)](LICENSE)

`wist-error` centralizes the cross-crate reason enums — built on [`orion-error`] — and the
conversions between them, so the center-side crates share one error language instead of each
hand-rolling a `StoreError` / `ConfigError` / `Box<dyn Error>`.

[`orion-error`]: https://crates.io/crates/orion-error

## Exports

| Name          | Purpose                                              |
| ------------- | ---------------------------------------------------- |
| `AppError` / `AppReason` / `AppResult` | Application-level errors and reasons. |
| `ConfigError` / `ConfigReason` / `ConfigResult` | Configuration errors and reasons. |
| `StoreError` / `StoreReason` / `StoreResult` | Store errors and reasons. |
| `SysErrorCode` | Shared system error codes. |

Reasons are unit-shaped plus a transparent `General(UnifiedReason)`; dynamic diagnostics live on
the structured error as detail / context / source. Lifting a lower reason into `AppReason` uses
[`orion-error`]'s `ConvErr` via the `From` impls in `convert`.

## Related crates

- [`wist-contracts`](../wist-contracts) — shared contract and schema types.
- [`wist-shared`](../wist-shared) — shared helpers.

## License

[Apache-2.0](LICENSE)
