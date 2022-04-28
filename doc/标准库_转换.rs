
// 
// Convert 
//      AsRef / AsMut
//          as_*: Free
//      From<U> for T <==> Into<T> for U
//          into_*: Variable
//      TryFrom<U> for T <==> TryInto<T> for U 
//      ToOwned
//          to_*: Expensive
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
// std::str::FromStr: "text".parse::<T>(): where T impl FromStr
//      pub trait FromStr {
//          type Err;
//          pub fn from_str(s: &str) -> Result<Self, Self::Err>;
//      }
// std::string::ToString
//      pub trait ToString {
//          pub fn to_string(&self) -> String;
//      }
// usages:
//      str:
//          str.as_bytes(): str => &[u8]
//          str.as_bytes_mut(): str => &mut [u8]
//          str.to_owned(): str => String
//          str::from_utf8(): &[u8] => &str
//          str::from_utf8_mut(): &mut [u8] => &mut str
//      String:
//          String.as_bytes(): String => &[u8]
//          String.as_str(): String => &str
//          String.as_mut_str(): String => &mut str
//          String::as_ref(): String => [u8]
//          String.into_bytes(): String => Vec<u8>
//          String.borrow(): String => &str
//          String.borrow_mut(&mut self): String =>&mut str
//          String::from_utf8_lossy(): &[u8] => Cow<'_, str>
//      Path:
//          Path.to_owned(): Path => PathBuf
