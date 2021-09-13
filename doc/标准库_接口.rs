// 
// std::clone::Clone
//      pub trait Clone {
//          pub fn clone(&self) -> Self;
//          pub fn clone_from(&mut self, source: &Self);
//      }
// 
// std::hash::Hash
//      pub trait Hash {
//          pub fn hash<H>(&self, state: &mut H) where H: Hasher;
//          pub fn hash_slice<H>(data: &[Self], state: &mut H) where H: Hasher,
//      }
// 
// std::default:Default
//      pub trait Default {
//          pub fn default() -> Self;
//      }
// 
// std::borrow
//      std::borrow::Borrow
//          pub trait Borrow<Borrowed> where Borrowed: ?Sized, {
//              pub fn borrow(&self) -> &Borrowed;
//          }
//      std::borrow::BorrowMut
//          pub trait BorrowMut<Borrowed>: Borrow<Borrowed> where Borrowed: ?Sized, {
//              pub fn borrow_mut(&mut self) -> &mut Borrowed;
//          }
//      std::borrow::ToOwned
//          pub trait ToOwned {
//              type Owned: Borrow<Self>;
//              pub fn to_owned(&self) -> Self::Owned;
//              pub fn clone_into(&self, target: &mut Self::Owned);
//          }
// 
// marker: 标识接口
//      编译器可知大小:
//          pub trait Sized { }
//      Copy语义标识:
//          pub trait Copy: Clone {}
//      Send语义标识(Move语义, 复制原值，传进闭包):
//          pub unsafe auto trait Send {}
//      Sync语义标识(同步语义, 线程安全):
//          pub unsafe auto trait Sync {}
//      Unpin语义标识(UnPin语义, 可移动):
//          pub auto trait Unpin { }
