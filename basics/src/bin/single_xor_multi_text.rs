use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use basics::single_xor_cipher::crack_single_xor_cipher_multi_text;

fn main() -> anyhow::Result<()> {
    let input = std::env::args()
        .nth(1)
        .ok_or_else(|| anyhow::anyhow!("No input provided!"))?;

    let input = BufReader::new(File::open(input)?);

    println!(
        "{}",
        crack_single_xor_cipher_multi_text(input.lines().filter_map(|v| v.ok()))?
    );

    Ok(())
}
