// 生命周期省略规则:
//      Each elided lifetime in input position becomes a distinct lifetime parameter
//      每一个在输入位置省略的生命周期都对应一个唯一的生命周期参数
//      If there is exactly one input lifetime position (elided or not), that lifetime is assigned to all elided output lifetimes
//      如果只有一个输入的生命周期位置(无论省略还是没省略), 那个生命周期会赋给所有省略了的输出生命周期
//      If there are multiple input lifetime positions, but one of them is &self or &mut self, the lifetime of self is assigned to all elided output lifetimes
//      如果有多个输入生命周期位置, 而其中一个是 &self 或者 &mut self, 那么 self 的生命周期会赋给所有省略了的输出生命周期
//      Otherwise, it is an error to elide an output lifetime
//      其他省略生命周期的情况都是错误的
// 
// Ownership: You own a resource, and when you are done with it, that resource is no longer in scope and gets deallocated.
// References to a resource depend on the lifetime of that resource (i.e., they are only valid until the resource is deallocated).
// Move semantics means: Giving an owned resource to a function means giving it away. You can no longer access it.
// To not move a resource, you instead use borrowing: You create a reference to it and move that. When you create a reference, you own that reference. Then you move it (and ownership of it) to the function you call. (Nothing new, just both concepts at the same time.)
// To manipulate a resource without giving up ownership, you can create one mutable reference. During the lifetime of this reference, no other references to the same resource can exist.
// That’s it. And it’s all checked at compile-time.
