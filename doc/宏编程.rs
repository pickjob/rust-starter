// macro_rules! $name {
//     ($arg: ty) => {$expansion};
//     ...
// }
//  $arg: ty
//      ident: an identifier
//      ty: a type
//      expr: an expression
//      item: 语言项, like a function, struct, module, etc.
//      block: 代码块, surrounded by braces
//      stmt: 语句, surrounded by semicolons
//      pat: a pattern
//      path: a path (e.g. foo, ::std::mem::replace, transmute::<_, int>, …)
//      meta: 元信息 the things that go inside #[...] and #![...] attributes
//      tt: a single token tree
//      vis: 可见性, pub
//      lifetime: 生命周期
//  Repetitions
//    $ ( ... ) sep rep
//      sep is an optional separator token. Common examples are , and ;
//      rep is the required repeat control. * (indicating zero or more repeats) or + (indicating one or more repeats)
//      允许最后多一个逗号:
//          ($($exprs:expr),* $(,)*) => {...};
//  Debug
//    #[feature(trace_macros)]
//    trace_macros!(true);
//    cargo expand
// 重复
macro_rules! vec_strs {
    (
        // Start a repetition:
        $(
            // Each repeat must contain an expression...
            $element:expr
        )
        // ...separated by commas...
        ,
        // ...zero or more times.
        *
    ) => {
        // Enclose the expansion in a block so that we can use
        // multiple statements.
        {
            let mut v = Vec::new();

            // Start a repetition:
            $(
                // Each repeat will contain the following statement, with
                // $element replaced with the corresponding expression.
                v.push(format!("{}", $element));
            )*

            v
        }
    };
}
// 递归回调宏
macro_rules! invoke_with_callback {
    ( $callback:ident($($args:tt)* )) => {
        $callback!($($args)*)
    };
}