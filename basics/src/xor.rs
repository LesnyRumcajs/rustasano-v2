use anyhow::ensure;
use itertools::Itertools;

/// Takes two equal-length buffers and produces their XOR combination.
pub fn fixed_xor<S: AsRef<[u8]>>(input1: S, input2: S) -> anyhow::Result<String> {
    let input1 = hex::decode(input1)?;
    let input2 = hex::decode(input2)?;

    ensure!(input1.len() == input2.len());

    let result = input1
        .iter()
        .zip(input2.iter())
        .map(|(&v1, &v2)| v1 ^ v2)
        .collect_vec();

    Ok(hex::encode(result))
}

#[test]
fn fixed_xor_invalid() {
    assert!(fixed_xor("12", "3456").is_err());
    assert!(fixed_xor("12", "1z").is_err());
}
#[test]
fn fixed_xor_valid() {
    assert_eq!(
        "746865206b696420646f6e277420706c6179",
        fixed_xor(
            "1c0111001f010100061a024b53535009181c",
            "686974207468652062756c6c277320657965"
        )
        .unwrap()
    );
}
