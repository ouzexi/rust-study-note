fn main() {
    let tup: (i32, f64, u8) = (500, 6.4, 1);

    let (x, y, z) = tup; // 解构

    println!("{}, {}, {}", tup.0, tup.1, tup.2);

    println!("{}, {}, {}", x, y, z);
}
