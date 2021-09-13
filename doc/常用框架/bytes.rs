// 
// bytes
//      traits:
//          bytes::buf::Buf: Read bytes from a buffer
//              pub trait Buf {
//                  pub fn reader(self) -> Reader<Self>; // Reader impl std::io::Read
//                  get_xxx
//              }
//          bytes::buf::BufMut: Write bytes to a buffer
//              pub unsafe trait BufMut {
//                  pub fn writer(self) -> Writer<Self>; // Writer impl std::io::Write
//                  put_xxx
//              }
//      structs:
//          bytes::Bytes: Send + Sync
//              pub const fn new() -> Bytes
//              pub const fn from_static(bytes: &'static [u8]) -> Bytes
//              pub fn len(&self) -> usize
//              pub fn is_empty(&self) -> bool
//              pub fn slice(&self, range: impl RangeBounds<usize>) -> Bytes
//              impl From<Vec<u8>> for Bytes
//              impl Deref<Target = [u8]> for Bytes
//              impl Buf for Bytes
//          bytes::BytesMut: Send + Sync
//              impl Deref<Target = [u8]> for BytesMut
//              impl Buf for BytesMut
//              impl BufMut for BytesMut
