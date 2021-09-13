use anyhow::Result;
use std::error::Error;
use ureq::SerdeValue;

fn main() -> Result<(), Box<dyn Error>> {
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
    println!("url: {:#?}, resp: {:#?}", url, resp);

    Ok(())
}

fn post_form_request() -> Result<(), Box<dyn Error>> {
    let url = "https://httpbin.org/post";
    let resp = ureq::post(url)
        .set("X-My-Header", "Secret")
        .send_form(&[("foo", "bar"), ("foo", "bar2")])?
        .into_string()?;
    println!("url: {:#?}, resp: {:#?}", url, resp);

    Ok(())
}

fn post_json_request() -> Result<(), Box<dyn Error>> {
    let url = "https://httpbin.org/post";
    let resp: SerdeValue = ureq::post(url)
        .set("X-My-Header", "Secret")
        .send_string(r#"{"name": "martin","rust": true}"#)?
        .into_json()?;
    println!("url: {:#?}, resp: {:#?}", url, resp["json"]);

    Ok(())
}
