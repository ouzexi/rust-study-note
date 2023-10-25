use std::ops::Deref;

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {  // 要使用*y引用，要实现Deref trait（类似于java重写接口的方法）
    type Target = T;

    fn deref(&self) -> &T {
        &self.0 // 把self元组的第0个元素返回
    }
}

fn hello(name: &str) {
    println!("Hello, {}", name);
}

fn main() {
    let m = MyBox::new(String::from("Rust"));

    // hello函数接收参数的类型是字符串切片的引用，但是这里可以传MyBox<String>的引用
    // 是因为Deref Trait自动拆箱的过程
    // &m = &MyBox<String> -> deref &String -> deref &str 
    hello(&m);

    hello("Rust");
}