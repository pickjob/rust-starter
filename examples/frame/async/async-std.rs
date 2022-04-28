use anyhow::Result;
use async_std::fs;
use async_std::prelude::*;
use std::error::Error;

#[async_std::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let path = "C:\\code";
    let mut dir = fs::read_dir(&path).await?;

    while let Some(res) = dir.next().await {
        let entry = res?;
        println!("{}", entry.file_name().to_string_lossy());
    }

    Ok(())
}
