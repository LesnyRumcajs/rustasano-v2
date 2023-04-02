use basics::single_xor_cipher::crack_single_xor_cipher;

fn main() -> anyhow::Result<()> {
    let input = std::env::args()
        .nth(1)
        .ok_or_else(|| anyhow::anyhow!("No input provided!"))?;

    println!("{}", crack_single_xor_cipher(input)?.0);

    Ok(())
}
