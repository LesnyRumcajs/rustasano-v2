use itertools::Itertools;

/// Scores the character based on most frequent English letters.
fn score_char(ch: char) -> u32 {
    static ENGLISH_FREQUENT_CHARS: &str = "ETAOIN SHRDLU";
    if ENGLISH_FREQUENT_CHARS.contains(ch) {
        10
    } else {
        0
    }
}

/// Cracks single-byte XOR cipher for English plaintext.
fn crack_single_xor_cipher<S: AsRef<[u8]>>(input: S) -> anyhow::Result<String> {
    let ciphertext = hex::decode(input)?;

    let result = (0..=0xFF)
        .map(|key| ciphertext.iter().map(|v| v ^ key).collect_vec())
        .map(|plain| {
            (
                plain.iter().fold(0, |acc, &v| acc + score_char(v as char)),
                plain,
            )
        })
        .sorted_by(|a, b| Ord::cmp(&b.0, &a.0))
        .next()
        .expect("unfallible");

    Ok(String::from_utf8_lossy(&result.1).to_string())
}

fn main() -> anyhow::Result<()> {
    let input = std::env::args()
        .nth(1)
        .ok_or_else(|| anyhow::anyhow!("No input provided!"))?;

    println!("{}", crack_single_xor_cipher(input)?);

    Ok(())
}

#[test]
fn crack_test() {
    assert_eq!(
        "cOOKING\0mc\u{7}S\0LIKE\0A\0POUND\0OF\0BACON",
        crack_single_xor_cipher(
            "1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736"
        )
        .unwrap()
    );
}
