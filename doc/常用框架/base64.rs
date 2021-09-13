// 
// base64
//      functions:
//          base64::encode
//              pub fn encode<T: AsRef<[u8]>>(input: T) -> String
//          base64::encode_config
//              pub fn encode_config<T: AsRef<[u8]>>(input: T, config: Config) -> String
//          base64::decode
//              pub fn decode<T: AsRef<[u8]>>(input: T) -> Result<Vec<u8>, DecodeError>
//          base64::decode_config
//              pub fn decode_config<T: AsRef<[u8]>>(input: T, config: Config) -> Result<Vec<u8>, DecodeError>
//      consts:
//          pub const STANDARD: Config; (+ and /)
//          pub const STANDARD_NO_PAD: Config;
//          pub const URL_SAFE: Config; (- and _)
//          pub const URL_SAFE_NO_PAD: Config;
