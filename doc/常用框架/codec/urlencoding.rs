// 
// urlencoding
//      functions:
//          urlencoding::encode
//              pub fn encode(data: &str) -> Cow<'_, str>
//          urlencoding::encode_binary
//              pub fn encode_binary(data: &[u8]) -> Cow<'_, str>
//          urlencoding::decode
//              pub fn decode(data: &str) -> Result<Cow<'_, str>, FromUtf8Error>
//          urlencoding::decode_binary
//              pub fn decode_binary(data: &[u8]) -> Cow<'_, [u8]>
