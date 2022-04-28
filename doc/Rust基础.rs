// Best Practice:
//      命名:
//          Types / Traits / Enum variants: UpperCamelCase
//          Modules / Methods / Functions / Macros / Local variables: snake_case
//          使用 mut、move、ref 方法后缀提示参数可变性
//      prefer using the borrowed type over borrowing the owned type
//          &str over &String
//          &[T] over &Vec<T>
//          &T over &Box<T>
//      prefer using trait
//          std::op::Deref: auto deref
//          std::op::Drop: destructor
//          std::default:Default: default construct
//          std::fmt::Display: {}
//          std::fmt::Debug: {:?}
//          std::clone::Clone
//          std::marker::Copy: Clone
//      prefer using trait objects
//      prefer using function pointers
//          type FnPtr = fn() -> String;
//      prefer using Fn trait objects
//      concatenating strings with format!
//      惯例:
//          new: 构造器, 参数 no self, >= 1
//          with_...: 可选构造器, 参数 no self, >= 1
//          from_...: 转换接口, 参数 >=1
//          into_...: 可能昂贵转换接口, 参数 self
//          as_...: 轻松转换接口, 参数&self
//          to_...: 昂贵转换接口, 参数&self
//          is_...: 判断, 返回bool, 参数 &self or none
//          has_...: 判断, 返回bool, 参数 &self or none

fn main() {
    // if-else
    if expr1 {
    } else {
    }
    // for
    for var in iterator {
        code
    }
    // while
    while expression {
        code
    }
    // match
    match x {
        0 if expr => expr, // 模式守护
        1..3 => expr,
        n @ 2 => expr, // 重新绑定, varName @ pattern
        ref r => println("{}", r), // 被匹配的引用
        _ => expr, // 默认情况
    }
    // if-let
    if let Some(y) = x { // 匹配的模式 = 匹配的值
    }
    // while-let
    while let Some(y) = x { // 匹配的模式 = 匹配的值
    }
    // range
    for i in 1..5 {
        println!("{}", i); // 1, 2, 3, 4
    }
    for i in 1 ..= 5 {
        println!("{}", i); // 1, 2, 3, 4, 5
    }
    println!("Hello World", i);
}

// struct
struct People {
    name: &'static str,
    gender: u32
}
struct Color(i32, i32, i32);
struct Empty;
// enum
enum Number {
    Zero,
    One,
    Two,
}
