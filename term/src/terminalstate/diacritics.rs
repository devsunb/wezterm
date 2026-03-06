/// Maps a combining diacritical mark to its row/column number (0-296).
/// Returns 0 for unrecognized characters, meaning "not specified" per the spec.
/// Derived from Kitty's rowcolumn-diacritics.txt (Unicode 6.0.0, combining class 230,
/// no decomposition mappings).
pub fn diacritic_to_num(c: char) -> u32 {
    match c as u32 {
        0x0305 => 0,
        0x030D..=0x030E => 1 + (c as u32 - 0x030D),
        0x0310 => 3,
        0x0312 => 4,
        0x033D..=0x033F => 5 + (c as u32 - 0x033D),
        0x0346 => 8,
        0x034A..=0x034C => 9 + (c as u32 - 0x034A),
        0x0350..=0x0352 => 12 + (c as u32 - 0x0350),
        0x0357 => 15,
        0x035B => 16,
        0x0363..=0x036F => 17 + (c as u32 - 0x0363),
        0x0483..=0x0487 => 30 + (c as u32 - 0x0483),
        0x0592..=0x0595 => 35 + (c as u32 - 0x0592),
        0x0597..=0x0599 => 39 + (c as u32 - 0x0597),
        0x059C..=0x05A1 => 42 + (c as u32 - 0x059C),
        0x05A8..=0x05A9 => 48 + (c as u32 - 0x05A8),
        0x05AB..=0x05AC => 50 + (c as u32 - 0x05AB),
        0x05AF => 52,
        0x05C4 => 53,
        0x0610..=0x0617 => 54 + (c as u32 - 0x0610),
        0x0657..=0x065B => 62 + (c as u32 - 0x0657),
        0x065D..=0x065E => 67 + (c as u32 - 0x065D),
        0x06D6..=0x06DC => 69 + (c as u32 - 0x06D6),
        0x06DF..=0x06E2 => 76 + (c as u32 - 0x06DF),
        0x06E4 => 80,
        0x06E7..=0x06E8 => 81 + (c as u32 - 0x06E7),
        0x06EB..=0x06EC => 83 + (c as u32 - 0x06EB),
        0x0730 => 85,
        0x0732..=0x0733 => 86 + (c as u32 - 0x0732),
        0x0735..=0x0736 => 88 + (c as u32 - 0x0735),
        0x073A => 90,
        0x073D => 91,
        0x073F..=0x0741 => 92 + (c as u32 - 0x073F),
        0x0743 => 95,
        0x0745 => 96,
        0x0747 => 97,
        0x0749..=0x074A => 98 + (c as u32 - 0x0749),
        0x07EB..=0x07F1 => 100 + (c as u32 - 0x07EB),
        0x07F3 => 107,
        0x0816..=0x0819 => 108 + (c as u32 - 0x0816),
        0x081B..=0x0823 => 112 + (c as u32 - 0x081B),
        0x0825..=0x0827 => 121 + (c as u32 - 0x0825),
        0x0829..=0x082D => 124 + (c as u32 - 0x0829),
        0x0951 => 129,
        0x0953..=0x0954 => 130 + (c as u32 - 0x0953),
        0x0F82..=0x0F83 => 132 + (c as u32 - 0x0F82),
        0x0F86..=0x0F87 => 134 + (c as u32 - 0x0F86),
        0x135D..=0x135F => 136 + (c as u32 - 0x135D),
        0x17DD => 139,
        0x193A => 140,
        0x1A17 => 141,
        0x1A75..=0x1A7C => 142 + (c as u32 - 0x1A75),
        0x1B6B => 150,
        0x1B6D..=0x1B73 => 151 + (c as u32 - 0x1B6D),
        0x1CD0..=0x1CD2 => 158 + (c as u32 - 0x1CD0),
        0x1CDA..=0x1CDB => 161 + (c as u32 - 0x1CDA),
        0x1CE0 => 163,
        0x1DC0..=0x1DC1 => 164 + (c as u32 - 0x1DC0),
        0x1DC3..=0x1DC9 => 166 + (c as u32 - 0x1DC3),
        0x1DCB..=0x1DCC => 173 + (c as u32 - 0x1DCB),
        0x1DD1..=0x1DE6 => 175 + (c as u32 - 0x1DD1),
        0x1DFE => 197,
        0x20D0..=0x20D1 => 198 + (c as u32 - 0x20D0),
        0x20D4..=0x20D7 => 200 + (c as u32 - 0x20D4),
        0x20DB..=0x20DC => 204 + (c as u32 - 0x20DB),
        0x20E1 => 206,
        0x20E7 => 207,
        0x20E9 => 208,
        0x20F0 => 209,
        0x2CEF..=0x2CF1 => 210 + (c as u32 - 0x2CEF),
        0x2DE0..=0x2DFF => 213 + (c as u32 - 0x2DE0),
        0xA66F => 245,
        0xA67C..=0xA67D => 246 + (c as u32 - 0xA67C),
        0xA6F0..=0xA6F1 => 248 + (c as u32 - 0xA6F0),
        0xA8E0..=0xA8F1 => 250 + (c as u32 - 0xA8E0),
        0xAAB0 => 268,
        0xAAB2..=0xAAB3 => 269 + (c as u32 - 0xAAB2),
        0xAAB7..=0xAAB8 => 271 + (c as u32 - 0xAAB7),
        0xAABE..=0xAABF => 273 + (c as u32 - 0xAABE),
        0xAAC1 => 275,
        0xFE20..=0xFE26 => 276 + (c as u32 - 0xFE20),
        0x10A0F => 283,
        0x10A38 => 284,
        0x1D185..=0x1D189 => 285 + (c as u32 - 0x1D185),
        0x1D1AA..=0x1D1AD => 290 + (c as u32 - 0x1D1AA),
        0x1D242..=0x1D244 => 294 + (c as u32 - 0x1D242),
        _ => 0,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_diacritic_boundaries() {
        // First entry: U+0305 -> 0
        assert_eq!(diacritic_to_num('\u{0305}'), 0);
        // U+030D -> 1, U+030E -> 2
        assert_eq!(diacritic_to_num('\u{030D}'), 1);
        assert_eq!(diacritic_to_num('\u{030E}'), 2);
        // Last entry: U+1D244 -> 296
        assert_eq!(diacritic_to_num('\u{1D244}'), 296);
        // Unrecognized -> 0
        assert_eq!(diacritic_to_num('a'), 0);
        assert_eq!(diacritic_to_num('\u{0300}'), 0);
    }

    #[test]
    fn test_diacritic_contiguous_ranges() {
        // Cyrillic: U+0483..=U+0487 -> 30..=34
        assert_eq!(diacritic_to_num('\u{0483}'), 30);
        assert_eq!(diacritic_to_num('\u{0487}'), 34);
        // Latin small letters: U+0363..=U+036F -> 17..=29
        assert_eq!(diacritic_to_num('\u{0363}'), 17);
        assert_eq!(diacritic_to_num('\u{036F}'), 29);
        // Cyrillic extended: U+2DE0..=U+2DFF -> 213..=244
        assert_eq!(diacritic_to_num('\u{2DE0}'), 213);
        assert_eq!(diacritic_to_num('\u{2DFF}'), 244);
    }

    #[test]
    fn test_diacritic_total_count() {
        // Verify that the highest mapped value is 296 (297 entries total: 0..=296)
        let max_val = (0u32..=0x10FFFFu32)
            .filter_map(char::from_u32)
            .map(diacritic_to_num)
            .max()
            .unwrap();
        assert_eq!(max_val, 296);
    }
}
