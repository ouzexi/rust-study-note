fn main() {
    println!("Hello, world!");

    let mut x = 5;
    x = x + 1;
    x = x * 2;
    println!("x:{}", x);

    // shadowing 隐藏
    let spaces = "    ";
    let spaces = spaces.len();

    println!("{}", spaces);
}
