# paper-size

Standard paper sizes with parsing and unit conversion.

Zero dependencies, `no_std` compatible, builds to WASM.

## Install

```toml
[dependencies]
paper-size = "0.1"
```

For `no_std` / WASM targets:

```toml
[dependencies]
paper-size = { version = "0.1", default-features = false }
```

## Usage

```rust
use paper_size::PaperSize;

// Named constants
let a4 = PaperSize::A4;
assert_eq!(a4.width_mm(), 210.0);
assert_eq!(a4.height_mm(), 297.0);

// Parse from string (case-insensitive)
let letter = PaperSize::parse("letter").unwrap();
let legal = PaperSize::parse("Legal").unwrap();

// Custom dimensions with unit suffix
let custom = PaperSize::parse("8.5x11in").unwrap();
let metric = PaperSize::parse("210x297mm").unwrap();
let points = PaperSize::parse("595x842pt").unwrap();

// Unit conversion
let a4 = PaperSize::A4;
a4.width_in();   // inches
a4.width_cm();   // centimeters
a4.width_pt();   // PostScript points (1/72 inch)

// Orientation
let landscape = PaperSize::A4.landscape();
let portrait = landscape.portrait();
```

## Coverage

100+ paper sizes across 15 standards:

| Series | Sizes |
|---|---|
| ISO A (ISO 216) | A0–A11 |
| ISO B (ISO 216) | B1–B8 |
| ISO C (ISO 269) | C3–C8 |
| DIN D | D3–D8 |
| SIS (Swedish) | G5, E5 |
| JIS B (JIS P 0138) | B0–B11 |
| SAC D (GB/T 148) | D0–D6 |
| ANSI (Y14.1) | A–E |
| ARCH | A–E, E1 |
| US | Letter, Legal, Tabloid, Ledger, Executive, Statement, Folio, Oficio, Gov Letter/Legal, Digest, Trade |
| UK Imperial | Brief, Draft, Foolscap, Quarto, Crown, Book A/B |
| French (AFNOR) | Telliere, Couronne, Raisin, Carre, Jesus |
| ISO ID (ISO 7810) | ID-1 (credit card), ID-2, ID-3 |
| Asia / Business | F4, JP/CN/EU business cards, JP Shiroku-ban, JP Kiku |
| Presentation | 16:9, 4:3 |

## Parsing

`PaperSize::parse()` accepts:

- **Named sizes** (case-insensitive): `"a4"`, `"Letter"`, `"jis-b5"`, `"tabloid"`
- **Aliases**: `"b4"` → JIS B4, `"half-letter"` → Statement, `"folio"` → US Foolscap Folio
- **Custom WxH** with optional unit: `"210x297"` (mm default), `"8.5x11in"`, `"21x29.7cm"`, `"595x842pt"`

## Catalog

Browse all sizes programmatically:

```rust
use paper_size::{CATALOG, lookup, names};

// Iterate all entries
for entry in CATALOG {
    println!("{}: {}×{}mm", entry.name, entry.size.width_mm(), entry.size.height_mm());
}

// Direct lookup
let size = lookup("a4").unwrap();

// List all names
let all: Vec<&str> = names().collect();
```

## Data source

Dimensions sourced from [typst-library](https://github.com/typst/typst) (Apache-2.0) and cross-validated against ISO 216, JIS P 0138, and ANSI/ASME Y14.1 standards. Six test suites verify every value matches the upstream source.

## License

MIT
