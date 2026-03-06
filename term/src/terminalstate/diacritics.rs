/// Maps a combining diacritical mark to its 1-based row/column number (1-297).
/// Returns 0 for unrecognized characters, meaning "not specified".
/// This matches kitty's convention where 0 is reserved as a sentinel for
/// "not specified", and U+0305 (the first diacritic) maps to 1.
/// Derived from Kitty's rowcolumn-diacritics.c / rowcolumn-diacritics.txt.
pub fn diacritic_to_num(c: char) -> u32 {
    let zero_based = match c as u32 {
        0x0305 => Some(0),
        0x030D..=0x030E => Some(1 + (c as u32 - 0x030D)),
        0x0310 => Some(3),
        0x0312 => Some(4),
        0x033D..=0x033F => Some(5 + (c as u32 - 0x033D)),
        0x0346 => Some(8),
        0x034A..=0x034C => Some(9 + (c as u32 - 0x034A)),
        0x0350..=0x0352 => Some(12 + (c as u32 - 0x0350)),
        0x0357 => Some(15),
        0x035B => Some(16),
        0x0363..=0x036F => Some(17 + (c as u32 - 0x0363)),
        0x0483..=0x0487 => Some(30 + (c as u32 - 0x0483)),
        0x0592..=0x0595 => Some(35 + (c as u32 - 0x0592)),
        0x0597..=0x0599 => Some(39 + (c as u32 - 0x0597)),
        0x059C..=0x05A1 => Some(42 + (c as u32 - 0x059C)),
        0x05A8..=0x05A9 => Some(48 + (c as u32 - 0x05A8)),
        0x05AB..=0x05AC => Some(50 + (c as u32 - 0x05AB)),
        0x05AF => Some(52),
        0x05C4 => Some(53),
        0x0610..=0x0617 => Some(54 + (c as u32 - 0x0610)),
        0x0657..=0x065B => Some(62 + (c as u32 - 0x0657)),
        0x065D..=0x065E => Some(67 + (c as u32 - 0x065D)),
        0x06D6..=0x06DC => Some(69 + (c as u32 - 0x06D6)),
        0x06DF..=0x06E2 => Some(76 + (c as u32 - 0x06DF)),
        0x06E4 => Some(80),
        0x06E7..=0x06E8 => Some(81 + (c as u32 - 0x06E7)),
        0x06EB..=0x06EC => Some(83 + (c as u32 - 0x06EB)),
        0x0730 => Some(85),
        0x0732..=0x0733 => Some(86 + (c as u32 - 0x0732)),
        0x0735..=0x0736 => Some(88 + (c as u32 - 0x0735)),
        0x073A => Some(90),
        0x073D => Some(91),
        0x073F..=0x0741 => Some(92 + (c as u32 - 0x073F)),
        0x0743 => Some(95),
        0x0745 => Some(96),
        0x0747 => Some(97),
        0x0749..=0x074A => Some(98 + (c as u32 - 0x0749)),
        0x07EB..=0x07F1 => Some(100 + (c as u32 - 0x07EB)),
        0x07F3 => Some(107),
        0x0816..=0x0819 => Some(108 + (c as u32 - 0x0816)),
        0x081B..=0x0823 => Some(112 + (c as u32 - 0x081B)),
        0x0825..=0x0827 => Some(121 + (c as u32 - 0x0825)),
        0x0829..=0x082D => Some(124 + (c as u32 - 0x0829)),
        0x0951 => Some(129),
        0x0953..=0x0954 => Some(130 + (c as u32 - 0x0953)),
        0x0F82..=0x0F83 => Some(132 + (c as u32 - 0x0F82)),
        0x0F86..=0x0F87 => Some(134 + (c as u32 - 0x0F86)),
        0x135D..=0x135F => Some(136 + (c as u32 - 0x135D)),
        0x17DD => Some(139),
        0x193A => Some(140),
        0x1A17 => Some(141),
        0x1A75..=0x1A7C => Some(142 + (c as u32 - 0x1A75)),
        0x1B6B => Some(150),
        0x1B6D..=0x1B73 => Some(151 + (c as u32 - 0x1B6D)),
        0x1CD0..=0x1CD2 => Some(158 + (c as u32 - 0x1CD0)),
        0x1CDA..=0x1CDB => Some(161 + (c as u32 - 0x1CDA)),
        0x1CE0 => Some(163),
        0x1DC0..=0x1DC1 => Some(164 + (c as u32 - 0x1DC0)),
        0x1DC3..=0x1DC9 => Some(166 + (c as u32 - 0x1DC3)),
        0x1DCB..=0x1DCC => Some(173 + (c as u32 - 0x1DCB)),
        0x1DD1..=0x1DE6 => Some(175 + (c as u32 - 0x1DD1)),
        0x1DFE => Some(197),
        0x20D0..=0x20D1 => Some(198 + (c as u32 - 0x20D0)),
        0x20D4..=0x20D7 => Some(200 + (c as u32 - 0x20D4)),
        0x20DB..=0x20DC => Some(204 + (c as u32 - 0x20DB)),
        0x20E1 => Some(206),
        0x20E7 => Some(207),
        0x20E9 => Some(208),
        0x20F0 => Some(209),
        0x2CEF..=0x2CF1 => Some(210 + (c as u32 - 0x2CEF)),
        0x2DE0..=0x2DFF => Some(213 + (c as u32 - 0x2DE0)),
        0xA66F => Some(245),
        0xA67C..=0xA67D => Some(246 + (c as u32 - 0xA67C)),
        0xA6F0..=0xA6F1 => Some(248 + (c as u32 - 0xA6F0)),
        0xA8E0..=0xA8F1 => Some(250 + (c as u32 - 0xA8E0)),
        0xAAB0 => Some(268),
        0xAAB2..=0xAAB3 => Some(269 + (c as u32 - 0xAAB2)),
        0xAAB7..=0xAAB8 => Some(271 + (c as u32 - 0xAAB7)),
        0xAABE..=0xAABF => Some(273 + (c as u32 - 0xAABE)),
        0xAAC1 => Some(275),
        0xFE20..=0xFE26 => Some(276 + (c as u32 - 0xFE20)),
        0x10A0F => Some(283),
        0x10A38 => Some(284),
        0x1D185..=0x1D189 => Some(285 + (c as u32 - 0x1D185)),
        0x1D1AA..=0x1D1AD => Some(290 + (c as u32 - 0x1D1AA)),
        0x1D242..=0x1D244 => Some(294 + (c as u32 - 0x1D242)),
        _ => None,
    };
    // Convert 0-based table index to 1-based value.
    // 0 is reserved as "not specified" sentinel, matching kitty's convention.
    zero_based.map(|v| v + 1).unwrap_or(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_diacritic_boundaries() {
        // 1-based: U+0305 (first entry) -> 1
        assert_eq!(diacritic_to_num('\u{0305}'), 1);
        // U+030D -> 2, U+030E -> 3
        assert_eq!(diacritic_to_num('\u{030D}'), 2);
        assert_eq!(diacritic_to_num('\u{030E}'), 3);
        // Last entry: U+1D244 -> 297
        assert_eq!(diacritic_to_num('\u{1D244}'), 297);
        // Unrecognized -> 0 (sentinel for "not specified")
        assert_eq!(diacritic_to_num('a'), 0);
        assert_eq!(diacritic_to_num('\u{0300}'), 0);
    }

    #[test]
    fn test_diacritic_contiguous_ranges() {
        // Cyrillic: U+0483..=U+0487 -> 31..=35 (1-based)
        assert_eq!(diacritic_to_num('\u{0483}'), 31);
        assert_eq!(diacritic_to_num('\u{0487}'), 35);
        // Latin small letters: U+0363..=U+036F -> 18..=30
        assert_eq!(diacritic_to_num('\u{0363}'), 18);
        assert_eq!(diacritic_to_num('\u{036F}'), 30);
        // Cyrillic extended: U+2DE0..=U+2DFF -> 214..=245
        assert_eq!(diacritic_to_num('\u{2DE0}'), 214);
        assert_eq!(diacritic_to_num('\u{2DFF}'), 245);
    }

    #[test]
    fn test_diacritic_total_count() {
        // 1-based: highest mapped value is 297 (297 entries: 1..=297, 0=unrecognized)
        let max_val = (0u32..=0x10FFFFu32)
            .filter_map(char::from_u32)
            .map(diacritic_to_num)
            .max()
            .unwrap();
        assert_eq!(max_val, 297);
    }
}
