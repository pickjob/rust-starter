use anyhow::{Context, Result};
use flexi_logger::{
    colored_detailed_format, Age, Cleanup, Criterion, Duplicate, FileSpec, LevelFilter,
    LogSpecification, Logger, Naming,
};
use log::info;
use std::error::Error;
use std::str;

fn main() -> Result<(), Box<dyn Error>> {
    setup_logger().context(format!("Create logger failed"))?;
    info!("Hello world!");
    let src = "Hello World! 你好，世界!";
    info!("{}", base64::encode(src));
    info!("{}", base64::encode_config(src, base64::STANDARD));
    info!("{}", base64::encode_config(src, base64::STANDARD_NO_PAD));
    info!("{}", base64::encode_config(src, base64::URL_SAFE));
    info!("{}", base64::encode_config(src, base64::URL_SAFE_NO_PAD));
    let dst = "SGVsbG8gV29ybGQhIOS9oOWlve+8jOS4lueVjCE";
    info!("{}", str::from_utf8(&base64::decode(dst)?)?);
    info!(
        "{}",
        str::from_utf8(&base64::decode_config(dst, base64::STANDARD)?)?
    );
    info!(
        "{}",
        str::from_utf8(&base64::decode_config(dst, base64::STANDARD_NO_PAD)?)?
    );

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
