use anyhow::anyhow;
use itertools::Itertools;

/// Scores the character based on most frequent English letters.
fn score_char(ch: char) -> i32 {
    static ENGLISH_FREQUENT_CHARS: &str = "etaoin shrdlu";
    if ENGLISH_FREQUENT_CHARS.contains(ch.to_ascii_lowercase()) {
        1000
    } else {
        0
    }
}

/// Cracks single-byte XOR cipher for English plaintext. Returns the plaintext and its score.
pub fn crack_single_xor_cipher<S: AsRef<[u8]>>(input: S) -> anyhow::Result<(String, i32)> {
    let ciphertext = hex::decode(input)?;

    let result = (0..=0xFF)
        .map(|key| ciphertext.iter().map(|v| v ^ key).collect_vec())
        .map(|plain| {
            (
                plain.iter().fold(0, |acc, &v| acc + score_char(v as char)),
                plain,
            )
        })
        .max_by_key(|v| v.0)
        .expect("unfallible");

    Ok((String::from_utf8_lossy(&result.1).to_string(), result.0))
}

pub fn crack_single_xor_cipher_multi_text<I: Iterator<Item = String>>(
    input: I,
) -> anyhow::Result<String> {
    Ok(input
        .filter_map(|line| crack_single_xor_cipher(line).ok())
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
                "1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736"
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
