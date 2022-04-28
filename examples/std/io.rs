use anyhow::{Context, Result};
use flexi_logger::{
    colored_detailed_format, Age, Cleanup, Criterion, Duplicate, FileSpec, LevelFilter,
    LogSpecification, Logger, Naming,
};
use log::info;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader, Write};

fn main() -> Result<(), Box<dyn Error>> {
    setup_logger().context(format!("Create logger failed"))?;
    let mut file = File::create("logs/hello_world.txt")?;
    file.write_all("Hello World!".as_bytes())?;
    file.flush()?;
    std::mem::drop(file);

    let reader = BufReader::new(File::open("logs/hello_world.txt")?);
    for l in reader.lines() {
        info!("{}", l?);
    }

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
