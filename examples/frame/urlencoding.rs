use anyhow::Result;

fn main() -> Result<()> {
    let src = "Hello World! 你好，世界!";
    println!("{}", urlencoding::encode(src));

    let dst = "Hello%20World%21%20%E4%BD%A0%E5%A5%BD%EF%BC%8C%E4%B8%96%E7%95%8C%21";
    println!("{}", urlencoding::decode(dst)?);

    Ok(())
}
