use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::error::Error;

fn main() -> anyhow::Result<(), Box<dyn Error>> {
    json_example()?;
    yaml_example()?;
    qs_example()?;

    Ok(())
}

#[derive(Debug, Serialize, Deserialize)]
struct Person {
    name: String,
    age: u8,
    phones: Vec<String>,
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

    println!("Result: {:#?}", person);
    println!("RawResult: {}", serde_json::to_string(&person)?);

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

    println!("Result: {:#?}", person);
    println!("RawResult: {}", serde_yaml::to_string(&person)?);

    Ok(())
}

fn qs_example() -> Result<()> {
    let data = "name=John+Doe&age=43&phones[0]=%2B44+1234567&phones[1]=%2B44+2345678";

    let person: Person = serde_qs::from_str(data)?;

    println!("ParsedResult: {:#?}", person);
    println!("RawResult: {}", serde_qs::to_string(&person)?);

    Ok(())
}
