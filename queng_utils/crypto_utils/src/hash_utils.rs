use base16ct;
use sha3::{Digest, Sha3_512};

const SALT: &str = "2451!@43158)c2#$%b30d&(7f653a985>da7";

/// Calculates a SHA-512 digest as a lowercase hex encoded UTF-8 string.
///
/// This function takes a `String` value as input and calculates the SHA-512 digest
/// using the `sha3` crate. The digest is then encoded as a lowercase hex string using
/// the `base16ct` crate.
///
/// # Arguments
///
/// * `val` - A `String` value to be hashed.
///
/// # Returns
///
/// Returns a `String` representing the SHA-512 digest of the input value as a lowercase
/// hex encoded UTF-8 string.
///
/// # Notes
///
/// The `sha3` crate is used for SHA-512 digest calculation. The `base16ct` crate is used
/// for encoding the digest as a lowercase hex string.
///
/// The `SALT` constant is used to add additional data to the input value before hashing.
/// This can be used to add a salt to the hashing process.
///
/// The function uses the `chain_update` method from the `sha3::Digest` trait to update the
/// digest with the input value and the `SALT` constant.
///
/// The `finalize` method is called to retrieve the final digest value.
///
/// The `base16ct::lower::encode_string` function is used to encode the digest value as a
/// lowercase hex string.
pub fn sha512_digest(val: String) -> String {
    // https://github.com/RustCrypto/hashes/tree/master/sha3
    let hash = Sha3_512::new()
        .chain_update(val)
        .chain_update(SALT)
        .finalize();

    base16ct::lower::encode_string(&hash)
}
