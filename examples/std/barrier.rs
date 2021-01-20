use log::info;
use std::sync::{Arc, Barrier};
use std::thread;
use std::{error::Error, time::Duration};

/**
 *  Barrier: 栅栏，线程间同步机制
 */
fn main() -> Result<(), Box<dyn Error>> {
    match log4rs::init_file("log4rs.yml", Default::default()) {
        Ok(_) => info!("log4rs loading success"),
        _ => unreachable!(),
    };
    let barrier = Arc::new(Barrier::new(10));
    for _ in 0..10 {
        let c = Arc::clone(&barrier);
        thread::spawn(move || {
            info!("before wait");
            c.wait();
            info!("after wait");
        });
    }
    thread::sleep(Duration::new(5, 0));
    info!("in the end ...");

    Ok(())
}
