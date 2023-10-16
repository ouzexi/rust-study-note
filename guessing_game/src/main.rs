use std::io; // prelude
use std::cmp::Ordering;
use rand::Rng; // trait - 类似接口，里面定义了很多方法

fn main() {
    println!("猜数！");

    let secret_number = rand::thread_rng().gen_range(1, 101); // 生成一个1-100之间的随机数

    // println!("生成神秘数字是：{}", secret_number);

    loop {
        println!("猜测一个数");
    
        let mut guess = String::new();
    
        io::stdin().read_line(&mut guess).expect("无法读取行");
    
        let guess: u32 = match guess.trim().parse() { // match返回2个变体，OK时使用输入的数字，Err时继续下一次循环
            Ok(num) => num,
            Err(_) => continue,
        }; // parse会将字符串转成某个类型的数字，这里是u32
    
        println!("你猜测的数是：{}", guess);
    
        match guess.cmp(&secret_number) { // 因为这里guess是u32类型，所以与它比较的secret_number也会从i32被转换为u32类型
            Ordering::Less => println!("太小了"),
            Ordering::Greater => println!("太大了"),
            Ordering::Equal => {
                println!("猜对了");
                break;
            },
        }
    }
}