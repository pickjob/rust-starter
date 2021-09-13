// 
// primitive type
// 
//      bool: true false
//      char:
//           pub fn from_u32(i: u32) -> Option<char>
//           is_xxx to_xx
//      i8 i16 i32 i64: signed number
//      u8 u16 u32 u64: unsigned number
//      isize: sigend pointer-size
//      usize: unsigned pointer-size
//           from_str_radix
//           swap_bytes
//           from_be / from_le / from_be_bytes / from_le_bytes
//           to_be / to_le / to_be_bytes / to_le_bytes
//      fn: function pointer
//      array: [T; N], 指定长度
//           let array: [i32; 3] = [0; 3];
//           // This iterates by reference:
//           for item in array.iter().enumerate() {
//               let (i, x): (usize, &i32) = item;
//               println!("array[{}] = {}", i, x);
//           }
//           // This iterates by value:
//           for item in array.into_iter().enumerate() {
//               let (i, x): (usize, i32) = item;
//               println!("array[{}] = {}", i, x);
//           }
//           impl<T, A, const N: usize> TryFrom<Vec<T, A>> for [T; N]
//      slice: &[T] &mut [T], 长度未明确
//           get / len / is_empty
//           iter / iter_mut
//           first / last
//           start_with / end_with
//      str: string slice UTF-8
//           get / len / is_empty
//           first / last
//           start_with / end_with
//           as_bytes / as_bytes_mut
//           trim_matches
//      tuple: (T, U, ...)
//      unit: ()
// prelude
//      std::prelude::v1
//          std::marker::{Copy, Send, Sized, Sync, Unpin}
//          std::ops::{Drop, Fn, FnMut, FnOnce}
//          std::mem::drop
//          std::boxed::Box
//          std::borrow::ToOwned
//          std::clone::Clone
//          std::cmp::{PartialEq, PartialOrd, Eq, Ord}
//          std::convert::{AsRef, AsMut, Into, From}
//          std::default::Default
//          std::iter::{Iterator, Extend, IntoIterator, DoubleEndedIterator, ExactSizeIterator}
//          std::option::Option::{self, Some, None}
//          std::result::Result::{self, Ok, Err}
//          std::string::{String, ToString}
//          std::vec::Vec
