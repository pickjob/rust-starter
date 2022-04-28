// 
// serde: 序列化、反序列化框架
//      traits:
//          serde::Serialize: 支持被序列化
//          serde::Deserialize: 支持序列化结构
//          serde::Serializer: 支持被反序列化
//          serde::Deserializer: 支持反序列化结构
//          serde::de::Visitor: Visitor辅助完成反序列化
//              pub trait Serialize {
//                  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer;
//              }
//              pub trait Deserialize<'de>: Sized {
//                  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: Deserializer<'de>;
//              }
//              pub trait Visitor<'de>: Sized {
//                  type Value;
//                  fn expecting(&self, formatter: &mut Formatter<'_>) -> Result;
//                  fn visit_xxx<E>(self, v: bool) -> Result<Self::Value, E> where E: Error,
//              }
//      macro:
//          标注结构
//              #[derive(Serialize, Deserialize)]
//          标注属性
//              重命名
//                  #[serde(rename(serialize = "ser_name"))]
//                  #[serde(rename(deserialize = "de_name"))]
//                  #[serde(rename(serialize = "ser_name", deserialize = "de_name"))]
//                  #[serde(rename_all = "...")]
//                      "lowercase", "UPPERCASE", "PascalCase", "camelCase", "snake_case", "SCREAMING_SNAKE_CASE", "kebab-case", "SCREAMING-KEBAB-CASE"
//              拒绝未知字段
//                  #[serde(deny_unknown_fields)]
//              调用生成默认字段
//                  #[serde(default)] #[serde(default = "path")]
//              忽略、跳过
//                  #[serde(transparent)]
//                  #[serde(skip)] #[serde(skip_serializing)] #[serde(skip_deserializing)]
//              使用自定义序列、反序列化函数
//                  #[serde(serialize_with = "path")] #[serde(deserialize_with = "path")]
//                  #[serde(with = "module")]
//                      $module::serialize $module::deserialize
//                      fn serialize<S>(&T, S) -> Result<S::Ok, S::Error> where S: Serializer,
//                      fn deserialize<'de, D>(D) -> Result<T, D::Error> where D: Deserializer<'de>
// serde_json: json
//      serde_json::Value {
//          Null,
//          Bool(bool),
//          Number(Number),
//          String(String),
//          Array(Vec<Value>),
//          Object(Map<String, Value>),
//      }
//          serde_json::from_str<'a, T>(s: &'a str) -> Result<T> where T: Deserialize<'a>, 
//          serde_json::from_slice<'a, T>(v: &'a [u8]) -> Result<T> where T: Deserialize<'a>,
//          serde_json::from_reader<R, T>(rdr: R) -> Result<T> where R: Read, T: DeserializeOwned, 
//          serde_json::from_value<T>(value: Value) -> Result<T, Error> where T: DeserializeOwned,
//
//          serde_json::to_string<T>(value: &T) -> Result<String> where T: ?Sized + Serialize,
//          serde_json::to_string_pretty<T>(value: &T) -> Result<String> where T: ?Sized + Serialize, 
//          serde_json::to_writer<W, T>(writer: W, value: &T) -> Result<()>  where W: Write, T: ?Sized + Serialize, 
//          serde_json::to_writer_pretty<W, T>(writer: W, value: &T) -> Result<()> where W: Write, T: ?Sized + Serialize, 
//          serde_json::to_vec<T>(value: &T) -> Result<Vec<u8>> where T: ?Sized + Serialize,
//          serde_json::to_vec_pretty<T>(value: &T) -> Result<Vec<u8>> where T: ?Sized + Serialize, 
//          serde_json::to_value<T>(value: T) -> Result<Value, Error> where T: Serialize,
// serde_yaml: yaml
//      serde_yaml::Value {
//          Null,
//          Bool(bool),
//          Number(Number),
//          String(String),
//          Sequence(Sequence),
//          Mapping(Mapping),
//      }
//          serde_yaml::from_str<T>(s: &str) -> Result<T> where T: DeserializeOwned, 
//          serde_yaml::from_slice<T>(v: &[u8]) -> Result<T> where T: DeserializeOwned, 
//          serde_yaml::from_reader<R, T>(rdr: R) -> Result<T> where R: Read, T: DeserializeOwned, 
//          serde_yaml::from_value<T>(value: Value) -> Result<T, Error> where T: DeserializeOwned, 
// 
//          serde_yaml::to_string<T: ?Sized>(value: &T) -> Result<String> where T: Serialize, 
//          serde_yaml::to_writer<W, T: ?Sized>(writer: W, value: &T) -> Result<()> where W: Write, T: Serialize, 
//          serde_yaml::to_vec<T: ?Sized>(value: &T) -> Result<Vec<u8>> where T: Serialize, 
//          serde_json::to_value<T>(value: T) -> Result<Value, Error> where T: Serialize, 
// serde_qs: url encoding
//      serde_qs::from_str<'de, T: Deserialize<'de>>(input: &'de str) -> Result<T, Error>
//      serde_qs::from_bytes<'de, T: Deserialize<'de>>(input: &'de [u8]) -> Result<T, Error>
// 
//      serde_qs::to_string<T: Serialize>(input: &T) -> Result<String, Error>
//      serde_qs::to_writer<T: Serialize, W: Write>(input: &T, writer: &mut W) -> Result<(), Error>
// serde-transcode: Deserializer To Serializer
//       serde_transcode::transcode<'de, D, S>(d: D, s: S) -> Result<S::Ok, S::Error> where D: de::Deserializer<'de>, S: ser::Serializer
