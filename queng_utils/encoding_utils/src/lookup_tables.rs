const MAX_U128_CHARS: u64 = 64;

/// Validates if a character is valid for encoding.
/// Valid characters are:
/// - Uppercase letters (A-Z)
/// - Lowercase letters (a-z)
/// - Digits (0-9)
/// - Underscore (_)
#[inline(always)]
pub fn validate_char(c: u8) -> bool {
    (c >= b'A' && c <= b'Z') || (c >= b'a' && c <= b'z') || (c >= b'0' && c <= b'9') || c == b'_'
}

/// Converts a byte character to its corresponding u64 value in the encoding scheme.
///
/// The encoding scheme maps:
/// - Uppercase letters (A-Z) to values 1-26
/// - Lowercase letters (a-z) to values 27-52
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
/// - Values 1-26 to uppercase letters (A-Z)
/// - Values 27-52 to lowercase letters (a-z)
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

        // A-Z (Uppercase)
        arr[b'A' as usize] = 1;
        arr[b'B' as usize] = 2;
        arr[b'C' as usize] = 3;
        arr[b'D' as usize] = 4;
        arr[b'E' as usize] = 5;
        arr[b'F' as usize] = 6;
        arr[b'G' as usize] = 7;
        arr[b'H' as usize] = 8;
        arr[b'I' as usize] = 9;
        arr[b'J' as usize] = 10;
        arr[b'K' as usize] = 11;
        arr[b'L' as usize] = 12;
        arr[b'M' as usize] = 13;
        arr[b'N' as usize] = 14;
        arr[b'O' as usize] = 15;
        arr[b'P' as usize] = 16;
        arr[b'Q' as usize] = 17;
        arr[b'R' as usize] = 18;
        arr[b'S' as usize] = 19;
        arr[b'T' as usize] = 20;
        arr[b'U' as usize] = 21;
        arr[b'V' as usize] = 22;
        arr[b'W' as usize] = 23;
        arr[b'X' as usize] = 24;
        arr[b'Y' as usize] = 25;
        arr[b'Z' as usize] = 26;
        // a-z (Lowercase)
        arr[b'a' as usize] = 27;
        arr[b'b' as usize] = 28;
        arr[b'c' as usize] = 29;
        arr[b'd' as usize] = 30;
        arr[b'e' as usize] = 31;
        arr[b'f' as usize] = 32;
        arr[b'g' as usize] = 33;
        arr[b'h' as usize] = 34;
        arr[b'i' as usize] = 35;
        arr[b'j' as usize] = 36;
        arr[b'k' as usize] = 37;
        arr[b'l' as usize] = 38;
        arr[b'm' as usize] = 39;
        arr[b'n' as usize] = 40;
        arr[b'o' as usize] = 41;
        arr[b'p' as usize] = 42;
        arr[b'q' as usize] = 43;
        arr[b'r' as usize] = 44;
        arr[b's' as usize] = 45;
        arr[b't' as usize] = 46;
        arr[b'u' as usize] = 47;
        arr[b'v' as usize] = 48;
        arr[b'w' as usize] = 49;
        arr[b'x' as usize] = 50;
        arr[b'y' as usize] = 51;
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
        // A-Z (Uppercase)
        arr[1] = 'A';
        arr[2] = 'B';
        arr[3] = 'C';
        arr[4] = 'D';
        arr[5] = 'E';
        arr[6] = 'F';
        arr[7] = 'G';
        arr[8] = 'H';
        arr[9] = 'I';
        arr[10] = 'J';
        arr[11] = 'K';
        arr[12] = 'L';
        arr[13] = 'M';
        arr[14] = 'N';
        arr[15] = 'O';
        arr[16] = 'P';
        arr[17] = 'Q';
        arr[18] = 'R';
        arr[19] = 'S';
        arr[20] = 'T';
        arr[21] = 'U';
        arr[22] = 'V';
        arr[23] = 'W';
        arr[24] = 'X';
        arr[25] = 'Y';
        arr[26] = 'Z';
        // a-z (Lowercase)
        arr[27] = 'a';
        arr[28] = 'b';
        arr[29] = 'c';
        arr[30] = 'd';
        arr[31] = 'e';
        arr[32] = 'f';
        arr[33] = 'g';
        arr[34] = 'h';
        arr[35] = 'i';
        arr[36] = 'j';
        arr[37] = 'k';
        arr[38] = 'l';
        arr[39] = 'm';
        arr[40] = 'n';
        arr[41] = 'o';
        arr[42] = 'p';
        arr[43] = 'q';
        arr[44] = 'r';
        arr[45] = 's';
        arr[46] = 't';
        arr[47] = 'u';
        arr[48] = 'v';
        arr[49] = 'w';
        arr[50] = 'x';
        arr[51] = 'y';
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
        // Special characters
        arr[63] = '_';

        arr
    },
};
