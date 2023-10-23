use std::{thread, time::Duration};

fn main() {
    let example_closure = |x| x; // 相当于js的(x) => x;

    let s = example_closure(String::from("hello"));
    let n = example_closure(5); // 这个会报错，因为上面已经推断x为String类型了
}

fn generate_workout(intensity: u32, random_number: u32) {
    let expensive_closure = |num| { // 匿名函数，参数要用||包裹
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2)); // 模拟耗时操作
        num
    };

    if intensity < 25 {
        println!("Today, do {} pushups!", expensive_closure(intensity)); // 使用闭包
        println!("Next, do {} situps!", expensive_closure(intensity));
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!"); // 这里就不需要调用耗时函数了
        } else {
            println!("Today, run for {} minutes!", expensive_closure(intensity));
        }
    }
}