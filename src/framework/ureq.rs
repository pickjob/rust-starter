use crate::common::{Result, Show};
use async_std::task;
use futures::join;
use log::info;
pub(crate) struct UreqExample {}

impl Show for UreqExample {
    fn show() -> Result {
        task::block_on(task_all())?;
        Ok(())
    }
}

async fn task_all() -> Result {
    let _ = join!(get_request(), post_request(), post_form_request());
    Ok(())
}

async fn get_request() -> Result {
    let url = "http://httpbin.org/get";
    let resp = ureq::get(url)
        .set("X-My-Header", "Secret")
        .query("param", "我们")
        .call();
    info!("url: {}, resp: {}", url, resp.into_string()?);
    Ok(())
}

async fn post_request() -> Result {
    let url = "https://httpbin.org/post";
    let resp = ureq::post(url)
        .set("X-My-Header", "Secret")
        .send_string(r#"{"name": "martin","rust": true}"#);
    info!("url: {}, resp: {}", url, resp.into_string()?);
    Ok(())
}

async fn post_form_request() -> Result {
    let url = "https://httpbin.org/post";
    let resp = ureq::post(url)
        .set("X-My-Header", "Secret")
        .send_form(&[("foo", "bar"), ("foo", "bar2")]);
    info!("url: {}, resp: {}", url, resp.into_string()?);
    Ok(())
}
