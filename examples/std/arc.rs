use log::info;
use std::sync::Arc;
use std::thread;
use std::{error::Error, time::Duration};

/**
 *  Arc: 实现线程间安全共享数据
 */
fn main() -> Result<(), Box<dyn Error>> {
    match log4rs::init_file("log4rs.yml", Default::default()) {
        Ok(_) => info!("log4rs loading success"),
        _ => unreachable!(),
    };
    let five = Arc::new(5);
    for _ in 0..10 {
        let five = five.clone();
        thread::spawn(move || {
            info!("{:#?}", five);
        });
    }
    thread::sleep(Duration::new(5, 0));
    info!("in the end ...");

    Ok(())
}
