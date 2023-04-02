use itertools::Itertools;

pub fn repeating_key_xor(key: &str, input: &str) -> anyhow::Result<String> {
    let key = key.as_bytes();
    let input = input.as_bytes();

    let ciphertext = input
        .iter()
        .zip(key.iter().cycle())
        .map(|(x, y)| x ^ y)
        .collect_vec();

    Ok(hex::encode(ciphertext))
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn repeating_key_xor_test() -> anyhow::Result<()> {
        let input = "Burning 'em, if you ain't quick and nimble\nI go crazy when I hear a cymbal";
        let expected = "0b3637272a2b2e63622c2e69692a23693a2a3c6324202d623d63343c2a26226324272765272a282b2f20430a652e2c652a3124333a653e2b2027630c692b20283165286326302e27282f";
        assert_eq!(expected, repeating_key_xor("ICE", input)?);

        Ok(())
    }
}
