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
fn main() {
    let x = 5;
    // let y = &x;
    let y = MyBox::new(x); // 用MyBox新建一个y，相当于获取了x的引用

    assert_eq!(5, x);
    assert_eq!(5, *y);
}