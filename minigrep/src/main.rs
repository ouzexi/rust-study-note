use std::env;
// cargo run abcd readme.txt 实现简易的grep命令，在readme.txt查找adbc字符串
fn main() {
    let args: Vec<String> = env::args().collect();

    let query = &args[1];
    let filename = &args[2];

    println!("Search for {}", query);
    println!("In file {}", filename);
}