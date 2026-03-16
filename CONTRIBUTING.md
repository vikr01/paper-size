# Contributing to paper-size

## Setup

```sh
git clone https://github.com/vikr01/paper-size.git
cd paper-size
cargo test
```

For WASM verification:

```sh
rustup target add wasm32-unknown-unknown
cargo build --target wasm32-unknown-unknown --no-default-features
```

## Pre-commit checklist

All must pass before pushing:

```sh
cargo fmt --all
cargo clippy -- -D warnings
cargo test
cargo test --no-default-features
cargo build --target wasm32-unknown-unknown --no-default-features
cargo doc --no-deps
```

CI enforces the same checks on every push and pull request.

## Architecture

This is a single-file library crate (`src/lib.rs`) with zero dependencies.

- `no_std` by default — `std` is an opt-in feature (enabled by default)
- All paper dimensions stored in millimeters as `f64`
- Conversions (inches, cm, pt) computed from mm at call site
- `CATALOG` is a static slice of `PaperEntry` — names, aliases, sizes
- `PaperSize::parse()` tries named lookup first, then WxH with unit suffix

## Adding paper sizes

1. Add the `const` to the appropriate section in `PaperSize` impl block
2. Add a `PaperEntry` to `CATALOG` with canonical name and aliases
3. Add a cross-validation test against the authoritative source
4. Cite the standard (ISO, JIS, ANSI, etc.) in a comment

All dimensions must be in millimeters. Use `from_mm()`, not `from_in()`, to avoid float drift.

## Data source

Dimensions are sourced from [typst-library](https://github.com/typst/typst) (Apache-2.0).
When adding sizes not in typst, cite the primary standard document.

## Code style

- `no_std` compatible — no `String`, `Vec`, `format!` outside `#[cfg(feature = "std")]` or `#[cfg(test)]`
- No `unsafe` except the existing `SmallString::as_ref()` (validated invariant)
- No dependencies — this crate must remain zero-dep
- Tests: use `assert_eq!` with a message string on every assertion

## Commits

- One logical concern per commit
- Do not amend published commits or force-push

## Pull requests

- Include tests for new paper sizes
- Run the pre-commit checklist before opening
- Link the standard or data source for any new dimensions

## Releases

Releases are automated via [release-please](https://github.com/googleapis/release-please).
Merge to `main` and the bot handles versioning, changelog, and crates.io publish.
