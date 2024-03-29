use anyhow::{Context, Result};
use flexi_logger::{
    colored_detailed_format, Age, Cleanup, Criterion, Duplicate, FileSpec, LevelFilter,
    LogSpecification, Logger, Naming,
};
use log::info;
use std::sync::{Arc, Barrier};
use std::thread;
use std::{error::Error, time::Duration};
/**
 *  Barrier: 栅栏，线程间同步机制
 */
fn main() -> Result<(), Box<dyn Error>> {
    setup_logger().context(format!("Create logger failed"))?;

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

fn setup_logger() -> Result<()> {
    let mut builder = LogSpecification::builder();
    builder.default(LevelFilter::Trace);
    let logger = Logger::with(builder.build());
    logger
        .format(colored_detailed_format)
        .duplicate_to_stdout(Duplicate::Info)
        .log_to_file(
            FileSpec::default()
                .directory("logs")
                .basename("app")
                .suffix("log")
                .suppress_timestamp(),
        )
        .append()
        .rotate(
            Criterion::Age(Age::Day),
            Naming::Timestamps,
            Cleanup::KeepCompressedFiles(7),
        )
        .print_message()
        .start()?;

    Ok(())
}
