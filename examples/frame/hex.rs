use anyhow::Result;
use std::str;

fn main() -> Result<()> {
    let src = "Hello World! 你好，世界!";
    println!("{}", hex::encode(src));

    let dst = "48656c6c6f20776f726c6421";
    println!("{}", str::from_utf8(&hex::decode(dst)?)?);

    Ok(())
}
