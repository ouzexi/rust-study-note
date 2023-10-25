struct CustomSmartPointer {
    data: String,
}
// 实现Drop Trait，可以自定义当值将要离开作用域时发生的动作，一般用于文件，网络资源释放
impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`!", self.data);
    }
}

fn main() {
    let c = CustomSmartPointer{ data: String::from("my stuff") };
    drop(c); // 主动调用drop后，不会在离开main时被2次调用
    let d = CustomSmartPointer { data: String::from("other stuff") };
    println!("CustomSmartPointers created.");
}
// 主动调用预导入模块std::mem::drop，就会主动调用，如上例就是先打印c再到d
// Dropping CustomSmartPointer with data `my stuff`!
// CustomSmartPointers created.
// Dropping CustomSmartPointer with data `other stuff`!


// 如果不主动调用drop方法，在离开main函数时，会先打印Dropping CustomSmartPointer with data d再到c
// CustomSmartPointers created.
// Dropping CustomSmartPointer with data `other stuff`!
// Dropping CustomSmartPointer with data `my stuff`!