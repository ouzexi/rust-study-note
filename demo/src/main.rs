fn main() {
    let v = Some(0u8);
    match v {
        Some(3) => println!("three"),
        _ => println!("others"),
    }

    // if let只针对一种模式匹配，等价上面
    if let Some(3) = v {
        println!("three");
    } else {
        println!("others");
    }

}