// 宏属性(Macro attributes)
//   #[proc_macro_attribute]
//   pub fn return_as_is(_attr: TokenStream, item: TokenStream) -> TokenStream {
//       item
//   }
//   #[return_as_is]
//   fn invoke() {}
// 派生宏辅助属性(Derive macro helper attributes)
//   #[proc_macro_derive(HelperAttr, attributes(helper))]
//   pub fn derive_helper_attr(_item: TokenStream) -> TokenStream {
//       TokenStream::new()
//   }
//   #[derive(HelperAttr)]
//   struct Struct {
//       #[helper] field: ()
//   }
// 工具属性(Tool attributes)
// 内建属性(Built-in attributes)
//   - 条件编译
//       - cfg
//       - cfg_attr
//   - 测试
//       - test
//       - ignore
//       - should_panic
//   - 派生
//       - derive
//           - Copy、Clone
//           - Hash
//           - Default
//           - Debug
//           - Comparision: EQ, PartialEq, Ord, PartialOrd
//   - 宏相关
//       - macro_export
//       - macro_use
//       - proc_macro
//       - proc_macro_derive
//       - proc_macro_attribute
//   - 诊断
//       - allow, warn, deny, forbid - lint 相关标志开关，各种 lint 见附录。
//       - deprecated
//       - must_use
//   - ABI, 链接, 符号, 和 FFI
//       - link
//       - link_name
//       - no_link
//       - repr
//       - crate_type
//       - no_main
//       - export_name
//       - link_section
//       - no_mangle
//       - used
//       - crate_name
//   - 代码生成
//       - inline
//       - cold
//       - no_builtins
//       - target_feature
//   - 文档
//       - doc
//   - 预引入
//       - no_std
//       - no_implicit_prelude
//   - 模块
//       - path
//   - 限制
//       - recursion_limit
//       - type_length_limit
//   - 运行时
//       - panic_handler
//       - global_allocator
//       - windows_subsystem
//   - 语言特性
//       - feature - 经常会碰到这里面一些陌生的 feature 名称，需要根据具体的 rustc 版本和所使用的库文档进行查阅。
//   - 类型系统
//       - non_exhaustive
