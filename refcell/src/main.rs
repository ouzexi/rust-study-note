use std::sync::{Mutex, Arc};  // RefCell<T>可以修改Rc<T>里的内容，Mutex<T>可以修改Arc<T>里的内容
use std::thread;
// Rc<T>和Arc<T>的api是一样的，但Arc可用户并发情景，Mutex有死锁风险
fn main() {
    let counter = Arc::new(Mutex::new(0)); // 共享counter，就可以在循环中的多个线程获取所有权
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();

            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());
}