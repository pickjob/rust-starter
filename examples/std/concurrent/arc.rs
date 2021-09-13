use anyhow::Result;
use std::sync::Arc;
use std::thread;
use std::{error::Error, time::Duration};

/**
 *  Arc: 实现线程间安全共享数据
 */
fn main() -> Result<(), Box<dyn Error>> {
    let five = Arc::new(5);
    for _ in 0..10 {
        let five = five.clone();
        thread::spawn(move || {
            println!("{:#?}", five);
        });
    }
    thread::sleep(Duration::new(5, 0));
    println!("in the end ...");

    Ok(())
}
