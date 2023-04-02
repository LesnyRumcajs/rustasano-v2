use base64::{prelude::BASE64_STANDARD, Engine};

pub fn hex_to_base64<S: AsRef<[u8]>>(input: S) -> anyhow::Result<String> {
    let raw = hex::decode(input)?;
    Ok(BASE64_STANDARD.encode(raw))
}

#[test]
fn hex_to_base64_invalid_should_fail() {
    assert!(hex_to_base64("zz").is_err());
}

#[test]
fn hex_to_base64_valid() {
    assert_eq!("SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t", hex_to_base64("49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d").unwrap());
}
