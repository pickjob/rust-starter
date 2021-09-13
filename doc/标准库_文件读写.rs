// 
// 系统文件信息、读写
// 
// std::io
//      prelude:
//           use std::io::prelude::*;
//              pub use super::BufRead;	
//              pub use super::Read;	
//              pub use super::Seek;	
//              pub use super::Write;
//      traits:
//           pub trait Read {
//               // Trait Read 核心方法
//               fn read(&mut self, buf: &mut [u8]) -> Result<usize>;
//               // 各种便捷读取方法
//               fn read_exact(&mut self, buf: &mut [u8]) -> Result<()> { ... } // 至多 buf.length 长度读取
//               fn read_to_end(&mut self, buf: &mut Vec<u8>) -> Result<usize> { ... } // 读取全部
//               fn read_to_string(&mut self, buf: &mut String) -> Result<usize> { ... } // 读取转为String
//               // byte Iterator
//               fn bytes(self) -> Bytes<Self> where Self: Sized, { ... }
//               // 获得自身引用 Adaptor
//               fn by_ref(&mut self) -> &mut Self where Self: Sized, { ... }
//               // 连接两个流 Adaptor
//               fn chain<R: Read>(self, next: R) -> Chain<Self, R> where Self: Sized, { ... }
//               // 至多只能读 limit Adaptor
//               fn take(self, limit: u64) -> Take<Self> where Self: Sized, { ... }
// 
//               fn read_vectored(&mut self, bufs: &mut [IoSliceMut<'_>]) -> Result<usize> { ... }
//               fn is_read_vectored(&self) -> bool { ... }
//               unsafe fn initializer(&self) -> Initializer { ... }
//           }
//           pub trait BufRead: Read {
//               // line Iterator
//               fn lines(self) -> Lines<Self>;
//               // 读取一行
//               fn read_line(&mut self, buf: &mut String) -> Result<usize>;
//               // 按字节切割
//               fn split(self, byte: u8) -> Split<Self>;
// 
//               fn fill_buf(&mut self) -> Result<&[u8]>;
//               fn consume(&mut self, amt: usize);
//               fn read_until(&mut self, byte: u8, buf: &mut Vec<u8>) -> Result<usize>;
//           }
//           pub trait Write {
//               // 写相关操作
//               fn write(&mut self, buf: &[u8]) -> Result<usize>;
//               fn write_all(&mut self, buf: &[u8]) -> Result<()> { ... }
//               fn write_fmt(&mut self, fmt: Arguments<'_>) -> Result<()> { ... }
//               fn flush(&mut self) -> Result<()>;
//               // 获得自身引用
//               fn by_ref(&mut self) -> &mut Self where Self: Sized, { ... }
// 
//               fn write_vectored(&mut self, bufs: &[IoSlice<'_>]) -> Result<usize> { ... }
//               fn write_all_vectored(&mut self, bufs: &mut [IoSlice<'_>]) -> Result<()> { ... }
//               fn is_write_vectored(&self) -> bool { ... }
//           }
//           pub trait Seek {
//               // 查找定位
//               fn seek(&mut self, pos: SeekFrom) -> Result<u64>;
//               // 重置流定位
//               fn rewind(&mut self) -> Result<()> { ... }
//               // 流大小
//               fn stream_len(&mut self) -> Result<u64>;
//               // 流位置
//               fn stream_position(&mut self) -> Result<u64>;
//           }
//      functions:
//           // 当前程序标准输入、输出、异常流
//           std::io::stdin() -> Stdin
//           std::io::stdout() -> Stdout
//           std::io::stderr() -> Stderr
//           // 复制转发流
//           std::io::copy<R: ?Sized, W: ?Sized>(reader: &mut R, writer: &mut W) -> Result<u64> where R: Read, W: Write,
//           // 空Read、Write
//           std::io::empty() -> Empty
//           std::io::sink() -> Sink
//           // 重复读到一字节
//           std::io::repeat(byte: u8) -> Repeat
//      struts:
//           std::io::BufReader<R: Read>
//                pub fn new(inner: R) -> BufReader<R>
//                pub fn with_capacity(capacity: usize, inner: R) -> BufReader<R>
//                pub fn get_ref(&self) -> &R
//                pub fn get_mut(&mut self) -> &mut R
//                  impl<R: Read> BufRead for BufReader<R>
//                  impl<R: Read> Read for BufReader<R>
//                  impl<R: Seek> Seek for BufReader<R>
//           std::io::BufWriter<W: Write>
//                pub fn new(inner: W) -> BufWriter<W>
//                pub fn with_capacity(capacity: usize, inner: W) -> BufWriter<W>
//                pub fn get_ref(&self) -> &W
//                pub fn get_mut(&mut self) -> &mut W
//                  impl<W: Write> Drop for BufWriter<W>
//                  impl<W: Write + Seek> Seek for BufWriter<W>
//                  impl<W: Write> Write for BufWriter<W>
// std::fs
//      std::fs::File:
//          // 创建、打开文件
//          pub fn create<P: AsRef<Path>>(path: P) -> Result<File>
//          pub fn open<P: AsRef<Path>>(path: P) -> Result<File>
//          // 同步信息
//          pub fn sync_data(&self) -> Result<()>
//          pub fn sync_all(&self) -> Result<()>
//          pub fn set_len(&self, size: u64) -> Result<()>
//          pub fn metadata(&self) -> Result<Metadata>
//              impl From<File> for Stdio
//              impl Seek for File
//              impl Seek for &File
//              impl Read for File
//              impl Read for &File
//              impl Write for File
//              impl Write for &File
//      std::fs::OpenOptions:
//          pub fn new() -> Self
//          pub fn create(&mut self, create: bool) -> &mut Self
//          pub fn open<P: AsRef<Path>>(&self, path: P) -> Result<File>
//          pub fn write(&mut self, write: bool) -> &mut Self
//          pub fn append(&mut self, append: bool) -> &mut Self
//          pub fn truncate(&mut self, truncate: bool) -> &mut Self
//      目录操作:
//          std::fs::create_dir<P: AsRef<Path>>(path: P) -> Result<()>
//          std::fs::create_dir_all<P: AsRef<Path>>(path: P) -> Result<()>
//          std::fs::read_dir<P: AsRef<Path>>(path: P) -> Result<ReadDir>
//          std::fs::remove_dir<P: AsRef<Path>>(path: P) -> Result<()>
//          std::fs::remove_dir_all<P: AsRef<Path>>(path: P) -> Result<()>
//      文件操作:
//          // 读、写、复制、重命名、删除
//          std::fs::read<P: AsRef<Path>>(path: P) -> Result<Vec<u8>>
//          std::fs::read_to_string<P: AsRef<Path>>(path: P) -> Result<String>
//          std:fs::write<P: AsRef<Path>, C: AsRef<[u8]>>(path: P, contents: C) -> Result<()>
//          std::fs::copy<P: AsRef<Path>, Q: AsRef<Path>>(from: P, to: Q) -> Result<u64>
//          std::fs::rename<P: AsRef<Path>, Q: AsRef<Path>>(from: P, to: Q) -> Result<()>
//          std::fs::remove_file<P: AsRef<Path>>(path: P) -> Result<()>
// std::path
//       std::path::Path
//          pub fn new<S: AsRef<OsStr> + ?Sized>(s: &S) -> &Path
//          pub fn to_path_buf(&self) -> PathBuf
//          pub fn parent(&self) -> Option<&Path>
//          pub fn ancestors(&self) -> Ancestors<'_>
//          pub fn starts_with<P: AsRef<Path>>(&self, base: P) -> bool
//          pub fn ends_with<P: AsRef<Path>>(&self, child: P) -> bool
//          pub fn display(&self) -> Display<'_>
//          pub fn metadata(&self) -> Result<Metadata>
//          pub fn read_dir(&self) -> Result<ReadDir>
//          pub fn exists(&self) -> bool
//          pub fn extension(&self) -> Option<&OsStr>
//          pub fn is_file(&self) -> bool
//          pub fn is_dir(&self) -> bool
//          pub fn into_path_buf(self: Box<Path>) -> PathBuf
//      std::path::PathBuf
//          pub fn new() -> PathBuf
//          pub fn as_path(&self) -> &Path
//          pub fn clear(&mut self)
//          pub fn pop(&mut self) -> bool
//          pub fn push<P: AsRef<Path>>(&mut self, path: P)
//      std::path::Display
//          impl Debug for Display<'_>
//          impl Display for Display<'_>
