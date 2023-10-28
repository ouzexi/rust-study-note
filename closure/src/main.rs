// 1 函数指针
// 可以将函数传递给其他函数，函数在传递过程中会被强制转换成fn类型
fn add_one(x: i32) -> i32 {
    x + 1
}

fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 {
    f(arg) + f(arg)
}

fn main() {
    let answer = do_twice(add_one, 5);
    println!("The answer is: {}", answer); //12
}

// 2 函数指针与闭包的不同
// fn是一个类型，不是一个trait

// 3 返回闭包
/* fn returns_closure() -> Fn(i32) -> i32 {  报错
    |x| x + 1
} */

n returns_closure() -> Box<dyn Fn(i32) -> i32> {
    Box::new(|x| x + 1)
}