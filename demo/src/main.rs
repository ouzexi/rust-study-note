#[derive(Debug)] // 格式化方式
struct Rectangle {
    width: u32,
    length: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.length
    }
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.length > other.length
    }

    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            length: size
        }
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        length: 50,
    };
    let rect2 = Rectangle {
        width: 10,
        length: 40,
    };
    let rect3 = Rectangle {
        width: 35,
        length: 55,
    };

    println!("{}", &rect1.area());

    println!("{}", &rect1.can_hold(&rect2));
    println!("{}", &rect1.can_hold(&rect3));

    let s = Rectangle::square(20);
    println!("{:#?}", s);

}