use std::fs;
use anyhow::{Context, Result};

fn main() -> Result<()> {
    let path = "not_exists_file.txt";
    fs::read(path)
        .context(format!("Failed to read instrs from {}", path))?;

    // fs::read(path)
    //     .with_context(|| format!("Failed to read instrs from {}", path))?;

    Ok(())
}
