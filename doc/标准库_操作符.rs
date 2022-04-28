// 
// std::cmp: 比较操作符
//      pub trait Eq: PartialEq<Self> { }
//      pub trait PartialEq<Rhs = Self>  where Rhs: ?Sized, {
//          pub fn eq(&self, other: &Rhs) -> bool;
//          pub fn ne(&self, other: &Rhs) -> bool { ... }
//      }
//      pub trait Ord: Eq + PartialOrd<Self> {
//          pub fn cmp(&self, other: &Self) -> Ordering;
//          pub fn max(self, other: Self) -> Self { ... }
//          pub fn min(self, other: Self) -> Self { ... }
//          pub fn clamp(self, min: Self, max: Self) -> Self { ... }
//      }
//      pub trait PartialOrd<Rhs = Self>: PartialEq<Rhs> where Rhs: ?Sized, {
//          pub fn partial_cmp(&self, other: &Rhs) -> Option<Ordering>;
//          pub fn lt(&self, other: &Rhs) -> bool { ... }
//          pub fn le(&self, other: &Rhs) -> bool { ... }
//          pub fn gt(&self, other: &Rhs) -> bool { ... }
//          pub fn ge(&self, other: &Rhs) -> bool { ... }
//      }
// 
// std::op
//      Add: +
//      AddAssign: +=
//      BitAnd: &
//      BitAndAssign: &=
//      BitOr: |
//      BitOrAssign: |=
//      BitXor: ^
//      BitXorAssign: ^=
//      Div: /
//      DivAssign: /=
//      Mul: *
//      MulAssign: *=
//      Neg: -
//      Not: !
//      RangeBounds: .., a.., ..b, ..=c, d..e, or f..=g
//      Rem: %
//      RemAssign: %=
//      Shl: <<
//      ShlAssign: <<=
//      Shr: >>
//      ShrAssign: >>=
//      Sub: -
//      SubAssign: -=
//      Index: container[index]
//      IndexMut: container[index]
//      Drop
//          pub trait Drop {
//              pub fn drop(&mut self);
//          } 
//      Deref: *v
//          pub trait Deref {
//              type Target: ?Sized;
//              pub fn deref(&self) -> &Self::Target;
//          }
//              impl<T, A> Deref for Box<T, A> where T: ?Sized, A: Allocator, -> &T
//              impl<T> Deref for Rc<T> where T: ?Sized, -> &T
//              impl<T> Deref for Arc<T> where T: ?Sized, -> &T
//              impl<T, A> Deref for Vec<T, A> where A: Allocator, -> &[T] 
//              impl<T: ?Sized> Deref for MutexGuard<'_, T> -> &T
//              impl Deref for String -> &str
//      DerefMut: *v = 1 (*DerefMut::deref_mut(&mut x))
//          pub trait DerefMut: Deref {
//              pub fn deref_mut(&mut self) -> &mut Self::Target;
//          }
//              impl<T, A> DerefMut for Box<T, A> where T: ?Sized, A: Allocator,  -> &mut T
//              impl<T, A> DerefMut for Vec<T, A> where A: Allocator,  -> &mut [T]
//              impl<T: ?Sized> DerefMut for MutexGuard<'_, T> -> &mut T
//              impl<T: ?Sized> DerefMut for RwLockWriteGuard<'_, T> -> &mut T
//              impl DerefMut for String -> -> &mut str
// 
// std::ops::FnOnce: 捕获环境自由变量所有权 (self)
//      pub trait FnOnce<Args> {
//          type Output;
//          fn call_once(self, args: Args) -> Self::Output;
//      }
// std::ops::FnMut: 可变方式借用环境自由变量 (&mut T)
//      pub trait FnMut<Args>: FnOnce<Args> {
//         fn call_once(self, args: Args) -> Self::Output;
//      }
// std::ops::Fn: 不可变方式借用环境自由变量 (&T)
//      pub trait Fn<Args>: FnMut<Args> {
//          fn call_once(self, args: Args) -> Self::Output;
//      }
