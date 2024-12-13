pub const MAX_CHARS: u64 = 39;

// Checks if a character is valid i.e. in the range [0-9,A-Z,_,-]
#[inline(always)]
pub fn is_valid_char(x: u8) -> bool {
    x.is_ascii_digit() || x.is_ascii_uppercase() || x == b'_' || x == b'-'
}

#[inline(always)]
pub fn lookup_u64(x: u8) -> u64 {
    LOOKUP_INT64.data[x as usize]
}

#[inline(always)]
pub fn lookup_char(x: u64) -> char {
    LOOKUP_STR.data[x as usize]
}

// Wrapper struct for cache aligned array
#[repr(align(64))]
struct AlignedU64Array {
    data: [u64; 128],
}

// The constant lookup table for encoding characters to U64 integers
// Performance of Rust's match vs. lookup tables
// https://kevinlynagh.com/notes/match-vs-lookup/
static LOOKUP_INT64: AlignedU64Array = AlignedU64Array {
    data: {
        let mut arr = [0; 128]; // Actual size is 96 bytes, but 128 aligns better to 64 bytes cache size
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
        arr[b'0' as usize] = 27;
        arr[b'1' as usize] = 28;
        arr[b'2' as usize] = 29;
        arr[b'3' as usize] = 30;
        arr[b'4' as usize] = 31;
        arr[b'5' as usize] = 32;
        arr[b'6' as usize] = 33;
        arr[b'7' as usize] = 34;
        arr[b'8' as usize] = 35;
        arr[b'9' as usize] = 36;
        arr[b'_' as usize] = 37;
        arr[b'-' as usize] = 38;
        arr
    },
};

// Wrapper struct for aligned array
#[repr(align(64))]
struct AlignedCharArray {
    data: [char; MAX_CHARS as usize],
}

// Reverse lookup table that decodes U64 integers to characters
static LOOKUP_STR: AlignedCharArray = AlignedCharArray {
    data: {
        let mut arr = ['\0'; MAX_CHARS as usize];
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
        arr[27] = '0';
        arr[28] = '1';
        arr[29] = '2';
        arr[30] = '3';
        arr[31] = '4';
        arr[32] = '5';
        arr[33] = '6';
        arr[34] = '7';
        arr[35] = '8';
        arr[36] = '9';
        arr[37] = '_';
        arr[38] = '-';
        arr
    },
};
