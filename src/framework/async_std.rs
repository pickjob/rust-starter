use async_std::fs;
use async_std::prelude::*;
use async_std::task;
use log::info;

use crate::common::Result;
use crate::common::Show;

pub(crate) struct AsyncStdExample {}

impl Show for AsyncStdExample {
    fn show() -> Result {
        task::block_on(say_hi());
        task::block_on(list_path("/"))?;
        Ok(())
    }
}

async fn say_hi() {
    info!("async fn say_hi");
}

async fn list_path(path: &str) -> Result {
    let mut dir = fs::read_dir(path).await?;
    while let Some(res) = dir.next().await {
        let entry = res?;
        info!("{}", entry.file_name().to_string_lossy());
    }
    Ok(())
}
