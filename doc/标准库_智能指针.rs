// 
// std::boxed::Box: 堆包装
//      pub fn new(x: T) -> Box<T, Global>
//      pub fn downcast<T>(self) -> Result<Box<T, A>, Box<dyn Any + 'static, A>>
//      *box = 1
// 
// std::cell::Cell 、 std::cell::RefCell:  提供外部不变性, 内部可变性 线程不安全
//      pub const fn new(value: T) -> Cell<T>
//      pub const fn new(value: T) -> RefCell<T>
//  common
//      pub const fn into_inner(self) -> T: 获得T, RefCell不再可用
//      pub fn swap(&self, other: &Cell<T>)
//      pub fn replace(&self, val: T) -> T: 交换内部对象
//      pub fn replace_with<F>(&self, f: F) -> T where F: FnOnce(&mut T) -> T 
//      pub fn take(&self) -> T where T: Default, 
//  Cell:
//      pub fn set(&self, val: T)
//      pub fn get(&self) -> T where T: Copy,
//  RefCell:
//      pub fn borrow(&self) -> Ref<'_, T>
//      pub fn borrow_mut(&self) -> RefMut<'_, T>
//      pub fn try_borrow(&self) -> Result<Ref<'_, T>, BorrowError>
//      pub fn try_borrow_mut(&self) -> Result<RefMut<'_, T>, BorrowMutError>

// 
// Rc / Arc
// 
//  std::rc::Rc 、  std::sync::Arc: 引用计数, 内部包含不可变对象(可变使用Cell RefCel)
//      pub fn new(value: T) -> Rc<T>
//      pub fn pin(value: T) -> Pin<Rc<T>>
//      pub fn downgrade(this: &Rc<T>) -> Weak<T>
//      pub fn weak_count(this: &Rc<T>) -> usize
//      pub fn strong_count(this: &Rc<T>) -> usize


// 
// Any
// 
// traits:
//      std::any::Any
// implements:
//      Any::is::<T>()
//      Any::downcast_ref::<T>() 
//      Any::downcast_mut::<T>()
fn is_string(s: &dyn Any) {
    if s.is::<String>() {
        println!("It's a string!");
    } else {
        println!("Not a string...");
    }
}
fn print_if_string(s: &dyn Any) {
    if let Some(string) = s.downcast_ref::<String>() {
        println!("It's a string({}): '{}'", string.len(), string);
    } else {
        println!("Not a string...");
    }
}
fn modify_if_u32(s: &mut dyn Any) {
    if let Some(num) = s.downcast_mut::<u32>() {
        *num = 42;
    }
}
