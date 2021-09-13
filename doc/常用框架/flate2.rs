// 
// flate2: compression/decompression library
//      flate2::GzBuilder
//          pub fn new() -> GzBuilder
//          pub fn filename<T: Into<Vec<u8>>>(self, filename: T) -> GzBuilder
//          pub fn comment<T: Into<Vec<u8>>>(self, comment: T) -> GzBuilder
// 
//          pub fn write<W: Write>(self, w: W, lvl: Compression) -> GzEncoder<W>
//          pub fn read<R: Read>(self, r: R, lvl: Compression) -> GzEncoder<R>
//          pub fn buf_read<R>(self, r: R, lvl: Compression) -> GzEncoder<R>
//      read: 输入流编码、解码
//          flate2::read::GzDecoder
//              impl<R: Read> Read for GzDecoder<R>
//              impl<R: Read + Write> Write for GzDecoder<R>
//          flate2::read::GzEncoder
//              impl<R: Read> Read for GzEncoder<R>
//              impl<R: Read + Write> Write for GzEncoder<R>
//      write: 输出流解码、编码
//          flate2::write::GzDecoder<W: Write>
//              impl<W: Read + Write> Read for GzDecoder<W>
//              impl<W: Write> Write for GzDecoder<W>
//          flate2::write::GzEncoder<W: Write>
//              impl<W: Write> Write for GzEncoder<W>
//              impl<R: Read + Write> Read for GzEncoder<R>
//      结束编码、解码:
//          pub fn finish(self) -> Result<W>
