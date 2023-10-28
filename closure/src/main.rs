// 1 在Trait定义中使用关联类型来指定占位类型
pub trait Iterator {
    type Item; // 关联类型

    fn next(&mut self) -> Option<Self::Item>;
}

fn main() {
    println!("Hello, world!");

    // 完全限定语法-调用同名方法
    let person = Human;
    person.fly();
    Pilot::fly(&person);
    Wizard::fly(&person);
}

// 2 默认泛型参数和运算符重载
// <PlaceholderType=ConcreteType>

// 3 完全限定语法 如何调用同名方法
trait Pilot {
    fn fly(&self);
}

trait Wizard {
    fn fly(&self);
}

struct Human;

impl Pilot for Human {
    fn fly(&self) {
        println!("This is your captain speaking.");
    }
}

impl Wizard for Human {
    fn fly(&self) {
        println!("Up!");
    }
}

impl Human {
    fn fly(&self) {
        println!("*waving arms furiously*");
    }
}

// 4 使用supertrait来要求trait附带其他trait的功能
// 被一个trait简介依赖的trait也需要被实现

// 5 使用newtype模式在外部类型上实现外部trait