use anyhow::Result;
use std::error::Error;
use std::sync::mpsc::channel;
use std::thread;

fn main() -> Result<(), Box<dyn Error>> {
    let (tx, rx) = channel();
    thread::spawn(move || {
        tx.send(10).unwrap();
    });
    println!("{:#?}", rx.recv().unwrap());
    println!("in the end ...");

    Ok(())
}
