use anyhow::{Context, Result};
use chrono::offset::{Local, Utc};
use flexi_logger::{
    colored_detailed_format, Age, Cleanup, Criterion, Duplicate, FileSpec, LevelFilter,
    LogSpecification, Logger, Naming,
};
use log::info;

fn main() -> Result<()> {
    setup_logger().context(format!("Create logger failed"))?;
    let date_format = "%Y-%m-%d";
    let utc_today = Utc::today();
    info!("{}", utc_today.format(date_format));
    let local_today = Local::today();
    info!("{}", local_today.format(date_format));

    let time_format = "%Y-%m-%d %H:%M:%S";
    let utc_now = Utc::now();
    info!("{}", utc_now.format(time_format));
    let local_now = Local::now();
    info!("{}", local_now.format(time_format));

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
