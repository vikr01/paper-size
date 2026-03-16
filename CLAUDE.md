# paper-size — Agent Reference

Read [CONTRIBUTING.md](CONTRIBUTING.md) for setup, code style, and commit conventions.
Read [README.md](README.md) for usage examples and coverage tables.

## Project

- Standard paper sizes: ISO 216, ISO 269, DIN D, JIS P 0138, SAC GB/T 148, ANSI Y14.1, ARCH, US/UK/FR
- Single-file library crate: `src/lib.rs`
- Zero dependencies; `no_std` default; WASM target: `wasm32-unknown-unknown`
- Data sourced from typst-library (Apache-2.0), cross-validated in tests

## Invariants

```
Let S = set of all PaperSize constants, C = CATALOG entries, D(s) = (width_mm, height_mm)

∀ s ∈ S:
  D(s) stored in mm — ∄ from_in() in const definitions (float drift)

∀ c ∈ C:
  PaperSize::parse(c.name) = Ok(c.size)
  ∀ a ∈ c.aliases: PaperSize::parse(a) = Ok(c.size)

∀ c ∈ C, c.name ≠ "ledger":
  c.size.height ≥ c.size.width      // portrait orientation

∀ n ∈ [0, 7]:
  |A(n+1).width - A(n).height / 2| < 1.5    // ISO 216 halving property
```

## Structure

```
entity              location             purpose
─────────────────── ──────────────────── ──────────────────────────────────
PaperSize           src/lib.rs           struct: width, height in mm
PaperSize::*        src/lib.rs           named constants (A0..A11, LETTER, etc.)
CATALOG             src/lib.rs           static &[PaperEntry] — name + aliases + size
lookup(name)        src/lib.rs           case-insensitive name → Option<PaperSize>
PaperSize::parse(s) src/lib.rs           named ∪ WxH{mm,in,cm,pt} → Result
```

## Adding a paper size

```
ADD_PAPER(name, width_mm, height_mm, standard_ref):
  assert width_mm > 0 ∧ height_mm > 0
  add const to PaperSize impl        // Self::from_mm(width_mm, height_mm)
  add PaperEntry to CATALOG           // name, aliases, size
  add cross_validate test             // expected values from standard_ref
  ∄ from_in() — convert to mm first
```

## Prohibitions

- ∄ dependencies — crate must remain zero-dep
- ∄ `from_in()` in const definitions — store mm, compute inches at call site
- ∄ `unsafe` additions — existing `SmallString::as_ref()` is the only exception
- ∄ `std` usage outside `#[cfg(feature = "std")]` or `#[cfg(test)]`
- ∄ `alloc` — error paths use stack-allocated `SmallString`
- ∄ push without user approval
- ∄ amend published commits; ∄ force-push

## Testing

```
VALIDATE(crate):
  cargo test                           // std feature (default)
  cargo test --no-default-features     // no_std
  cargo build --target wasm32-unknown-unknown --no-default-features   // WASM
  cargo clippy -- -D warnings
  cargo doc --no-deps
```

- Cross-validation tests compare every constant against typst-library values
- ∀ `assert_*` → include message string as last argument
