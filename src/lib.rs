//! Standard paper sizes with parsing and unit conversion.
//!
//! Covers ISO 216 (A/B/C series), DIN D, JIS B, ANSI, ARCH, and North
//! American/international sizes. All dimensions stored in millimeters
//! (source of truth), with conversion to inches, points, and centimeters.
//!
//! Dimension data sourced from [typst-library](https://github.com/typst/typst)
//! (Apache-2.0) and cross-validated against ISO 216, JIS P 0138, and
//! ANSI/ASME Y14.1 standards.
//!
//! # Usage
//!
//! ```
//! use paper_size::PaperSize;
//!
//! let a4 = PaperSize::A4;
//! assert_eq!(a4.width_mm(), 210.0);
//! assert_eq!(a4.height_mm(), 297.0);
//!
//! // Parse from string (case-insensitive)
//! let letter = PaperSize::parse("letter").unwrap();
//! assert!((letter.width_in() - 8.5).abs() < 0.01);
//!
//! // Custom size: "WxH" in millimeters
//! let custom = PaperSize::parse("100x200").unwrap();
//! assert_eq!(custom.width_mm(), 100.0);
//! ```

#![no_std]

#[cfg(feature = "std")]
extern crate std;

// ── Constants ────────────────────────────────────────────────────────────────

const MM_PER_INCH: f64 = 25.4;
const POINTS_PER_INCH: f64 = 72.0;

// ── PaperSize ────────────────────────────────────────────────────────────────

/// A paper size defined by width and height in millimeters (portrait orientation).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PaperSize {
    width: f64,
    height: f64,
}

impl PaperSize {
    /// Create a paper size from width and height in millimeters.
    pub const fn from_mm(width_mm: f64, height_mm: f64) -> Self {
        Self { width: width_mm, height: height_mm }
    }

    /// Create a paper size from width and height in inches.
    pub const fn from_in(width_in: f64, height_in: f64) -> Self {
        Self { width: width_in * MM_PER_INCH, height: height_in * MM_PER_INCH }
    }

    // ── Accessors ────────────────────────────────────────────────────────

    /// Width in millimeters.
    pub const fn width_mm(&self) -> f64 { self.width }
    /// Height in millimeters.
    pub const fn height_mm(&self) -> f64 { self.height }
    /// Width in inches.
    pub const fn width_in(&self) -> f64 { self.width / MM_PER_INCH }
    /// Height in inches.
    pub const fn height_in(&self) -> f64 { self.height / MM_PER_INCH }
    /// Width in centimeters.
    pub const fn width_cm(&self) -> f64 { self.width / 10.0 }
    /// Height in centimeters.
    pub const fn height_cm(&self) -> f64 { self.height / 10.0 }
    /// Width in PostScript points (1/72 inch).
    pub const fn width_pt(&self) -> f64 { self.width / MM_PER_INCH * POINTS_PER_INCH }
    /// Height in PostScript points (1/72 inch).
    pub const fn height_pt(&self) -> f64 { self.height / MM_PER_INCH * POINTS_PER_INCH }

    // ── Orientation ──────────────────────────────────────────────────────

    /// Returns `true` if height > width (portrait).
    pub const fn is_portrait(&self) -> bool { self.height > self.width }
    /// Returns `true` if width > height (landscape).
    pub const fn is_landscape(&self) -> bool { self.width > self.height }

    /// Returns the landscape orientation (swap width and height if portrait).
    pub const fn landscape(&self) -> Self {
        if self.width >= self.height { *self }
        else { Self { width: self.height, height: self.width } }
    }

    /// Returns the portrait orientation (swap width and height if landscape).
    pub const fn portrait(&self) -> Self {
        if self.height >= self.width { *self }
        else { Self { width: self.height, height: self.width } }
    }

    // ══════════════════════════════════════════════════════════════════════
    // All dimensions in mm, sourced from typst-library (Apache-2.0).
    // https://github.com/typst/typst/blob/main/crates/typst-library/src/layout/page.rs
    // ══════════════════════════════════════════════════════════════════════

    // ── ISO A series (ISO 216) ───────────────────────────────────────────

    pub const A0:  Self = Self::from_mm(841.0, 1189.0);
    pub const A1:  Self = Self::from_mm(594.0, 841.0);
    pub const A2:  Self = Self::from_mm(420.0, 594.0);
    pub const A3:  Self = Self::from_mm(297.0, 420.0);
    pub const A4:  Self = Self::from_mm(210.0, 297.0);
    pub const A5:  Self = Self::from_mm(148.0, 210.0);
    pub const A6:  Self = Self::from_mm(105.0, 148.0);
    pub const A7:  Self = Self::from_mm(74.0, 105.0);
    pub const A8:  Self = Self::from_mm(52.0, 74.0);
    pub const A9:  Self = Self::from_mm(37.0, 52.0);
    pub const A10: Self = Self::from_mm(26.0, 37.0);
    pub const A11: Self = Self::from_mm(18.0, 26.0);

    // ── ISO B series (ISO 216) ───────────────────────────────────────────

    pub const ISO_B1:  Self = Self::from_mm(707.0, 1000.0);
    pub const ISO_B2:  Self = Self::from_mm(500.0, 707.0);
    pub const ISO_B3:  Self = Self::from_mm(353.0, 500.0);
    pub const ISO_B4:  Self = Self::from_mm(250.0, 353.0);
    pub const ISO_B5:  Self = Self::from_mm(176.0, 250.0);
    pub const ISO_B6:  Self = Self::from_mm(125.0, 176.0);
    pub const ISO_B7:  Self = Self::from_mm(88.0, 125.0);
    pub const ISO_B8:  Self = Self::from_mm(62.0, 88.0);

    // ── ISO C series (ISO 269, envelopes) ────────────────────────────────

    pub const C3:  Self = Self::from_mm(324.0, 458.0);
    pub const C4:  Self = Self::from_mm(229.0, 324.0);
    pub const C5:  Self = Self::from_mm(162.0, 229.0);
    pub const C6:  Self = Self::from_mm(114.0, 162.0);
    pub const C7:  Self = Self::from_mm(81.0, 114.0);
    pub const C8:  Self = Self::from_mm(57.0, 81.0);

    // ── DIN D series ─────────────────────────────────────────────────────

    pub const DIN_D3: Self = Self::from_mm(272.0, 385.0);
    pub const DIN_D4: Self = Self::from_mm(192.0, 272.0);
    pub const DIN_D5: Self = Self::from_mm(136.0, 192.0);
    pub const DIN_D6: Self = Self::from_mm(96.0, 136.0);
    pub const DIN_D7: Self = Self::from_mm(68.0, 96.0);
    pub const DIN_D8: Self = Self::from_mm(48.0, 68.0);

    // ── SIS (Swedish, academic) ──────────────────────────────────────────

    pub const SIS_G5: Self = Self::from_mm(169.0, 239.0);
    pub const SIS_E5: Self = Self::from_mm(115.0, 220.0);

    // ── JIS B series (Japan, JIS P 0138) ─────────────────────────────────

    pub const JIS_B0:  Self = Self::from_mm(1030.0, 1456.0);
    pub const JIS_B1:  Self = Self::from_mm(728.0, 1030.0);
    pub const JIS_B2:  Self = Self::from_mm(515.0, 728.0);
    pub const JIS_B3:  Self = Self::from_mm(364.0, 515.0);
    pub const JIS_B4:  Self = Self::from_mm(257.0, 364.0);
    pub const JIS_B5:  Self = Self::from_mm(182.0, 257.0);
    pub const JIS_B6:  Self = Self::from_mm(128.0, 182.0);
    pub const JIS_B7:  Self = Self::from_mm(91.0, 128.0);
    pub const JIS_B8:  Self = Self::from_mm(64.0, 91.0);
    pub const JIS_B9:  Self = Self::from_mm(45.0, 64.0);
    pub const JIS_B10: Self = Self::from_mm(32.0, 45.0);
    pub const JIS_B11: Self = Self::from_mm(22.0, 32.0);

    // ── SAC D series (China, GB/T 148) ───────────────────────────────────

    pub const SAC_D0: Self = Self::from_mm(764.0, 1064.0);
    pub const SAC_D1: Self = Self::from_mm(532.0, 760.0);
    pub const SAC_D2: Self = Self::from_mm(380.0, 528.0);
    pub const SAC_D3: Self = Self::from_mm(264.0, 376.0);
    pub const SAC_D4: Self = Self::from_mm(188.0, 260.0);
    pub const SAC_D5: Self = Self::from_mm(130.0, 184.0);
    pub const SAC_D6: Self = Self::from_mm(92.0, 126.0);

    // ── Japan specialty ──────────────────────────────────────────────────

    pub const JP_SHIROKU_BAN_4: Self = Self::from_mm(264.0, 379.0);
    pub const JP_SHIROKU_BAN_5: Self = Self::from_mm(189.0, 262.0);
    pub const JP_SHIROKU_BAN_6: Self = Self::from_mm(127.0, 188.0);
    pub const JP_KIKU_4:        Self = Self::from_mm(227.0, 306.0);
    pub const JP_KIKU_5:        Self = Self::from_mm(151.0, 227.0);
    pub const JP_BUSINESS_CARD: Self = Self::from_mm(91.0, 55.0);

    // ── ANSI (ANSI/ASME Y14.1) ──────────────────────────────────────────

    pub const ANSI_A: Self = Self::from_mm(216.0, 279.0);
    pub const ANSI_B: Self = Self::from_mm(279.0, 432.0);
    pub const ANSI_C: Self = Self::from_mm(432.0, 559.0);
    pub const ANSI_D: Self = Self::from_mm(559.0, 864.0);
    pub const ANSI_E: Self = Self::from_mm(864.0, 1118.0);

    // ── ARCH (Architectural) ─────────────────────────────────────────────

    pub const ARCH_A:  Self = Self::from_mm(229.0, 305.0);
    pub const ARCH_B:  Self = Self::from_mm(305.0, 457.0);
    pub const ARCH_C:  Self = Self::from_mm(457.0, 610.0);
    pub const ARCH_D:  Self = Self::from_mm(610.0, 914.0);
    pub const ARCH_E:  Self = Self::from_mm(914.0, 1219.0);
    pub const ARCH_E1: Self = Self::from_mm(762.0, 1067.0);

    // ── North American (US) ──────────────────────────────────────────────

    pub const US_LETTER:         Self = Self::from_mm(215.9, 279.4);
    pub const US_LEGAL:          Self = Self::from_mm(215.9, 355.6);
    pub const US_TABLOID:        Self = Self::from_mm(279.4, 431.8);
    pub const US_LEDGER:         Self = Self::from_mm(431.8, 279.4);
    pub const US_EXECUTIVE:      Self = Self::from_mm(184.15, 266.7);
    pub const US_STATEMENT:      Self = Self::from_mm(139.7, 215.9);
    pub const US_FOOLSCAP_FOLIO: Self = Self::from_mm(215.9, 342.9);
    pub const US_OFICIO:         Self = Self::from_mm(215.9, 340.36);
    pub const US_GOV_LETTER:     Self = Self::from_mm(203.2, 266.7);
    pub const US_GOV_LEGAL:      Self = Self::from_mm(215.9, 330.2);
    pub const US_DIGEST:         Self = Self::from_mm(139.7, 215.9);
    pub const US_TRADE:          Self = Self::from_mm(152.4, 228.6);
    pub const US_BUSINESS_CARD:  Self = Self::from_mm(88.9, 50.8);

    // ── Convenience aliases ──────────────────────────────────────────────

    pub const LETTER:    Self = Self::US_LETTER;
    pub const LEGAL:     Self = Self::US_LEGAL;
    pub const TABLOID:   Self = Self::US_TABLOID;
    pub const LEDGER:    Self = Self::US_LEDGER;
    pub const EXECUTIVE: Self = Self::US_EXECUTIVE;
    pub const STATEMENT: Self = Self::US_STATEMENT;

    // ── ISO ID cards (ISO 7810) ──────────────────────────────────────────

    pub const ISO_ID_1: Self = Self::from_mm(85.6, 53.98);
    pub const ISO_ID_2: Self = Self::from_mm(74.0, 105.0);
    pub const ISO_ID_3: Self = Self::from_mm(88.0, 125.0);

    // ── International ────────────────────────────────────────────────────

    pub const ASIA_F4:          Self = Self::from_mm(210.0, 330.0);
    pub const CN_BUSINESS_CARD: Self = Self::from_mm(90.0, 54.0);
    pub const EU_BUSINESS_CARD: Self = Self::from_mm(85.0, 55.0);

    // ── UK Imperial ──────────────────────────────────────────────────────

    pub const UK_BRIEF:     Self = Self::from_mm(406.4, 342.9);
    pub const UK_DRAFT:     Self = Self::from_mm(254.0, 406.4);
    pub const UK_FOOLSCAP:  Self = Self::from_mm(203.2, 330.2);
    pub const UK_QUARTO:    Self = Self::from_mm(203.2, 254.0);
    pub const UK_CROWN:     Self = Self::from_mm(508.0, 381.0);
    pub const UK_BOOK_A:    Self = Self::from_mm(111.0, 178.0);
    pub const UK_BOOK_B:    Self = Self::from_mm(129.0, 198.0);

    // ── French traditional (AFNOR) ───────────────────────────────────────

    pub const FR_TELLIERE:           Self = Self::from_mm(340.0, 440.0);
    pub const FR_COURONNE_ECRITURE:  Self = Self::from_mm(360.0, 460.0);
    pub const FR_COURONNE_EDITION:   Self = Self::from_mm(370.0, 470.0);
    pub const FR_RAISIN:             Self = Self::from_mm(500.0, 650.0);
    pub const FR_CARRE:              Self = Self::from_mm(450.0, 560.0);
    pub const FR_JESUS:              Self = Self::from_mm(560.0, 760.0);

    // ── Presentation / newspaper ─────────────────────────────────────────

    pub const PRESENTATION_16_9: Self = Self::from_mm(297.0, 167.0625);
    pub const PRESENTATION_4_3:  Self = Self::from_mm(280.0, 210.0);
    pub const NEWSPAPER_COMPACT:    Self = Self::from_mm(280.0, 430.0);
    pub const NEWSPAPER_BERLINER:   Self = Self::from_mm(315.0, 470.0);
    pub const NEWSPAPER_BROADSHEET: Self = Self::from_mm(381.0, 578.0);
}

// ── Catalog ──────────────────────────────────────────────────────────────────

/// A named paper size entry for lookup.
pub struct PaperEntry {
    /// Canonical name (lowercase, e.g. "a4", "letter").
    pub name: &'static str,
    /// Aliases (e.g. "11x17" for tabloid).
    pub aliases: &'static [&'static str],
    /// The paper size.
    pub size: PaperSize,
}

/// All named paper sizes in the catalog.
pub static CATALOG: &[PaperEntry] = &[
    // ISO A
    PaperEntry { name: "a0",  aliases: &[], size: PaperSize::A0 },
    PaperEntry { name: "a1",  aliases: &[], size: PaperSize::A1 },
    PaperEntry { name: "a2",  aliases: &[], size: PaperSize::A2 },
    PaperEntry { name: "a3",  aliases: &[], size: PaperSize::A3 },
    PaperEntry { name: "a4",  aliases: &[], size: PaperSize::A4 },
    PaperEntry { name: "a5",  aliases: &[], size: PaperSize::A5 },
    PaperEntry { name: "a6",  aliases: &[], size: PaperSize::A6 },
    PaperEntry { name: "a7",  aliases: &[], size: PaperSize::A7 },
    PaperEntry { name: "a8",  aliases: &[], size: PaperSize::A8 },
    PaperEntry { name: "a9",  aliases: &[], size: PaperSize::A9 },
    PaperEntry { name: "a10", aliases: &[], size: PaperSize::A10 },
    PaperEntry { name: "a11", aliases: &[], size: PaperSize::A11 },
    // ISO B
    PaperEntry { name: "iso-b1", aliases: &[], size: PaperSize::ISO_B1 },
    PaperEntry { name: "iso-b2", aliases: &[], size: PaperSize::ISO_B2 },
    PaperEntry { name: "iso-b3", aliases: &[], size: PaperSize::ISO_B3 },
    PaperEntry { name: "iso-b4", aliases: &[], size: PaperSize::ISO_B4 },
    PaperEntry { name: "iso-b5", aliases: &[], size: PaperSize::ISO_B5 },
    PaperEntry { name: "iso-b6", aliases: &[], size: PaperSize::ISO_B6 },
    PaperEntry { name: "iso-b7", aliases: &[], size: PaperSize::ISO_B7 },
    PaperEntry { name: "iso-b8", aliases: &[], size: PaperSize::ISO_B8 },
    // ISO C
    PaperEntry { name: "iso-c3", aliases: &["c3"], size: PaperSize::C3 },
    PaperEntry { name: "iso-c4", aliases: &["c4"], size: PaperSize::C4 },
    PaperEntry { name: "iso-c5", aliases: &["c5"], size: PaperSize::C5 },
    PaperEntry { name: "iso-c6", aliases: &["c6"], size: PaperSize::C6 },
    PaperEntry { name: "iso-c7", aliases: &["c7"], size: PaperSize::C7 },
    PaperEntry { name: "iso-c8", aliases: &["c8"], size: PaperSize::C8 },
    // DIN D
    PaperEntry { name: "din-d3", aliases: &[], size: PaperSize::DIN_D3 },
    PaperEntry { name: "din-d4", aliases: &[], size: PaperSize::DIN_D4 },
    PaperEntry { name: "din-d5", aliases: &[], size: PaperSize::DIN_D5 },
    PaperEntry { name: "din-d6", aliases: &[], size: PaperSize::DIN_D6 },
    PaperEntry { name: "din-d7", aliases: &[], size: PaperSize::DIN_D7 },
    PaperEntry { name: "din-d8", aliases: &[], size: PaperSize::DIN_D8 },
    // SIS
    PaperEntry { name: "sis-g5", aliases: &[], size: PaperSize::SIS_G5 },
    PaperEntry { name: "sis-e5", aliases: &[], size: PaperSize::SIS_E5 },
    // JIS B
    PaperEntry { name: "jis-b0",  aliases: &[], size: PaperSize::JIS_B0 },
    PaperEntry { name: "jis-b1",  aliases: &[], size: PaperSize::JIS_B1 },
    PaperEntry { name: "jis-b2",  aliases: &[], size: PaperSize::JIS_B2 },
    PaperEntry { name: "jis-b3",  aliases: &[], size: PaperSize::JIS_B3 },
    PaperEntry { name: "jis-b4",  aliases: &["b4"], size: PaperSize::JIS_B4 },
    PaperEntry { name: "jis-b5",  aliases: &["b5"], size: PaperSize::JIS_B5 },
    PaperEntry { name: "jis-b6",  aliases: &[], size: PaperSize::JIS_B6 },
    PaperEntry { name: "jis-b7",  aliases: &[], size: PaperSize::JIS_B7 },
    PaperEntry { name: "jis-b8",  aliases: &[], size: PaperSize::JIS_B8 },
    PaperEntry { name: "jis-b9",  aliases: &[], size: PaperSize::JIS_B9 },
    PaperEntry { name: "jis-b10", aliases: &[], size: PaperSize::JIS_B10 },
    PaperEntry { name: "jis-b11", aliases: &[], size: PaperSize::JIS_B11 },
    // SAC D
    PaperEntry { name: "sac-d0", aliases: &[], size: PaperSize::SAC_D0 },
    PaperEntry { name: "sac-d1", aliases: &[], size: PaperSize::SAC_D1 },
    PaperEntry { name: "sac-d2", aliases: &[], size: PaperSize::SAC_D2 },
    PaperEntry { name: "sac-d3", aliases: &[], size: PaperSize::SAC_D3 },
    PaperEntry { name: "sac-d4", aliases: &[], size: PaperSize::SAC_D4 },
    PaperEntry { name: "sac-d5", aliases: &[], size: PaperSize::SAC_D5 },
    PaperEntry { name: "sac-d6", aliases: &[], size: PaperSize::SAC_D6 },
    // Japan specialty
    PaperEntry { name: "jp-shiroku-ban-4", aliases: &[], size: PaperSize::JP_SHIROKU_BAN_4 },
    PaperEntry { name: "jp-shiroku-ban-5", aliases: &[], size: PaperSize::JP_SHIROKU_BAN_5 },
    PaperEntry { name: "jp-shiroku-ban-6", aliases: &[], size: PaperSize::JP_SHIROKU_BAN_6 },
    PaperEntry { name: "jp-kiku-4",        aliases: &[], size: PaperSize::JP_KIKU_4 },
    PaperEntry { name: "jp-kiku-5",        aliases: &[], size: PaperSize::JP_KIKU_5 },
    PaperEntry { name: "jp-business-card", aliases: &[], size: PaperSize::JP_BUSINESS_CARD },
    // ANSI
    PaperEntry { name: "ansi-a", aliases: &[], size: PaperSize::ANSI_A },
    PaperEntry { name: "ansi-b", aliases: &[], size: PaperSize::ANSI_B },
    PaperEntry { name: "ansi-c", aliases: &[], size: PaperSize::ANSI_C },
    PaperEntry { name: "ansi-d", aliases: &[], size: PaperSize::ANSI_D },
    PaperEntry { name: "ansi-e", aliases: &[], size: PaperSize::ANSI_E },
    // ARCH
    PaperEntry { name: "arch-a",  aliases: &[], size: PaperSize::ARCH_A },
    PaperEntry { name: "arch-b",  aliases: &[], size: PaperSize::ARCH_B },
    PaperEntry { name: "arch-c",  aliases: &[], size: PaperSize::ARCH_C },
    PaperEntry { name: "arch-d",  aliases: &[], size: PaperSize::ARCH_D },
    PaperEntry { name: "arch-e",  aliases: &[], size: PaperSize::ARCH_E },
    PaperEntry { name: "arch-e1", aliases: &[], size: PaperSize::ARCH_E1 },
    // US
    PaperEntry { name: "us-letter",    aliases: &["letter", "usletter"], size: PaperSize::US_LETTER },
    PaperEntry { name: "us-legal",     aliases: &["legal", "uslegal"],   size: PaperSize::US_LEGAL },
    PaperEntry { name: "us-tabloid",   aliases: &["tabloid"],            size: PaperSize::US_TABLOID },
    PaperEntry { name: "us-ledger",    aliases: &["ledger"],             size: PaperSize::US_LEDGER },
    PaperEntry { name: "us-executive", aliases: &["executive"],          size: PaperSize::US_EXECUTIVE },
    PaperEntry { name: "us-statement", aliases: &["statement", "half-letter"], size: PaperSize::US_STATEMENT },
    PaperEntry { name: "us-foolscap-folio", aliases: &["folio"],         size: PaperSize::US_FOOLSCAP_FOLIO },
    PaperEntry { name: "us-oficio",    aliases: &["oficio"],             size: PaperSize::US_OFICIO },
    PaperEntry { name: "us-gov-letter", aliases: &[],                    size: PaperSize::US_GOV_LETTER },
    PaperEntry { name: "us-gov-legal",  aliases: &[],                    size: PaperSize::US_GOV_LEGAL },
    PaperEntry { name: "us-digest",     aliases: &["digest"],            size: PaperSize::US_DIGEST },
    PaperEntry { name: "us-trade",      aliases: &["trade"],             size: PaperSize::US_TRADE },
    PaperEntry { name: "us-business-card", aliases: &[],                 size: PaperSize::US_BUSINESS_CARD },
    // ISO ID
    PaperEntry { name: "iso-id-1", aliases: &["id-1", "credit-card"], size: PaperSize::ISO_ID_1 },
    PaperEntry { name: "iso-id-2", aliases: &["id-2"],                size: PaperSize::ISO_ID_2 },
    PaperEntry { name: "iso-id-3", aliases: &["id-3"],                size: PaperSize::ISO_ID_3 },
    // International
    PaperEntry { name: "asia-f4",          aliases: &["f4"],     size: PaperSize::ASIA_F4 },
    PaperEntry { name: "cn-business-card", aliases: &[],         size: PaperSize::CN_BUSINESS_CARD },
    PaperEntry { name: "eu-business-card", aliases: &[],         size: PaperSize::EU_BUSINESS_CARD },
    // UK
    PaperEntry { name: "uk-brief",    aliases: &[], size: PaperSize::UK_BRIEF },
    PaperEntry { name: "uk-draft",    aliases: &[], size: PaperSize::UK_DRAFT },
    PaperEntry { name: "uk-foolscap", aliases: &[], size: PaperSize::UK_FOOLSCAP },
    PaperEntry { name: "uk-quarto",   aliases: &[], size: PaperSize::UK_QUARTO },
    PaperEntry { name: "uk-crown",    aliases: &[], size: PaperSize::UK_CROWN },
    PaperEntry { name: "uk-book-a",   aliases: &[], size: PaperSize::UK_BOOK_A },
    PaperEntry { name: "uk-book-b",   aliases: &[], size: PaperSize::UK_BOOK_B },
    // France
    PaperEntry { name: "fr-telliere",          aliases: &[], size: PaperSize::FR_TELLIERE },
    PaperEntry { name: "fr-couronne-ecriture", aliases: &[], size: PaperSize::FR_COURONNE_ECRITURE },
    PaperEntry { name: "fr-couronne-edition",  aliases: &[], size: PaperSize::FR_COURONNE_EDITION },
    PaperEntry { name: "fr-raisin",            aliases: &[], size: PaperSize::FR_RAISIN },
    PaperEntry { name: "fr-carre",             aliases: &[], size: PaperSize::FR_CARRE },
    PaperEntry { name: "fr-jesus",             aliases: &[], size: PaperSize::FR_JESUS },
    // Presentation / newspaper
    PaperEntry { name: "presentation-16-9", aliases: &["16:9"],     size: PaperSize::PRESENTATION_16_9 },
    PaperEntry { name: "presentation-4-3",  aliases: &["4:3"],      size: PaperSize::PRESENTATION_4_3 },
    PaperEntry { name: "newspaper-compact",    aliases: &[],        size: PaperSize::NEWSPAPER_COMPACT },
    PaperEntry { name: "newspaper-berliner",   aliases: &[],        size: PaperSize::NEWSPAPER_BERLINER },
    PaperEntry { name: "newspaper-broadsheet", aliases: &[],        size: PaperSize::NEWSPAPER_BROADSHEET },
];

/// Look up a paper size by name (case-insensitive).
///
/// Searches canonical names and aliases.
pub fn lookup(name: &str) -> Option<PaperSize> {
    let lower = to_ascii_lower(name);
    let s = lower.as_ref();
    for entry in CATALOG {
        if entry.name == s {
            return Some(entry.size);
        }
        for alias in entry.aliases {
            if *alias == s {
                return Some(entry.size);
            }
        }
    }
    None
}

/// All canonical paper size names in the catalog.
pub fn names() -> impl Iterator<Item = &'static str> {
    CATALOG.iter().map(|e| e.name)
}

// ── Parsing ──────────────────────────────────────────────────────────────────

/// Error returned when parsing a paper size string fails.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParseError {
    input: SmallString,
    kind: ParseErrorKind,
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum ParseErrorKind {
    Unknown,
    InvalidDimension,
    NonPositive,
}

impl core::fmt::Display for ParseError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self.kind {
            ParseErrorKind::Unknown => write!(
                f,
                "unknown paper size: '{}'. Use a named size (a4, letter, ...) or WxH in mm",
                self.input.as_ref()
            ),
            ParseErrorKind::InvalidDimension => write!(
                f,
                "invalid dimension in '{}': expected a number",
                self.input.as_ref()
            ),
            ParseErrorKind::NonPositive => write!(
                f,
                "paper dimensions must be positive: '{}'",
                self.input.as_ref()
            ),
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for ParseError {}

impl PaperSize {
    /// Parse a paper size from a string.
    ///
    /// Accepts:
    /// - Named sizes (case-insensitive): `"a4"`, `"Letter"`, `"tabloid"`, etc.
    /// - Custom `WxH` in millimeters: `"210x297"`, `"100.5x200"`
    /// - Custom `WxH` with unit suffix: `"8.5x11in"`, `"210x297mm"`
    pub fn parse(s: &str) -> Result<Self, ParseError> {
        let trimmed = s.trim();

        // Try named lookup first
        if let Some(size) = lookup(trimmed) {
            return Ok(size);
        }

        // Try WxH parsing
        let (dims, unit) = if let Some(stripped) = trimmed.strip_suffix("mm") {
            (stripped, Unit::Mm)
        } else if let Some(stripped) = trimmed.strip_suffix("in") {
            (stripped, Unit::In)
        } else if let Some(stripped) = trimmed.strip_suffix("cm") {
            (stripped, Unit::Cm)
        } else if let Some(stripped) = trimmed.strip_suffix("pt") {
            (stripped, Unit::Pt)
        } else {
            (trimmed, Unit::Mm)
        };

        let sep = dims.find('x').or_else(|| dims.find('X'));
        let sep = match sep {
            Some(i) => i,
            None => return Err(ParseError {
                input: SmallString::from_str(s),
                kind: ParseErrorKind::Unknown,
            }),
        };

        let w_str = dims[..sep].trim();
        let h_str = dims[sep + 1..].trim();

        let w: f64 = parse_f64(w_str).ok_or_else(|| ParseError {
            input: SmallString::from_str(s),
            kind: ParseErrorKind::InvalidDimension,
        })?;
        let h: f64 = parse_f64(h_str).ok_or_else(|| ParseError {
            input: SmallString::from_str(s),
            kind: ParseErrorKind::InvalidDimension,
        })?;

        if w <= 0.0 || h <= 0.0 {
            return Err(ParseError {
                input: SmallString::from_str(s),
                kind: ParseErrorKind::NonPositive,
            });
        }

        Ok(match unit {
            Unit::Mm => Self::from_mm(w, h),
            Unit::In => Self::from_in(w, h),
            Unit::Cm => Self::from_mm(w * 10.0, h * 10.0),
            Unit::Pt => Self::from_mm(w * MM_PER_INCH / POINTS_PER_INCH, h * MM_PER_INCH / POINTS_PER_INCH),
        })
    }
}

#[derive(Debug, Clone, Copy)]
enum Unit { Mm, In, Cm, Pt }

// ── no_std helpers ───────────────────────────────────────────────────────────

/// Tiny inline string to avoid alloc in error paths.
#[derive(Debug, Clone, PartialEq, Eq)]
struct SmallString {
    buf: [u8; 64],
    len: usize,
}

impl SmallString {
    fn from_str(s: &str) -> Self {
        let mut buf = [0u8; 64];
        let len = s.len().min(64);
        buf[..len].copy_from_slice(&s.as_bytes()[..len]);
        Self { buf, len }
    }

    fn as_ref(&self) -> &str {
        // Safety: we only copy valid UTF-8 bytes in from_str
        unsafe { core::str::from_utf8_unchecked(&self.buf[..self.len]) }
    }
}

/// ASCII-lowercase a string without alloc. Returns a stack buffer.
fn to_ascii_lower(s: &str) -> SmallString {
    let mut buf = [0u8; 64];
    let len = s.len().min(64);
    for (i, b) in s.as_bytes()[..len].iter().enumerate() {
        buf[i] = b.to_ascii_lowercase();
    }
    SmallString { buf, len }
}

/// Parse f64 without std.
/// With std feature, delegates to str::parse.
fn parse_f64(s: &str) -> Option<f64> {
    #[cfg(feature = "std")]
    { s.parse::<f64>().ok() }
    #[cfg(not(feature = "std"))]
    {
        let s = s.trim();
        if s.is_empty() { return None; }

        let (neg, s) = if let Some(rest) = s.strip_prefix('-') {
            (true, rest)
        } else {
            (false, s)
        };

        let dot = s.find('.');
        let (int_part, frac_part) = match dot {
            Some(i) => (&s[..i], Some(&s[i + 1..])),
            None => (s, None),
        };

        let mut result: f64 = 0.0;
        for b in int_part.as_bytes() {
            if !b.is_ascii_digit() { return None; }
            result = result * 10.0 + (*b - b'0') as f64;
        }

        if let Some(frac) = frac_part {
            let mut factor = 0.1;
            for b in frac.as_bytes() {
                if !b.is_ascii_digit() { return None; }
                result += (*b - b'0') as f64 * factor;
                factor *= 0.1;
            }
        }

        Some(if neg { -result } else { result })
    }
}

// ── Display ──────────────────────────────────────────────────────────────────

impl core::fmt::Display for PaperSize {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        for entry in CATALOG {
            if (entry.size.width - self.width).abs() < 0.5
                && (entry.size.height - self.height).abs() < 0.5
            {
                return write!(f, "{}", entry.name);
            }
        }
        write!(f, "{:.0}x{:.0}mm", self.width, self.height)
    }
}

// ── Tests ────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    extern crate std;
    use std::format;
    use super::*;

    #[test]
    fn a4_dimensions() {
        assert_eq!(PaperSize::A4.width_mm(), 210.0);
        assert_eq!(PaperSize::A4.height_mm(), 297.0);
        assert!(PaperSize::A4.is_portrait());
    }

    #[test]
    fn letter_dimensions_in_inches() {
        assert!((PaperSize::LETTER.width_in() - 8.5).abs() < 0.01);
        assert!((PaperSize::LETTER.height_in() - 11.0).abs() < 0.01);
    }

    #[test]
    fn parse_named_case_insensitive() {
        assert_eq!(PaperSize::parse("a4").unwrap().width_mm(), 210.0);
        assert_eq!(PaperSize::parse("A4").unwrap().width_mm(), 210.0);
        assert_eq!(PaperSize::parse("Letter").unwrap().width_in(), PaperSize::LETTER.width_in());
        assert_eq!(PaperSize::parse("LETTER").unwrap().width_in(), PaperSize::LETTER.width_in());
    }

    #[test]
    fn parse_aliases() {
        assert_eq!(PaperSize::parse("half-letter").unwrap().width_mm(), PaperSize::STATEMENT.width_mm());
        assert_eq!(PaperSize::parse("b4").unwrap().width_mm(), PaperSize::JIS_B4.width_mm());
        assert_eq!(PaperSize::parse("folio").unwrap().width_mm(), PaperSize::US_FOOLSCAP_FOLIO.width_mm());
        assert_eq!(PaperSize::parse("ledger").unwrap().width_mm(), PaperSize::US_LEDGER.width_mm());
    }

    #[test]
    fn parse_custom_mm() {
        let ps = PaperSize::parse("100x200").unwrap();
        assert_eq!(ps.width_mm(), 100.0);
        assert_eq!(ps.height_mm(), 200.0);
    }

    #[test]
    fn parse_custom_inches() {
        let ps = PaperSize::parse("8.5x11in").unwrap();
        assert!((ps.width_in() - 8.5).abs() < 0.01);
        assert!((ps.height_in() - 11.0).abs() < 0.01);
    }

    #[test]
    fn parse_custom_cm() {
        let ps = PaperSize::parse("21x29.7cm").unwrap();
        assert!((ps.width_mm() - 210.0).abs() < 0.1);
        assert!((ps.height_mm() - 297.0).abs() < 0.1);
    }

    #[test]
    fn parse_rejects_invalid() {
        assert!(PaperSize::parse("banana").is_err());
        assert!(PaperSize::parse("0x0").is_err());
        assert!(PaperSize::parse("-1x10").is_err());
        assert!(PaperSize::parse("abcxdef").is_err());
    }

    #[test]
    fn landscape_portrait_swap() {
        let land = PaperSize::A4.landscape();
        assert!(land.is_landscape());
        assert_eq!(land.width_mm(), 297.0);
        assert_eq!(land.height_mm(), 210.0);

        let port = land.portrait();
        assert!(port.is_portrait());
        assert_eq!(port.width_mm(), 210.0);
    }

    #[test]
    fn all_catalog_entries_parse() {
        for entry in CATALOG {
            assert!(
                PaperSize::parse(entry.name).is_ok(),
                "Failed to parse: {}",
                entry.name
            );
        }
    }

    #[test]
    fn display_named() {
        assert_eq!(format!("{}", PaperSize::A4), "a4");
    }

    #[test]
    fn display_custom() {
        let custom = PaperSize::from_mm(100.0, 200.0);
        assert_eq!(format!("{}", custom), "100x200mm");
    }

    #[test]
    fn unit_conversions() {
        let a4 = PaperSize::A4;
        assert!((a4.width_cm() - 21.0).abs() < 0.01);
        assert!((a4.width_pt() - 595.28).abs() < 0.5);
    }

    #[test]
    fn iso_a_series_halving_property() {
        // ISO 216: each A(n+1) width = A(n) height / 2 (rounded to mm)
        let sizes = [
            PaperSize::A0, PaperSize::A1, PaperSize::A2, PaperSize::A3,
            PaperSize::A4, PaperSize::A5, PaperSize::A6, PaperSize::A7,
        ];
        for pair in sizes.windows(2) {
            assert!(
                (pair[1].width_mm() - pair[0].height_mm() / 2.0).abs() < 1.5,
                "A-series halving violated: {} vs {}",
                pair[1].width_mm(), pair[0].height_mm() / 2.0
            );
        }
    }

    // ── Cross-validation against typst-library values ────────────────────
    // Source: https://github.com/typst/typst (Apache-2.0)
    // These tests verify our constants match typst's authoritative data.

    #[test]
    fn cross_validate_iso_a_vs_typst() {
        let typst: &[(&str, f64, f64)] = &[
            ("a0", 841.0, 1189.0), ("a1", 594.0, 841.0), ("a2", 420.0, 594.0),
            ("a3", 297.0, 420.0),  ("a4", 210.0, 297.0), ("a5", 148.0, 210.0),
            ("a6", 105.0, 148.0),  ("a7", 74.0, 105.0),  ("a8", 52.0, 74.0),
            ("a9", 37.0, 52.0),    ("a10", 26.0, 37.0),  ("a11", 18.0, 26.0),
        ];
        for &(name, w, h) in typst {
            let ps = PaperSize::parse(name).unwrap();
            assert_eq!(ps.width_mm(), w, "{name} width mismatch");
            assert_eq!(ps.height_mm(), h, "{name} height mismatch");
        }
    }

    #[test]
    fn cross_validate_iso_b_vs_typst() {
        let typst: &[(&str, f64, f64)] = &[
            ("iso-b1", 707.0, 1000.0), ("iso-b2", 500.0, 707.0),
            ("iso-b3", 353.0, 500.0),  ("iso-b4", 250.0, 353.0),
            ("iso-b5", 176.0, 250.0),  ("iso-b6", 125.0, 176.0),
            ("iso-b7", 88.0, 125.0),   ("iso-b8", 62.0, 88.0),
        ];
        for &(name, w, h) in typst {
            let ps = PaperSize::parse(name).unwrap();
            assert_eq!(ps.width_mm(), w, "{name} width mismatch");
            assert_eq!(ps.height_mm(), h, "{name} height mismatch");
        }
    }

    #[test]
    fn cross_validate_jis_b_vs_typst() {
        let typst: &[(&str, f64, f64)] = &[
            ("jis-b0", 1030.0, 1456.0), ("jis-b1", 728.0, 1030.0),
            ("jis-b2", 515.0, 728.0),   ("jis-b3", 364.0, 515.0),
            ("jis-b4", 257.0, 364.0),    ("jis-b5", 182.0, 257.0),
            ("jis-b6", 128.0, 182.0),    ("jis-b7", 91.0, 128.0),
            ("jis-b8", 64.0, 91.0),      ("jis-b9", 45.0, 64.0),
            ("jis-b10", 32.0, 45.0),     ("jis-b11", 22.0, 32.0),
        ];
        for &(name, w, h) in typst {
            let ps = PaperSize::parse(name).unwrap();
            assert_eq!(ps.width_mm(), w, "{name} width mismatch");
            assert_eq!(ps.height_mm(), h, "{name} height mismatch");
        }
    }

    #[test]
    fn cross_validate_us_vs_typst() {
        let typst: &[(&str, f64, f64)] = &[
            ("us-letter", 215.9, 279.4), ("us-legal", 215.9, 355.6),
            ("us-tabloid", 279.4, 431.8), ("us-executive", 184.15, 266.7),
            ("us-statement", 139.7, 215.9),
        ];
        for &(name, w, h) in typst {
            let ps = PaperSize::parse(name).unwrap();
            assert!((ps.width_mm() - w).abs() < 0.01, "{name} width mismatch: {} vs {w}", ps.width_mm());
            assert!((ps.height_mm() - h).abs() < 0.01, "{name} height mismatch: {} vs {h}", ps.height_mm());
        }
    }

    #[test]
    fn cross_validate_ansi_vs_typst() {
        let typst: &[(&str, f64, f64)] = &[
            ("ansi-a", 216.0, 279.0), ("ansi-b", 279.0, 432.0),
            ("ansi-c", 432.0, 559.0), ("ansi-d", 559.0, 864.0),
            ("ansi-e", 864.0, 1118.0),
        ];
        for &(name, w, h) in typst {
            let ps = PaperSize::parse(name).unwrap();
            assert_eq!(ps.width_mm(), w, "{name} width mismatch");
            assert_eq!(ps.height_mm(), h, "{name} height mismatch");
        }
    }

    #[test]
    fn cross_validate_arch_vs_typst() {
        let typst: &[(&str, f64, f64)] = &[
            ("arch-a", 229.0, 305.0),  ("arch-b", 305.0, 457.0),
            ("arch-c", 457.0, 610.0),  ("arch-d", 610.0, 914.0),
            ("arch-e", 914.0, 1219.0), ("arch-e1", 762.0, 1067.0),
        ];
        for &(name, w, h) in typst {
            let ps = PaperSize::parse(name).unwrap();
            assert_eq!(ps.width_mm(), w, "{name} width mismatch");
            assert_eq!(ps.height_mm(), h, "{name} height mismatch");
        }
    }
}
