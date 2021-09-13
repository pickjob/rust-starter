// Best Practice:
//      命名:
//          Types / Traits / Enum variants: UpperCamelCase
//          Modules / Methods / Functions / Macros / Local variables: snake_case
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

//      Convert:
//          as_(std::convert::AsRef, std::convert::AsMut): Free
//          into_(std::convert::From, std::convert::Into): Variable
//          to_(std::borrow::ToOwned): Expensive
//          usage:
//                  str.as_bytes(): str => &[u8]
//                  str.as_bytes_mut(): str => &mut [u8]
//                  str.to_owned(): str => String
//                  str::from_utf8(): &[u8] => &str
//                  str::from_utf8_mut(): &mut [u8] => &mut str
// 
//                  String.as_bytes(): String => &[u8]
//                  String.as_str(): String => &str
//                  String.as_mut_str(): String => &mut str
//                  String::as_ref(): String => [u8]
//                  String.into_bytes(): String => Vec<u8>
//                  String.borrow(): String => &str
//                  String.borrow_mut(&mut self): String =>&mut str
//                  String::from_utf8_lossy(): &[u8] => Cow<'_, str>
// 
//                  Path.to_owned(): Path => PathBuf
//      Deref:
//          Vec<T> impl Deref<Target = [T]>
//          String impl Deref<Target = str> 
//          PathBuf impl Deref<Path>
// 
//          Rc<T> impl<T> Deref<Target = T> where T: ?Sized, 
//          Arc<T> impl<T> Deref for where T: ?Sized,
//          Box<T> impl<T> Deref<Target = T> where T: ?Sized,
//          Cow<T> impl<T> Deref<Target = T> where T: ToOwned + ?Sized, 
//      Default:
//          String impl Default
//          Option<T> impl<T> Default
// 
//          Rc<T> impl<T> Default where T: Default, 
//          Arc<T> impl<T> Default where T: Default, 
//          Box<[T]> impl<T> Default
//          Cell<T> impl<T> Default where T: Default, 
//          RefCell<T> impl<T> Default where T: Default, 
//          Cow<T> impl Default where T: ToOwned + ?Sized, <B as ToOwned>::Owned: Default,
//  
//          Vec<T>, HashMap<K, V>, VecDeque<T> impl Default
//      Iterator:
//          fn iter(&self) -> Iter             // Iter implements Iterator<Item = &U>
//          fn iter_mut(&mut self) -> IterMut  // IterMut implements Iterator<Item = &mut U>
//          fn into_iter(self) -> IntoIter     // IntoIter implements Iterator<Item = U>Fiter
// 

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
