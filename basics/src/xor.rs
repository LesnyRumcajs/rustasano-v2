use anyhow::ensure;
use itertools::Itertools;

/// Computes the Hamming/edit distance, i.e., number of bits differing.
pub fn compute_hamming_distance(x: &[u8], y: &[u8]) -> u32 {
    x.iter()
        .zip(y.iter())
        .fold(0, |acc, (x, y)| acc + (x ^ y).count_ones())
}

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

#[cfg(test)]
mod test {
    use super::*;

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

    #[test]
    fn hamming_distance_test() {
        assert_eq!(
            37,
            compute_hamming_distance("this is a test".as_bytes(), "wokka wokka!!!".as_bytes())
        );
    }
}
