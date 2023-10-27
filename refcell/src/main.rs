use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_millis(1));  // 如果是发送多个值，会阻塞线程，看到接收者在等待
        }
    });

    for received in rx {  // 这里使用循环解构，就不需要调用recv方法了
        println!("Got: {}", received);
    }
}