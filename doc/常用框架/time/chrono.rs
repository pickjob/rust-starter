// 
// Chrono: Date and Time for Rust
//      use chrono::prelude::*;
//      traits:
//          chrono::offset::TimeZone
//      structs:
//          chrono::offset::Utc
//              pub fn now() -> DateTime<Utc>
//              pub fn today() -> Date<Utc>
//          chrono::offset::Local
//              pub fn today() -> Date<Local>
//              pub fn now() -> DateTime<Local>
//          chrono::DateTime<Tz: TimeZone>
//              pub fn format<'a>(&self, fmt: &'a str) -> DelayedFormat<StrftimeItems<'a>>
//              pub fn parse_from_str(s: &str, fmt: &str) -> ParseResult<DateTime<FixedOffset>>
//              impl FromStr for DateTime<Utc>
//              impl FromStr for DateTime<Local>
//              impl FromStr for DateTime<FixedOffset>
//      DATE SPECIFIERS: %Y-%m-%d %H:%M:%S
//          %Y	The full proleptic Gregorian year, zero-padded to 4 digits
//          %m  Month number (01--12), zero-padded to 2 digits
//          %d  Day number (01--31), zero-padded to 2 digits
//          %H	Hour number (00--23), zero-padded to 2 digits
//          %M	Minute number (00--59), zero-padded to 2 digits
//          %S  Second number (00--60), zero-padded to 2 digits
//      #[serde(serialize_with = "", deserialize_with = "", with = "module")]
//          module
//              ts_milliseconds
//              ts_milliseconds_option
//              ts_nanoseconds
//              ts_nanoseconds_option
//              ts_seconds
//              ts_seconds_option
// 
