use base64::{prelude::BASE64_STANDARD, Engine};
use itertools::Itertools;
use rayon::prelude::{IntoParallelRefIterator, ParallelIterator};

use crate::{single_xor_cipher::crack_single_xor_cipher, xor::compute_hamming_distance};

pub fn repeating_key_xor(key: &[u8], input: &[u8]) -> anyhow::Result<String> {
    let ciphertext = input
        .iter()
        .zip(key.iter().cycle())
        .map(|(x, y)| x ^ y)
        .collect_vec();

    Ok(hex::encode(ciphertext))
}

pub fn break_repeating_key_xor(input: &str) -> anyhow::Result<String> {
    let input = input.lines().join("");
    let ciphertext = BASE64_STANDARD.decode(input)?;

    // Guess the key size from the hamming distance
    let keysizes = (2..=40)
        .map(|keysize| {
            let chunk1 = &ciphertext[0..keysize];
            let chunk2 = &ciphertext[keysize..keysize * 2];
            let chunk3 = &ciphertext[keysize * 2..keysize * 3];
            let chunk4 = &ciphertext[keysize * 3..keysize * 4];

            let distance = compute_hamming_distance(chunk1, chunk2);
            let distance2 = compute_hamming_distance(chunk3, chunk4);

            let distance = (distance + distance2) / 2;

            (keysize, distance / keysize as u32)
        })
        .sorted_by_key(|x| x.1)
        .map(|x| x.0)
        .take(10)
        .collect_vec();

    let winner = keysizes
        .par_iter()
        .map(|&keysize| {
            // transpose
            let blocks = ciphertext.chunks(keysize).collect_vec();
            let mut candidate: Vec<Vec<u8>> = vec![vec![]; blocks.len()];

            for block in blocks.iter() {
                for (j, byte) in block.iter().enumerate() {
                    candidate[j].push(*byte);
                }
            }

            let (score, key) =
                candidate
                    .iter()
                    .take(keysize)
                    .fold((0, Vec::new()), |(score, mut key), entry| {
                        let (_, score_partial, key_partial) = crack_single_xor_cipher(entry)
                            .expect("fallible but too much effort to handle");
                        key.push(key_partial);
                        (score + score_partial, key)
                    });
            Ok((score, key))
        })
        .filter_map(|v: anyhow::Result<_>| v.ok())
        .max_by_key(|x| x.0)
        .expect("infallible");

    let result = hex::decode(repeating_key_xor(&winner.1, &ciphertext)?)?;
    let result = String::from_utf8_lossy(&result);

    Ok(result.to_string())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn repeating_key_xor_test() -> anyhow::Result<()> {
        let input = "Burning 'em, if you ain't quick and nimble\nI go crazy when I hear a cymbal";
        let expected = "0b3637272a2b2e63622c2e69692a23693a2a3c6324202d623d63343c2a26226324272765272a282b2f20430a652e2c652a3124333a653e2b2027630c692b20283165286326302e27282f";
        assert_eq!(expected, repeating_key_xor(b"ICE", input.as_bytes())?);

        Ok(())
    }

    #[test]
    fn break_repeating_key_xor_test() -> anyhow::Result<()> {
        let input = std::fs::read_to_string("inputs/challenge6.txt")?;
        let expected = std::fs::read_to_string("inputs/challenge6_plain.txt")?;
        // key:  Terminator X: Bring the noise
        let result = break_repeating_key_xor(&input)?;

        assert_eq!(expected, result);

        Ok(())
    }
}
