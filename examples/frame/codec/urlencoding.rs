use anyhow::{Context, Result};
use flexi_logger::{
    colored_detailed_format, Age, Cleanup, Criterion, Duplicate, FileSpec, LevelFilter,
    LogSpecification, Logger, Naming,
};
use log::info;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    setup_logger().context(format!("Create logger failed"))?;
    let src = "Hello World! 你好，世界!";
    info!("{}", urlencoding::encode(src));

    let dst = "Hello%20World%21%20%E4%BD%A0%E5%A5%BD%EF%BC%8C%E4%B8%96%E7%95%8C%21";
    info!("{}", urlencoding::decode(dst)?);

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
