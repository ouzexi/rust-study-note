use crate::List::{Cons, Nil};
use std::rc::Rc;

enum List {
    Cons(i32, Rc<List>),
    Nil
}

fn main() {
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    println!("count after creating a = {}", Rc::strong_count(&a)); // 此时a的强引用为1，只有自己引用

    let b = Cons(3, Rc::clone(&a));
    println!("count after creating b = {}", Rc::strong_count(&a)); // 此时引用值为2，b引用它

    {
        let c = Cons(4, Rc::clone(&a));
        println!("count after creating c = {}", Rc::strong_count(&a)); // 此时引用值为3，c引用它
    }
    println!("count after c goes out of score = {}", Rc::strong_count(&a)); // 此时c离开它的作用域了，释放了，引用剩2个
}