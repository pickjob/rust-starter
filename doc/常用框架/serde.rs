// 
// serde: 序列化、反序列化框架
//      traits:
//          serde::Serialize: A data structure that can be serialized into any data format supported by Serde, 由属性宏实现
//          serde::Deserialize: A data structure that can be deserialized from any data format supported by Serde, 由属性宏实现
//          serde::Serializer: A data format that can serialize any data structure supported by Serde
//          serde::Deserializer: A data format that can deserialize any data structure supported by Serde
//          serde::de::Visitor: Visitor辅助完成反序列化
//              pub trait Deserialize<'de>: Sized {
//                  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: Deserializer<'de>;
//              }
//              pub trait Serialize {
//                  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer;
//              }
//              pub trait Visitor<'de>: Sized {
//                  type Value;
//                  fn expecting(&self, formatter: &mut Formatter<'_>) -> Result;
//                  fn visit_xxx<E>(self, v: bool) -> Result<Self::Value, E> where E: Error,
//              }
//      #[derive(Serialize, Deserialize)]
//      Attributes:  #[serde()]
//          Container attributes — apply to a struct or enum declaration
//              #[serde(deny_unknown_fields)]
//              #[serde(default)] #[serde(default = "path")]
//              #[serde(rename(serialize = "ser_name"))] #[serde(rename(deserialize = "de_name"))] #[serde(rename(serialize = "ser_name", deserialize = "de_name"))]
//              #[serde(rename_all = "...")]
//                  "lowercase", "UPPERCASE", "PascalCase", "camelCase", "snake_case", "SCREAMING_SNAKE_CASE", "kebab-case", "SCREAMING-KEBAB-CASE"
//          Variant attributes — apply to a variant of an enum
//              #[serde(skip)] #[serde(skip_serializing)] #[serde(skip_deserializing)]
//              #[serde(serialize_with = "path")] #[serde(deserialize_with = "path")]
//              #[serde(rename(serialize = "ser_name"))] #[serde(rename(deserialize = "de_name"))] #[serde(rename(serialize = "ser_name", deserialize = "de_name"))]
//              #[serde(alias = "name")]
//          Field attributes — apply to one field in a struct or in an enum variant
//              #[serde(borrow)] #[serde(borrow = "'a + 'b + ...")]
//              #[serde(skip)] #[serde(skip_serializing)] #[serde(skip_deserializing)]
//              #[serde(default)] #[serde(default = "path")]
//              #[serde(rename(serialize = "ser_name"))] #[serde(rename(deserialize = "de_name"))] #[serde(rename(serialize = "ser_name", deserialize = "de_name"))]
//              #[serde(serialize_with = "path")] #[serde(deserialize_with = "path")]
//              #[serde(with = "module")] : $module::serialize $module::deserialize
//              指定序列、反序列函数  
//                  fn<S>(&T, S) -> Result<S::Ok, S::Error> where S: Serializer,
//                  fn<'de, D>(D) -> Result<T, D::Error> where D: Deserializer<'de>
// serde_json、serde_yaml、serde_qs:
//      serde_json::Value
//          enum Value {
//              Null,
//              Bool(bool),
//              Number(Number),
//              String(String),
//              Array(Vec<Value>),
//              Object(Map<String, Value>),
//          }
//      serde_yaml::Value
//          pub enum Value {
//              Null,
//              Bool(bool),
//              Number(Number),
//              String(String),
//              Sequence(Sequence),
//              Mapping(Mapping),
//          }
//      macros:
//          serde_json::json!
//      functions:
//          serde_json::from_str / serde_json::to_string / serde_json::to_string_pretty
//          serde_json::from_slice
//          serde_json::from_reader / serde_json::to_writer / serde_json::to_writer_pretty
//          serde_json::from_value serde_json::to_value
//
//          serde_yaml::from_str / serde_yaml::to_string
//          serde_yaml::from_slice
//          serde_yaml::from_reader / serde_yaml::to_writer
//          serde_yaml::from_value serde_json::to_value
// 
//          serde_qs::from_str / serde_qs::to_string
// serde-transcode:
//      functions:
//          pub fn transcode<'de, D, S>(d: D, s: S) -> Result<S::Ok, S::Error> where D: de::Deserializer<'de>, S: ser::Serializer
