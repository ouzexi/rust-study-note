/* fn main() {
    let b = Box::new(5); // stack存指针，指针指向存在heap的数据
    println!("b = {}", b);
} */
use crate::List::{Cons, Nil};
// Box<T>只提供了间接存储和heap内存分配的功能
fn main() {
    let list = Cons(1,
        Box::new(Cons(2,
            Box::new(Cons(3,
                Box::new(Nil))))));
}

enum List {
    Cons(i32, Box<List>),
    Nil,
}
