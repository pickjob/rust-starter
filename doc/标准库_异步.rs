// 
// std::pin::Pin
//      pub const fn new(pointer: P) -> Pin<P>
//      pub const fn into_inner(pin: Pin<P>) -> P
//      impl<P> Deref for Pin<P> where P: Deref, 
//      impl<P> DerefMut for Pin<P>
// std::future
//      Traits:
//          pub trait Future {
//              type Output;
//              fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output>;
//      }
//      structs:
//          std::future::Ready
//              impl<T> Future for Ready<T>
//          std::future::Pending
//      function:
//          pub fn ready<T>(t: T) -> Ready<T>
//          pub fn pending<T>() -> Pending<T>
// futures::
//      macros:
//          futures::join!: 等待所有异步任务结束
//
// 异步系统:
//      异步函数:
//          async fn do_something() {
//              // something
//          }
//          .wait: 异步函数阻塞
//      执行器: 异步任务执行环境
//      Pin<Box<T>>
//          std::marker::PhantomPinned: 不可Pin
