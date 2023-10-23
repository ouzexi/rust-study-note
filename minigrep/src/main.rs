use std::{thread, time::Duration};

struct Cacher<T>
where T: Fn(u32) -> u32 {
    calculation: T,
    value: Option<u32>,
}

impl<T> Cacher<T> where T: Fn(u32) -> u32 {
    // 构造函数
    fn new(calculation: T) -> Cacher<T> {
        Cacher {
            calculation,
            value: None,
        }
    }

    fn value(&mut self, arg: u32) -> u32 {
        match self.value {
            Some(v) => v,
            None => {
                let v = (self.calculation)(arg);
                self.value = Some(v);
                v
            }
        }
    }
}

fn main() {
    // generate_workout(24, 4);

    let x = vec![1, 2, 3];

    let equal_to_x = move |z| z == x;

    // println!("can't use x here: {:?}", x); // 因为move将x的所有权移到匿名函数里了，所有匿名函数外不能再使用x了

    let y = vec![1, 2, 3];

    assert!(equal_to_x(y));
}

fn generate_workout(intensity: u32, random_number: u32) {
    let mut expensive_closure = Cacher::new(|num| { // 匿名函数，参数要用||包裹
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2)); // 模拟耗时操作
        num
    });

    if intensity < 25 {
        println!("Today, do {} pushups!", expensive_closure.value(intensity)); // 使用闭包
        println!("Next, do {} situps!", expensive_closure.value(intensity));
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!"); // 这里就不需要调用耗时函数了
        } else {
            println!("Today, run for {} minutes!", expensive_closure.value(intensity));
        }
    }
}