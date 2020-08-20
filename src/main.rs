mod common;
mod framework;

use common::Result;
use common::Show as _;
use framework::async_std::AsyncStdExample;
use framework::rand::RandExample;
use framework::ureq::UreqExample;

use log::info;
fn main() -> Result {
    log4rs::init_file("log4rs.yml", Default::default()).unwrap();
    info!("hello world");
    RandExample::show()?;
    AsyncStdExample::show()?;
    UreqExample::show()?;
    Ok(())
}
