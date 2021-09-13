// 
// std::result::Result
//     pub enum Result<T, E> {
//          Ok(T),
//          Err(E),
//      }
//          pub fn unwrap_or(self, default: T) -> T
//          pub fn unwrap_or_else<F>(self, op: F) -> T where F: FnOnce(E) -> T, 
// 
// std::option::Option
//      pub enum Option<T> {
//         None,
//         Some(T),
//      }
//          pub fn unwrap_or(self, default: T) -> T
//          pub fn unwrap_or_else<F>(self, f: F) -> T where F: FnOnce() -> T, 
// 
// std::borrow.Cow: Copy On Write Pointer
//      pub enum Cow<'a, B> where B: 'a + ToOwned + ?Sized, {
//          Borrowed(&'a B),
//          Owned(<B as ToOwned>::Owned),
//      }
//      
