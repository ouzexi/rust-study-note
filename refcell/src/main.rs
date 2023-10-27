use std::thread;

fn main() {
    let v = vec![1, 2, 3];
    let handle = thread::spawn(move || {
        println!("Here's a vector: {:?}", v); // 线程的闭包里不能主线程的v，因为主线程有可能比线程先结束，此时v被释放了，所以线程里获取不到v
    });                                       // 所以要在闭包函数前加上move关键字，将v的所有权交给线程

    // drop(v); 主动释放v，也会导致线程获取不到v

    handle.join().unwrap();
}