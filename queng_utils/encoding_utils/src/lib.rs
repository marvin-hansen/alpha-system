/*
 * Copyright (c) "2025" . Marvin Hansen All Rights Reserved.
 */

//! A high-performance library for encoding strings into fixed-size integers.
//!
//! This crate provides efficient implementations for encoding strings into various
//! integer formats (u64, u128) and back. The encoding scheme is optimized for
//! alphanumeric strings and supports the following features:
//!
//! - Space-efficient 6-bit character encoding
//! - Support for alphanumeric characters (A-Z, a-z, 0-9) and underscore
//! - Multiple encoding formats:
//!   * Single u128 (up to 21 characters)
//!   * Pair of u64s (up to 20 characters)
//! - Fast encoding/decoding with minimal allocations
//! - Comprehensive error handling
//!
//! # Example
//! ```
//! use encoding_utils::{encode_str_to_pair_u64, decode_pair_64_to_str};
//!
//! // Encode a string into a pair of u64s
//! let encoded = encode_str_to_pair_u64("Hello_World123").unwrap();
//!
//! // Decode back to the original string
//! let decoded = decode_pair_64_to_str(encoded).unwrap();
//! assert_eq!("Hello_World123", decoded);
//! ```

mod error_decoding;
mod error_encoding;
mod lookup_tables;
mod string_int128_codec;
mod string_int64_codec;
mod string_int64_pair_codec;

pub use error_decoding::BinaryDecodingError as DecodingError;
pub use error_encoding::BinaryEncodingError as EncodingError;
pub use lookup_tables::validate_char;
pub use string_int64_codec::{decode_int64_to_str, encode_str_to_int64};
pub use string_int64_pair_codec::{decode_pair_64_to_str, encode_str_to_pair_u64};
pub use string_int128_codec::{decode_int128_to_str, encode_str_to_int128};
