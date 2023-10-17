fn main() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is: {}", result); // 20

    //
    let a = [10, 20, 30, 40, 50];

    for element in a.iter() {
        println!("the value is： {}", element);
    }

    for number in (1..4).rev() { // ..是Range，1到3 rev是反转
        println!("{}!", number);
    }
    println!("LIFTOFF!");
}