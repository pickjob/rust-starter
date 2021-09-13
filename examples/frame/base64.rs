use anyhow::Result;
use std::str;

fn main() -> Result<()> {
    let src = "Hello World! 你好，世界!";
    println!("{}", base64::encode(src));
    println!("{}", base64::encode_config(src, base64::STANDARD));
    println!("{}", base64::encode_config(src, base64::STANDARD_NO_PAD));
    println!("{}", base64::encode_config(src, base64::URL_SAFE));
    println!("{}", base64::encode_config(src, base64::URL_SAFE_NO_PAD));

    let dst = "SGVsbG8gV29ybGQhIOS9oOWlve+8jOS4lueVjCE";
    println!("{}", str::from_utf8(&base64::decode(dst)?)?);
    println!("{}", str::from_utf8(&base64::decode_config(dst, base64::STANDARD)?)?);
    println!("{}", str::from_utf8(&base64::decode_config(dst, base64::STANDARD_NO_PAD)?)?);
    // println!("{}", str::from_utf8(&base64::decode_config(dst, base64::URL_SAFE)?)?);
    // println!("{}", str::from_utf8(&base64::decode_config(dst, base64::URL_SAFE_NO_PAD)?)?);

    Ok(())
}
