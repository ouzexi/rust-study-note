fn main() {
    // 能匹配任何可能传递的值的模式：无可辩驳的 letx = 5;
    // 对某些可能的值无法进行匹配的模式：可辩驳的 if let Some(x) = a_value;

    // 无可辩驳：函数参数、let、for循环
    // 可辩驳的：if let、while let
    let a: Option<i32> = Some(5);
    // let Some(x) = a; // 此处会报错，因为Some是可辩驳的
}