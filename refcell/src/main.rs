use std::sync::mpsc;
use std::thread;

fn main() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let val = String::from("hi");
        tx.send(val).unwrap();  // 发送端的send方法发送数据
        // println!("{}", val); 这里会报错了，因为val的所有权移交到主线程
    });

    let received = rx.recv().unwrap(); // 接收端的recv方法接收数据
    println!("Got: {}", received);
}