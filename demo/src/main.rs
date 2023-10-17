#[derive(Debug)] // 格式化方式
struct Rectangle {
    width: u32,
    length: u32,
}

fn main() {
    let rect = Rectangle {
        width: 30,
        length: 50,
    };

    println!("{}", area(&rect));

    println!("{:#?}", rect); // 格式化方式
}

fn area(rect: &Rectangle) -> u32 {
    rect.width * rect.length
}