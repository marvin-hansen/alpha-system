use base16ct;
use sha3::{Digest, Sha3_512};

const SALT: &str = "2451!@43158)c2#$%b30d&(7f653a985>da7";

/// calculates a sha512 digest as lowercase hex encoded UTF string
pub fn sha512_digest(val: String) -> String {
    // https://github.com/RustCrypto/hashes/tree/master/sha3
    let hash = Sha3_512::new()
        .chain_update(val)
        .chain_update(SALT)
        .finalize();

    base16ct::lower::encode_string(&hash)
}
