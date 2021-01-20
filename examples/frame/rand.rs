use log::info;
use rand::Rng;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    match log4rs::init_file("log4rs.yml", Default::default()) {
        Ok(_) => info!("log4rs loading success"),
        _ => unreachable!(),
    };
    let mut rng = rand::thread_rng();
    let n1: u8 = rng.gen();
    info!("Random u8: {:#?}", n1);
    info!("Random u32: {:#?}", rng.gen::<u32>());
    info!("Random i32: {:#?}", rng.gen::<i32>());
    info!("Random Range(0, 10): {:#?}", rng.gen_range(0..10));

    Ok(())
}
