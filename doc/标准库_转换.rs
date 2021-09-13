
// 
// Convert 
//      AsRef / AsMut: auto-dereference if the inner type is a reference
//      From<U> for T <==> Into<T> for U
//      TryFrom<U> for T <==> TryInto<T> for U 
// traits:
//      std::convert::AsRef: cheap reference-to-reference conversions
//      std::convert::AsMut: cheap mutable-to-mutable conversions
//      std::convert::From / TryFrom: consuming value-to-value conversions
//      std::convert::Into / TryInto: consuming value-to-value conversions to types outside the current crate
//              pub trait AsMut<T> where
//                               T: ?Sized, 
//              {
//                  pub fn as_mut(&mut self) -> &mut T;
//              }
//              pub trait AsRef<T> where
//                              T: ?Sized, 
//              {
//                  pub fn as_ref(&self) -> &T;
//              }
//              pub trait From<T> {
//                  pub fn from(T) -> Self;
//              }
//              pub trait Into<T> {
//                  pub fn into(self) -> T;
//              }
//              pub trait TryFrom<T> {
//                  type Error;
//                  pub fn try_from(value: T) -> Result<Self, Self::Error>;
//              }
//              pub trait TryInto<T> {
//                  type Error;
//                  pub fn try_into(self) -> Result<T, Self::Error>;
//              }
// std::str::FromStr
//      pub trait FromStr {
//          type Err;
//          pub fn from_str(s: &str) -> Result<Self, Self::Err>;
//      }
// std::string::ToString
//      pub trait ToString {
//          pub fn to_string(&self) -> String;
//      }