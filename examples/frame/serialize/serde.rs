use anyhow::{Context, Result};
use flexi_logger::{
    colored_detailed_format, Age, Cleanup, Criterion, Duplicate, FileSpec, LevelFilter,
    LogSpecification, Logger, Naming,
};
use log::info;
use serde::{Deserialize, Serialize};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    setup_logger().context(format!("Create logger failed"))?;
    json_example()?;
    yaml_example()?;
    qs_example()?;

    Ok(())
}

fn json_example() -> Result<()> {
    let data = r#"
        {
            "name": "John Doe",
            "age": 43,
            "phones": [
                "+44 1234567",
                "+44 2345678"
            ]
        }"#;
    let person: Person = serde_json::from_str(data)?;
    info!("Result: {:#?}", person);
    info!("RawResult: {}", serde_json::to_string(&person)?);

    Ok(())
}

fn yaml_example() -> Result<()> {
    let data = r#"
        name: "John Doe"
        age: 43
        phones:
            - "+44 1234567"
            - "+44 2345678"
        "#;
    let person: Person = serde_yaml::from_str(data)?;
    info!("Result: {:#?}", person);
    info!("RawResult: {}", serde_yaml::to_string(&person)?);

    Ok(())
}

fn qs_example() -> Result<()> {
    let data = "name=John+Doe&age=43&phones[0]=%2B44+1234567&phones[1]=%2B44+2345678";
    let person: Person = serde_qs::from_str(data)?;
    info!("ParsedResult: {:#?}", person);
    info!("RawResult: {}", serde_qs::to_string(&person)?);

    Ok(())
}

#[derive(Debug, Serialize, Deserialize)]
struct Person {
    name: String,
    age: u8,
    phones: Vec<String>,
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
