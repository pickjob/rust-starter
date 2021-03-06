use log::info;
use std::env::args;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    match log4rs::init_file("log4rs.yml", Default::default()) {
        Ok(_) => info!("log4rs loading success"),
        _ => unreachable!(),
    };
    let path = match args().nth(0) {
        Some(path) => path,
        None => "Unknow path".to_string(),
    };
    info!("Hello World! Your executable file path: {:#?}", path);

    Ok(())
}
