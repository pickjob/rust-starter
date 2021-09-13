use anyhow::Result;
use log::info;
use std::error::Error;



fn main() -> Result<(), Box<dyn Error>> {
    let acc = |x: &str| {
        info!("{:#?}", x);
    };
    accept(acc);

    Ok(())
}

fn accept<F>(f: F) 
    where F: FnOnce(&str), {
    let s = "Hello world";
    f(s);
}
