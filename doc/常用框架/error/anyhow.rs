// 
// anyhow: 异常处理
//      traits:
//          pub trait Context<T, E>: Sealed {
//              fn context<C>(self, context: C) -> Result<T, Error> where C: Display + Send + Sync + 'static;
//              fn with_context<C, F>(self, f: F) -> Result<T, Error> where C: Display + Send + Sync + 'static, F: FnOnce() -> C;
//          }
//              impl<T, E> Context<T, E> for Result<T, E> where E: StdError + Send + Sync + 'static, 
//      type:
//          anyhow::Result
//              pub type Result<T, E = Error> = Result<T, E>;
// 
