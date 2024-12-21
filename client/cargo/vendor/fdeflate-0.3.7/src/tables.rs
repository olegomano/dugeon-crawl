use crate::decompress::{EXCEPTIONAL_ENTRY, LITERAL_ENTRY};

/// Hard-coded Huffman codes used regardless of the input.
///
/// These values work well for PNGs with some form of filtering enabled, but will likely make most
/// other inputs worse.
pub(crate) const HUFFMAN_LENGTHS: [u8; 286] = [
    2, 3, 4, 5, 5, 6, 6, 7, 7, 7, 8, 8, 8, 8, 8, 9, 9, 9, 9, 9, 9, 9, 10, 10, 10, 10, 10, 10, 10,
    10, 10, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 12, 12, 12, 12, 12, 12, 12, 12, 12, 12,
    12, 12, 12, 12, 12, 12, 12, 12, 12, 12, 12, 12, 12, 12, 12, 12, 12, 12, 12, 12, 12, 12, 12, 12,
    12, 12, 12, 12, 12, 12, 12, 12, 12, 12, 12, 12, 12, 12, 12, 12, 12, 12, 12, 12, 12, 12, 12, 12,
    12, 12, 12, 12, 12, 12, 12, 12, 12, 12, 12, 12, 12, 12, 12, 12, 12, 12, 12, 12, 12, 12, 12, 12,
    12, 12, 12, 12, 12, 12, 12, 12, 12, 12, 12, 12, 12, 12, 12, 12, 12, 12, 12, 12, 12, 12, 12, 12,
    12, 12, 12, 12, 12, 12, 12, 12, 12, 12, 12, 12, 12, 12, 12, 12, 12, 12, 12, 12, 12, 12, 12, 12,
    12, 12, 12, 12, 12, 12, 12, 12, 12, 12, 12, 12, 12, 12, 12, 12, 12, 12, 12, 12, 12, 12, 12, 12,
    12, 12, 12, 12, 12, 12, 12, 12, 12, 12, 12, 12, 12, 12, 12, 12, 12, 11, 11, 11, 11, 11, 11, 11,
    11, 11, 11, 10, 11, 10, 10, 10, 10, 10, 10, 10, 10, 10, 9, 9, 9, 9, 9, 8, 9, 8, 8, 8, 8, 8, 7,
    7, 7, 6, 6, 6, 5, 4, 3, 12, 12, 12, 9, 9, 11, 10, 11, 11, 10, 11, 11, 11, 11, 11, 11, 12, 11,
    12, 12, 12, 12, 12, 12, 12, 12, 12, 12, 12, 9,
];

pub(crate) const HUFFMAN_CODES: [u16; 286] = match crate::compute_codes(&HUFFMAN_LENGTHS) {
    Some(codes) => codes,
    None => panic!("HUFFMAN_LENGTHS is invalid"),
};

/// Length code for length values (derived from deflate spec).
pub(crate) const LENGTH_TO_SYMBOL: [u16; 256] = [
    257, 258, 259, 260, 261, 262, 263, 264, 265, 265, 266, 266, 267, 267, 268, 268, 269, 269, 269,
    269, 270, 270, 270, 270, 271, 271, 271, 271, 272, 272, 272, 272, 273, 273, 273, 273, 273, 273,
    273, 273, 274, 274, 274, 274, 274, 274, 274, 274, 275, 275, 275, 275, 275, 275, 275, 275, 276,
    276, 276, 276, 276, 276, 276, 276, 277, 277, 277, 277, 277, 277, 277, 277, 277, 277, 277, 277,
    277, 277, 277, 277, 278, 278, 278, 278, 278, 278, 278, 278, 278, 278, 278, 278, 278, 278, 278,
    278, 279, 279, 279, 279, 279, 279, 279, 279, 279, 279, 279, 279, 279, 279, 279, 279, 280, 280,
    280, 280, 280, 280, 280, 280, 280, 280, 280, 280, 280, 280, 280, 280, 281, 281, 281, 281, 281,
    281, 281, 281, 281, 281, 281, 281, 281, 281, 281, 281, 281, 281, 281, 281, 281, 281, 281, 281,
    281, 281, 281, 281, 281, 281, 281, 281, 282, 282, 282, 282, 282, 282, 282, 282, 282, 282, 282,
    282, 282, 282, 282, 282, 282, 282, 282, 282, 282, 282, 282, 282, 282, 282, 282, 282, 282, 282,
    282, 282, 283, 283, 283, 283, 283, 283, 283, 283, 283, 283, 283, 283, 283, 283, 283, 283, 283,
    283, 283, 283, 283, 283, 283, 283, 283, 283, 283, 283, 283, 283, 283, 283, 284, 284, 284, 284,
    284, 284, 284, 284, 284, 284, 284, 284, 284, 284, 284, 284, 284, 284, 284, 284, 284, 284, 284,
    284, 284, 284, 284, 284, 284, 284, 284, 285,
];

/// Number of extra bits for length values (derived from deflate spec).
pub(crate) const LENGTH_TO_LEN_EXTRA: [u8; 256] = [
    0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 1, 1, 1, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2,
    3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3,
    4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4,
    4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4,
    5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5,
    5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5,
    5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5,
    5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 0,
];

pub(crate) const BITMASKS: [u32; 17] = [
    0x0000, 0x0001, 0x0003, 0x0007, 0x000F, 0x001F, 0x003F, 0x007F, 0x00FF, 0x01FF, 0x03FF, 0x07FF,
    0x0FFF, 0x1FFF, 0x3FFF, 0x7FFF, 0xFFFF,
];

/// Order of the length code length alphabet (derived from deflate spec).
pub(crate) const CLCL_ORDER: [usize; 19] = [
    16, 17, 18, 0, 8, 7, 9, 6, 10, 5, 11, 4, 12, 3, 13, 2, 14, 1, 15,
];

/// Number of extra bits for each length code (derived from deflate spec).
pub(crate) const LEN_SYM_TO_LEN_EXTRA: [u8; 29] = [
    0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 2, 2, 2, 2, 3, 3, 3, 3, 4, 4, 4, 4, 5, 5, 5, 5, 0,
];

/// The base length for each length code (derived from deflate spec).
pub(crate) const LEN_SYM_TO_LEN_BASE: [usize; 29] = [
    3, 4, 5, 6, 7, 8, 9, 10, 11, 13, 15, 17, 19, 23, 27, 31, 35, 43, 51, 59, 67, 83, 99, 115, 131,
    163, 195, 227, 258,
];

/// Number of extra bits for each distance code (derived from deflate spec.)
pub(crate) const DIST_SYM_TO_DIST_EXTRA: [u8; 30] = [
    0, 0, 0, 0, 1, 1, 2, 2, 3, 3, 4, 4, 5, 5, 6, 6, 7, 7, 8, 8, 9, 9, 10, 10, 11, 11, 12, 12, 13,
    13,
];

/// The base distance for each distance code (derived from deflate spec).
pub(crate) const DIST_SYM_TO_DIST_BASE: [u16; 30] = [
    1, 2, 3, 4, 5, 7, 9, 13, 17, 25, 33, 49, 65, 97, 129, 193, 257, 385, 513, 769, 1025, 1537,
    2049, 3073, 4097, 6145, 8193, 12289, 16385, 24577,
];

/// The main litlen_table uses a 12-bit input to lookup the meaning of the symbol. The table is
/// split into 4 sections:
///
///   aaaaaaaa_bbbbbbbb_1000yyyy_0000xxxx  x = input_advance_bits, y = output_advance_bytes (literal)
///   0000000z_zzzzzzzz_00000yyy_0000xxxx  x = input_advance_bits, y = extra_bits, z = distance_base (length)
///   00000000_00000000_01000000_0000xxxx  x = input_advance_bits (EOF)
///   0000xxxx_xxxxxxxx_01100000_00000000  x = secondary_table_index
///   00000000_00000000_01000000_00000000  invalid code
pub(crate) const LITLEN_TABLE_ENTRIES: [u32; 288] = {
    let mut entries = [EXCEPTIONAL_ENTRY; 288];
    let mut i = 0;
    while i < 256 {
        entries[i] = (i as u32) << 16 | LITERAL_ENTRY | (1 << 8);
        i += 1;
    }

    let mut i = 257;
    while i < 286 {
        entries[i] = (LEN_SYM_TO_LEN_BASE[i - 257] as u32) << 16
            | (LEN_SYM_TO_LEN_EXTRA[i - 257] as u32) << 8;
        i += 1;
    }
    entries
};

/// The distance table is a 512-entry table that maps 9 bits of distance symbols to their meaning.
///
///   00000000_00000000_00000000_00000000     symbol is more than 9 bits
///   zzzzzzzz_zzzzzzzz_0000yyyy_0000xxxx     x = input_advance_bits, y = extra_bits, z = distance_base
pub(crate) const DISTANCE_TABLE_ENTRIES: [u32; 32] = {
    let mut entries = [0; 32];
    let mut i = 0;
    while i < 30 {
        entries[i] = (DIST_SYM_TO_DIST_BASE[i] as u32) << 16
            | (DIST_SYM_TO_DIST_EXTRA[i] as u32) << 8
            | LITERAL_ENTRY;
        i += 1;
    }
    entries
};

pub(crate) const FIXED_LITLEN_TABLE: [u32; 512] = [
    16391, 5275912, 1081608, 7537672, 2032135, 7373064, 3178760, 12615945, 655367, 6324488,
    2130184, 10518793, 33032, 8421640, 4227336, 14713097, 393223, 5800200, 1605896, 9470217,
    3867399, 7897352, 3703048, 13664521, 1114375, 6848776, 2654472, 11567369, 557320, 8945928,
    4751624, 15761673, 262151, 5538056, 1343752, 14877960, 2818823, 7635208, 3440904, 13140233,
    852231, 6586632, 2392328, 11043081, 295176, 8683784, 4489480, 15237385, 524295, 6062344,
    1868040, 9994505, 5440519, 8159496, 3965192, 14188809, 1507847, 7110920, 2916616, 12091657,
    819464, 9208072, 5013768, 16285961, 196615, 5406984, 1212680, 10683656, 2294535, 7504136,
    3309832, 12878089, 721159, 6455560, 2261256, 10780937, 164104, 8552712, 4358408, 14975241,
    458759, 5931272, 1736968, 9732361, 4391943, 8028424, 3834120, 13926665, 1245703, 6979848,
    2785544, 11829513, 688392, 9077000, 4882696, 16023817, 327687, 5669128, 1474824, 16392,
    3343111, 7766280, 3571976, 13402377, 983303, 6717704, 2523400, 11305225, 426248, 8814856,
    4620552, 15499529, 589831, 6193416, 1999112, 10256649, 6489095, 8290568, 4096264, 14450953,
    1769991, 7241992, 3047688, 12353801, 950536, 9339144, 5144840, 16548105, 16391, 5341448,
    1147144, 8586504, 2032135, 7438600, 3244296, 12747017, 655367, 6390024, 2195720, 10649865,
    98568, 8487176, 4292872, 14844169, 393223, 5865736, 1671432, 9601289, 3867399, 7962888,
    3768584, 13795593, 1114375, 6914312, 2720008, 11698441, 622856, 9011464, 4817160, 15892745,
    262151, 5603592, 1409288, 16908296, 2818823, 7700744, 3506440, 13271305, 852231, 6652168,
    2457864, 11174153, 360712, 8749320, 4555016, 15368457, 524295, 6127880, 1933576, 10125577,
    5440519, 8225032, 4030728, 14319881, 1507847, 7176456, 2982152, 12222729, 885000, 9273608,
    5079304, 16417033, 196615, 5472520, 1278216, 12780808, 2294535, 7569672, 3375368, 13009161,
    721159, 6521096, 2326792, 10912009, 229640, 8618248, 4423944, 15106313, 458759, 5996808,
    1802504, 9863433, 4391943, 8093960, 3899656, 14057737, 1245703, 7045384, 2851080, 11960585,
    753928, 9142536, 4948232, 16154889, 327687, 5734664, 1540360, 16392, 3343111, 7831816, 3637512,
    13533449, 983303, 6783240, 2588936, 11436297, 491784, 8880392, 4686088, 15630601, 589831,
    6258952, 2064648, 10387721, 6489095, 8356104, 4161800, 14582025, 1769991, 7307528, 3113224,
    12484873, 1016072, 9404680, 5210376, 16679177, 16391, 5275912, 1081608, 7537672, 2032135,
    7373064, 3178760, 12681481, 655367, 6324488, 2130184, 10584329, 33032, 8421640, 4227336,
    14778633, 393223, 5800200, 1605896, 9535753, 3867399, 7897352, 3703048, 13730057, 1114375,
    6848776, 2654472, 11632905, 557320, 8945928, 4751624, 15827209, 262151, 5538056, 1343752,
    14877960, 2818823, 7635208, 3440904, 13205769, 852231, 6586632, 2392328, 11108617, 295176,
    8683784, 4489480, 15302921, 524295, 6062344, 1868040, 10060041, 5440519, 8159496, 3965192,
    14254345, 1507847, 7110920, 2916616, 12157193, 819464, 9208072, 5013768, 16351497, 196615,
    5406984, 1212680, 10683656, 2294535, 7504136, 3309832, 12943625, 721159, 6455560, 2261256,
    10846473, 164104, 8552712, 4358408, 15040777, 458759, 5931272, 1736968, 9797897, 4391943,
    8028424, 3834120, 13992201, 1245703, 6979848, 2785544, 11895049, 688392, 9077000, 4882696,
    16089353, 327687, 5669128, 1474824, 16392, 3343111, 7766280, 3571976, 13467913, 983303,
    6717704, 2523400, 11370761, 426248, 8814856, 4620552, 15565065, 589831, 6193416, 1999112,
    10322185, 6489095, 8290568, 4096264, 14516489, 1769991, 7241992, 3047688, 12419337, 950536,
    9339144, 5144840, 16613641, 16391, 5341448, 1147144, 8586504, 2032135, 7438600, 3244296,
    12812553, 655367, 6390024, 2195720, 10715401, 98568, 8487176, 4292872, 14909705, 393223,
    5865736, 1671432, 9666825, 3867399, 7962888, 3768584, 13861129, 1114375, 6914312, 2720008,
    11763977, 622856, 9011464, 4817160, 15958281, 262151, 5603592, 1409288, 16908296, 2818823,
    7700744, 3506440, 13336841, 852231, 6652168, 2457864, 11239689, 360712, 8749320, 4555016,
    15433993, 524295, 6127880, 1933576, 10191113, 5440519, 8225032, 4030728, 14385417, 1507847,
    7176456, 2982152, 12288265, 885000, 9273608, 5079304, 16482569, 196615, 5472520, 1278216,
    12780808, 2294535, 7569672, 3375368, 13074697, 721159, 6521096, 2326792, 10977545, 229640,
    8618248, 4423944, 15171849, 458759, 5996808, 1802504, 9928969, 4391943, 8093960, 3899656,
    14123273, 1245703, 7045384, 2851080, 12026121, 753928, 9142536, 4948232, 16220425, 327687,
    5734664, 1540360, 16392, 3343111, 7831816, 3637512, 13598985, 983303, 6783240, 2588936,
    11501833, 491784, 8880392, 4686088, 15696137, 589831, 6258952, 2064648, 10453257, 6489095,
    8356104, 4161800, 14647561, 1769991, 7307528, 3113224, 12550409, 1016072, 9404680, 5210376,
    16744713,
];

pub(crate) const FIXED_DIST_TABLE: [u32; 32] = [
    98309, 16877317, 1147653, 268536581, 360709, 67209477, 4293893, 1073843461, 229381, 33654789,
    2196485, 536972293, 623109, 134318597, 8488453, 5, 163845, 25265925, 1671941, 402754309,
    491781, 100763909, 6391045, 1610714373, 294917, 50432005, 3245061, 805407749, 885253,
    201427461, 12682757, 5,
];

#[cfg(test)]
pub(crate) const FIXED_CODE_LENGTHS: [u8; 320] = make_fixed_code_lengths();

#[cfg(test)]
const fn make_fixed_code_lengths() -> [u8; 320] {
    let mut i = 0;
    let mut lengths = [0; 320];
    while i < 144 {
        lengths[i] = 8;
        i += 1;
    }
    while i < 256 {
        lengths[i] = 9;
        i += 1;
    }
    while i < 280 {
        lengths[i] = 7;
        i += 1;
    }
    while i < 288 {
        lengths[i] = 8;
        i += 1;
    }
    while i < 320 {
        lengths[i] = 5;
        i += 1;
    }
    lengths
}