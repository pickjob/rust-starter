use anyhow::{Context, Result};
use std::error::Error;
use std::fs;

fn main() -> Result<(), Box<dyn Error>> {
    let path = "not_exists_file.txt";
    // .context 补充错误信息, 使用字符串
    fs::read(path).context(format!("Failed to read files, file path: {}", path))?;
    // 。with_context 补充错误信息, 使用函数
    // fs::read(path).with_context(|| format!("Failed to read instrs from {}", path))?;

    Ok(())
}
