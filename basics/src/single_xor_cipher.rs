use anyhow::anyhow;
use itertools::Itertools;

/// Scores the character based on most frequent English letters.
fn score_char(ch: char) -> i32 {
    static ENGLISH_FREQUENT_CHARS: &str = "etaoin shrdlu";
    if ENGLISH_FREQUENT_CHARS.contains(ch.to_ascii_lowercase()) {
        5
    } else if ch.is_ascii_alphabetic() {
        1
    } else {
        -5
    }
}

/// Cracks single-byte XOR cipher for English plaintext. Returns the plaintext, its score and key.
pub fn crack_single_xor_cipher(input: &[u8]) -> anyhow::Result<(String, i32, u8)> {
    let result = (0..=0xFF)
        .map(|key| (input.iter().map(|v| v ^ key).collect_vec(), key))
        .map(|(plain, key)| {
            (
                plain.iter().fold(0, |acc, &v| acc + score_char(v as char)),
                plain,
                key,
            )
        })
        .max_by_key(|v| v.0)
        .expect("unfallible");

    Ok((
        String::from_utf8_lossy(&result.1).to_string(),
        result.0,
        result.2,
    ))
}

pub fn crack_single_xor_cipher_multi_text<I: Iterator<Item = String>>(
    input: I,
) -> anyhow::Result<String> {
    Ok(input
        .filter_map(|line| crack_single_xor_cipher(&hex::decode(line).ok()?).ok())
        .max_by_key(|x| x.1)
        .ok_or_else(|| anyhow!("Couldn't find plaintext"))?
        .0)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{
        fs::File,
        io::{BufRead, BufReader},
    };

    #[test]
    fn crack_test() {
        assert_eq!(
            "Cooking MC's like a pound of bacon",
            crack_single_xor_cipher(
                &hex::decode(
                    "1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736"
                )
                .unwrap()
            )
            .unwrap()
            .0
        );
    }

    #[test]
    fn crack_test_multi() -> anyhow::Result<()> {
        let input = BufReader::new(File::open("inputs/challenge4.txt")?);

        assert_eq!(
            "Now that the party is jumping\n",
            crack_single_xor_cipher_multi_text(input.lines().filter_map(|v| v.ok()))?
        );

        Ok(())
    }
}
