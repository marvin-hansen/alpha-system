/*
 * Copyright (c) "2025" . Marvin Hansen All Rights Reserved.
 */

const MAX_U128_CHARS: u64 = 64;

/// Validates if a character is valid for encoding.
/// Valid characters are:
/// - Uppercase letters (A-Z)
/// - Lowercase letters (a-z)
/// - Digits (0-9)
/// - Underscore (_)
#[inline(always)]
pub fn validate_char(c: u8) -> bool {
    c.is_ascii_uppercase() || c.is_ascii_lowercase() || c.is_ascii_digit() || c == b'_'
}

/// Converts a byte character to its corresponding u64 value in the encoding scheme.
///
/// The encoding scheme maps:
/// - Uppercase letters (A-Z) to values 1-26 (sorted by frequency)
/// - Lowercase letters (a-z) to values 27-52 (sorted by frequency)
/// - Digits (0-9) to values 53-62
/// - Underscore (_) to value 63
/// - Invalid characters to 0
///
/// # Arguments
/// * `x` - The byte character to convert
///
/// # Returns
/// The encoded u64 value, or 0 if the character is invalid
#[inline(always)]
pub fn lookup_u64(x: u8) -> u64 {
    LOOKUP_INT64.data[x as usize]
}

/// Converts an encoded u64 value back to its corresponding character.
///
/// The decoding scheme maps:
/// - Values 1-26 to uppercase letters (A-Z) (sorted by frequency)
/// - Values 27-52 to lowercase letters (a-z) (sorted by frequency)
/// - Values 53-62 to digits (0-9)
/// - Value 63 to underscore (_)
/// - Invalid values to '\0'
///
/// # Arguments
/// * `x` - The encoded value to convert
///
/// # Returns
/// The decoded character, or '\0' if the value is invalid
#[inline(always)]
pub fn lookup_char(x: u64) -> char {
    LOOKUP_STR.data[x as usize]
}

// Wrapper struct for cache aligned array
#[repr(align(128))]
struct AlignedIntArray {
    data: [u64; 128],
}

static LOOKUP_INT64: AlignedIntArray = AlignedIntArray {
    data: {
        let mut arr = [0; 128]; // Actual size is 96 bytes, but 128 aligns better to 64 bytes cache size

        // A-Z (Uppercase, sorted by letter frequency)
        // https://pi.math.cornell.edu/~mec/2003-2004/cryptography/subs/frequencies.html
        arr[b'E' as usize] = 1;
        arr[b'T' as usize] = 2;
        arr[b'A' as usize] = 3;
        arr[b'O' as usize] = 4;
        arr[b'I' as usize] = 5;
        arr[b'N' as usize] = 6;
        arr[b'S' as usize] = 7;
        arr[b'R' as usize] = 8;
        arr[b'H' as usize] = 9;
        arr[b'D' as usize] = 10;
        arr[b'L' as usize] = 11;
        arr[b'U' as usize] = 12;
        arr[b'C' as usize] = 13;
        arr[b'M' as usize] = 14;
        arr[b'F' as usize] = 15;
        arr[b'Y' as usize] = 16;
        arr[b'W' as usize] = 17;
        arr[b'G' as usize] = 18;
        arr[b'P' as usize] = 19;
        arr[b'B' as usize] = 20;
        arr[b'V' as usize] = 21;
        arr[b'K' as usize] = 22;
        arr[b'X' as usize] = 23;
        arr[b'Q' as usize] = 24;
        arr[b'J' as usize] = 25;
        arr[b'Z' as usize] = 26;

        // a-z (Lowercase, sorted by letter frequency)
        arr[b'e' as usize] = 27;
        arr[b't' as usize] = 28;
        arr[b'a' as usize] = 29;
        arr[b'o' as usize] = 30;
        arr[b'i' as usize] = 31;
        arr[b'n' as usize] = 32;
        arr[b's' as usize] = 33;
        arr[b'r' as usize] = 34;
        arr[b'h' as usize] = 35;
        arr[b'd' as usize] = 36;
        arr[b'l' as usize] = 37;
        arr[b'u' as usize] = 38;
        arr[b'c' as usize] = 39;
        arr[b'm' as usize] = 40;
        arr[b'f' as usize] = 41;
        arr[b'y' as usize] = 42;
        arr[b'w' as usize] = 43;
        arr[b'g' as usize] = 44;
        arr[b'p' as usize] = 45;
        arr[b'b' as usize] = 46;
        arr[b'v' as usize] = 47;
        arr[b'k' as usize] = 48;
        arr[b'x' as usize] = 49;
        arr[b'q' as usize] = 50;
        arr[b'j' as usize] = 51;
        arr[b'z' as usize] = 52;

        // 0-9 (Numbers)
        arr[b'0' as usize] = 53;
        arr[b'1' as usize] = 54;
        arr[b'2' as usize] = 55;
        arr[b'3' as usize] = 56;
        arr[b'4' as usize] = 57;
        arr[b'5' as usize] = 58;
        arr[b'6' as usize] = 59;
        arr[b'7' as usize] = 60;
        arr[b'8' as usize] = 61;
        arr[b'9' as usize] = 62;

        // Special characters (_)
        arr[b'_' as usize] = 63;

        arr
    },
};

#[repr(align(64))]
struct AlignedCharArray {
    data: [char; MAX_U128_CHARS as usize],
}

static LOOKUP_STR: AlignedCharArray = AlignedCharArray {
    data: {
        let mut arr = ['\0'; MAX_U128_CHARS as usize];

        // A-Z (Uppercase, sorted by Letter frequency)
        arr[1] = 'E';
        arr[2] = 'T';
        arr[3] = 'A';
        arr[4] = 'O';
        arr[5] = 'I';
        arr[6] = 'N';
        arr[7] = 'S';
        arr[8] = 'R';
        arr[9] = 'H';
        arr[10] = 'D';
        arr[11] = 'L';
        arr[12] = 'U';
        arr[13] = 'C';
        arr[14] = 'M';
        arr[15] = 'F';
        arr[16] = 'Y';
        arr[17] = 'W';
        arr[18] = 'G';
        arr[19] = 'P';
        arr[20] = 'B';
        arr[21] = 'V';
        arr[22] = 'K';
        arr[23] = 'X';
        arr[24] = 'Q';
        arr[25] = 'J';
        arr[26] = 'Z';

        // a-z (Lowercase, sorted by Letter frequency)
        arr[27] = 'e';
        arr[28] = 't';
        arr[29] = 'a';
        arr[30] = 'o';
        arr[31] = 'i';
        arr[32] = 'n';
        arr[33] = 's';
        arr[34] = 'r';
        arr[35] = 'h';
        arr[36] = 'd';
        arr[37] = 'l';
        arr[38] = 'u';
        arr[39] = 'c';
        arr[40] = 'm';
        arr[41] = 'f';
        arr[42] = 'y';
        arr[43] = 'w';
        arr[44] = 'g';
        arr[45] = 'p';
        arr[46] = 'b';
        arr[47] = 'v';
        arr[48] = 'k';
        arr[49] = 'x';
        arr[50] = 'q';
        arr[51] = 'j';
        arr[52] = 'z';

        // 0-9
        arr[53] = '0';
        arr[54] = '1';
        arr[55] = '2';
        arr[56] = '3';
        arr[57] = '4';
        arr[58] = '5';
        arr[59] = '6';
        arr[60] = '7';
        arr[61] = '8';
        arr[62] = '9';

        // Special characters (_)
        arr[63] = '_';

        arr
    },
};
