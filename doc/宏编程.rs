// 普通宏: 简单AST替换
//     macro_rules! $name {
//         ($arg: ty) => {$expansion};
//         ...
//     }
//     $arg: ty
//         ident: 标识符, 变量名称
//         ty: 类型
//         expr: 表达式
//         item: 语言项, like a function, struct, module, etc.
//         block: 代码块, surrounded by braces
//         stmt: 语句, surrounded by semicolons
//         pat: a pattern
//         path: a path (e.g. foo, ::std::mem::replace, transmute::<_, int>, …)
//         meta: 元信息 the things that go inside #[...] and #![...] attributes
//         tt: a single token tree
//         vis: 可见性, pub
//         lifetime: 生命周期
//      Repetitions
//        $ ( ... ) sep rep
//          sep is an optional separator token. Common examples are , and ;
//          rep is the required repeat control. * (indicating zero or more repeats) or + (indicating one or more repeats)
//          允许最后多一个逗号:
//              ($($exprs:expr),* $(,)*) => {...};
//      Debug
//        #[feature(trace_macros)]
//        trace_macros!(true);
//        cargo expand
//        // 递归回调宏
//        macro_rules! invoke_with_callback {
//            ( $callback:ident($($args:tt)* )) => {
//                  $callback!($($args)*)
//            };
//        }
// 过程宏: AST ==> AST
//      proc-macro库
//          ```toml
//          [lib]
//          proc-macro = true
//          ```
//      辅助框架: quote、syn
//      分类:
//          函数式
//              使用公有函数 pub fn 声明, 函数名即宏名
//              #[proc_macro]
//              函数签名为 (TokenStream) -> TokenStream
//          derive 式
//              使用公有函数 pub fn 声明
//              #[proc_macro_derive(Name)]  #[proc_macro_derive(Name, attributes(attr))]
//              函数签名为 (TokenStream) -> TokenStream
//          属性式
//              使用公有函数 pub fn 声明，函数名为属性名
//              #[proc_macro_attribute]
//              函数签名为 (TokenStream, TokenStream) -> TokenStream
