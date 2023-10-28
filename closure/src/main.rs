// 1 类型别名
type Kilometers = i32;

fn main() {
    let x: i32 = 5;
    let y: Kilometers = 5;
    println!("x + y = {}", x + y);

    // 2 Never类型
    loop {
        // 永远不会返回则为Never类型
    }
}

// 3 动态大小和Sized Trait
// fn generic<T>(t: T) {}
// fn generic<T: Sized>(t: T) {}
// fn generic<T: ?Sized>(t: &T) {}