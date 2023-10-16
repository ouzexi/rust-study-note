use std::io; // prelude
use rand::Rng; // trait - 类似接口，里面定义了很多方法

fn main() {
    println!("猜数！");

    let secret_number = rand::thread_rng().gen_range(1, 101); // 生成一个1-100之间的随机数

    println!("生成神秘数字是：{}", secret_number);

    println!("猜测一个数");

    let mut guess = String::new();

    io::stdin().read_line(&mut guess).expect("无法读取行");

    println!("你猜测的数是：{}", guess);
}