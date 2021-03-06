use log::info;
use std::error::Error;
use ureq::SerdeValue;

/**
 *  Ureq: Http框架
 *      .get() / post() / put() / delete() -> Request
 *      .agent(): 维护不同请求间state
 *  Request:
 *      .call(): 请求无body
 *      .send_string(): body为string
 *      .send_bytes(): body为bytes
 *      .send_form(): application/x-www-form-urlencoded
 *      .send(): body未知, chunked发送
*/
fn main() -> Result<(), Box<dyn Error>> {
    match log4rs::init_file("log4rs.yml", Default::default()) {
        Ok(_) => info!("log4rs loading success"),
        _ => unreachable!(),
    };
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
    let resp: SerdeValue = ureq::post(url)
        .set("X-My-Header", "Secret")
        .send_string(r#"{"name": "martin","rust": true}"#)?
        .into_json()?;
    info!("url: {:#?}, resp: {:#?}", url, resp["json"]);

    Ok(())
}
