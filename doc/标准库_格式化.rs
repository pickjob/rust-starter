// 
// format
// 
// macros:
//      std::format! / std::format_args!: write formatted text to String
//      std::print! / std::println!:  write formatted text to io::stdout
//      std::eprint! / std::eprintln!: write formatted text to io::stderr
//      std::write! / std::writeln: write formatted text to &mut io::Write
// traits:
//      std::fmt::Display: support holder {}
//          fn fmt(&self, f: &mut Formatter) -> Result<(), Error>;
//      std::fmt::Debug: support holder {:?}、{:#?}
//          fn fmt(&self, f: &mut Formatter) -> Result<(), Error>;
//      std::fmt::Binary: 二进制格式接口
//      std::fmt::LowerHex / std::fmt::UpperHex: 十六进制接口
// usage:
//      "{}", arg
//      "{0}", arg
//      "{key}", key=val
//      "{:#X}", arg, 0xAA十六进制
//      "{:#x", arg, 0xaa进制
struct Point {
    x: i32,
    y: i32,
}
impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}
#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}
