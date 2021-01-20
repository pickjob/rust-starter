use log::info;
use std::error::Error;

fn accept<F>(f: F)
where
    F: FnOnce(&str),
{
    let s = "Hello world";
    f(s);
}

fn main() -> Result<(), Box<dyn Error>> {
    match log4rs::init_file("log4rs.yml", Default::default()) {
        Ok(_) => info!("log4rs loading success"),
        _ => unreachable!(),
    };
    let acc = |x: &str| {
        info!("{:#?}", x);
    };
    accept(acc);

    Ok(())
}
