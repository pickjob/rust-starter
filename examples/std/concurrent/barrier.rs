use anyhow::Result;
use std::sync::{Arc, Barrier};
use std::thread;
use std::{error::Error, time::Duration};

/**
 *  Barrier: 栅栏，线程间同步机制
 */
fn main() -> Result<(), Box<dyn Error>> {

    let barrier = Arc::new(Barrier::new(10));
    for _ in 0..10 {
        let c = Arc::clone(&barrier);
        thread::spawn(move || {
            println!("before wait");
            c.wait();
            println!("after wait");
        });
    }
    thread::sleep(Duration::new(5, 0));
    println!("in the end ...");

    Ok(())
}
