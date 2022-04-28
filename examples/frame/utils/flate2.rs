use anyhow::{Context, Result};
use flate2::{read, Compression, GzBuilder};
use flexi_logger::{
    colored_detailed_format, Age, Cleanup, Criterion, Duplicate, FileSpec, LevelFilter,
    LogSpecification, Logger, Naming,
};
use log::info;
use std::{
    error::Error,
    fs::File,
    io::{Read, Write},
};

fn main() -> Result<(), Box<dyn Error>> {
    setup_logger().context(format!("Create logger failed"))?;
    let mut encoder = GzBuilder::new()
        .filename("hello_world.txt")
        .comment("just show comment")
        .write(File::create("logs/hello_world.gz")?, Compression::default());
    encoder.write_all("Hello, world!".as_bytes())?;
    encoder.finish()?;
    let mut decoder = read::GzDecoder::new(File::open("logs/hello_world.gz")?);
    let mut info = String::new();
    decoder.read_to_string(&mut info)?;
    info!("{}", info);

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
