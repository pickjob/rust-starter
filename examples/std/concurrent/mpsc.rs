use log::info;
use std::error::Error;
use std::sync::mpsc::channel;
use std::thread;

fn main() -> Result<(), Box<dyn Error>> {
    match log4rs::init_file("log4rs.yml", Default::default()) {
        Ok(_) => info!("log4rs loading success"),
        _ => unreachable!(),
    };
    let (tx, rx) = channel();
    thread::spawn(move || {
        tx.send(10).unwrap();
    });
    info!("{:#?}", rx.recv().unwrap());
    info!("in the end ...");

    Ok(())
}
