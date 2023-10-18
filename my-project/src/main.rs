fn main() {
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = format!("{}-{}-{}", s1, s2, s3);
    println!("{}", s);

    let mut s4 = String::from("foo");
    let s5 = String::from("bar");
    s4.push_str(&s5);

    println!("{}", s4);
}