use anyhow::{Context, Result};
use flexi_logger::{
    colored_detailed_format, Age, Cleanup, Criterion, Duplicate, FileSpec, LevelFilter,
    LogSpecification, Logger, Naming,
};
use log::info;
use std::error::Error;
use ureq::serde_json::Value;

fn main() -> Result<(), Box<dyn Error>> {
    setup_logger().context(format!("Create logger failed"))?;
    get_request()?;
    post_form_request()?;
    post_json_request()?;

    Ok(())
}

fn get_request() -> Result<(), Box<dyn Error>> {
    let url = "http://httpbin.org/get";
    let resp = ureq::get(url)
        .set("X-My-Header", "Secret")
        .query("param", "我们")
        .call()?
        .into_string()?;
    info!("url: {:#?}, resp: {:#?}", url, resp);

    Ok(())
}

fn post_form_request() -> Result<(), Box<dyn Error>> {
    let url = "https://httpbin.org/post";
    let resp = ureq::post(url)
        .set("X-My-Header", "Secret")
        .send_form(&[("foo", "bar"), ("foo", "bar2")])?
        .into_string()?;
    info!("url: {:#?}, resp: {:#?}", url, resp);

    Ok(())
}

fn post_json_request() -> Result<(), Box<dyn Error>> {
    let url = "https://httpbin.org/post";
    let resp: Value = ureq::post(url)
        .set("X-My-Header", "Secret")
        .send_string(r#"{"name": "martin","rust": true}"#)?
        .into_json()?;
    info!("url: {:#?}, resp: {:#?}", url, resp["json"]);

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
