// 
// ureq: Http请求框架
//      functions:
//          // 构建请求
//          ureq::head(path: &str) -> Request
//          ureq::get(path: &str) -> Request
//          ureq::post(path: &str) -> Request
//          ureq::put(path: &str) -> Request
//          ureq::delete(path: &str) -> Request
//          ureq::request(method: &str, path: &str) -> Request
//          ureq::request_url(method: &str, url: &Url) -> Request
// 
//          // 维护不同请求间state
//          ureq::agent() -> Agent
//          ureq::builder() -> AgentBuilder
//      structs:
//          ureq::Agent
//              // 请求相关
//              pub fn head(&self, path: &str) -> Request
//              pub fn get(&self, path: &str) -> Request
//              pub fn post(&self, path: &str) -> Request
//              pub fn put(&self, path: &str) -> Request
//              pub fn delete(&self, path: &str) -> Request
//              pub fn request(&self, method: &str, path: &str) -> Request
//              // 其他操作
//              pub fn cookie_store(&self) -> CookieStoreGuard<'_>
//          ureq::Request
//              // 请求头
//              pub fn set(self, header: &str, value: &str) -> Self
//              pub fn header(&self, name: &str) -> Option<&str>
//              pub fn header_names(&self) -> Vec<String>
//              pub fn has(&self, name: &str) -> bool
//              // 请求url参数
//              pub fn query(self, param: &str, value: &str) -> Self
//              // 请求body相关
//              pub fn call(self) -> Result<Response, Error>: no body
//              pub fn send_form(self, data: &[(&str, &str)]) -> Result<Response, Error>: application/x-www-form-urlencoded
//              pub fn send_json(self, data: impl Serialize) -> Result<Response, Error>: 发送JSON数据
//              pub fn send_string(self, data: &str) -> Result<Response, Error>: body为string
//              pub fn send_bytes(self, data: &[u8]) -> Result<Response, Error>: body为bytes
//              pub fn send(self, reader: impl Read) -> Result<Response, Error>: body未知, chunked发送
//          ureq::Response
//              // 响应头
//              pub fn http_version(&self) -> &str
//              pub fn status(&self) -> u16
//              pub fn status_text(&self) -> &str
//              pub fn get_url(&self) -> &str
//              pub fn headers_names(&self) -> Vec<String>
//              pub fn has(&self, name: &str) -> bool
//              pub fn all(&self, name: &str) -> Vec<&str>
//              pub fn content_type(&self) -> &str
//              pub fn charset(&self) -> &str
//              // 消费响应body
//              pub fn into_reader(self) -> impl Read + Send
//              pub fn into_string(self) -> Result<String>
//              pub fn into_json<T: DeserializeOwned>(self) -> Result<T>
