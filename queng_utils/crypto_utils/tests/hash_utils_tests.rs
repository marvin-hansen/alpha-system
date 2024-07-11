use crypto_utils::prelude::hash_utils;

#[test]
fn test_sha512_digest() {
    let s = String::from("Hello Hash");
    let hash = hash_utils::sha512_digest(s);

    let actual = hash;
    let expected = "83b52075c4643666beb3929c96dda06e520373d32f9616dfe3df465e93454941128d02ca8d7d1857075694e6c2ee16a69eb18d2fa6e3e33e68398bdaabf4053c";

    assert_eq!(actual, expected);
}
