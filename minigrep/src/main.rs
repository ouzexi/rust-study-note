use std::env;
use std::fs;
// cargo run abcd readme.txt 实现简易的grep命令，在readme.txt查找adbc字符串
fn main() {
    let args: Vec<String> = env::args().collect();

    let query = &args[1];
    let filename = &args[2];

    println!("Search for {}", query);
    println!("In file {}", filename);

    let contents = fs::read_to_string(filename)
    .expect("Something went wrong reading the file");

    println!("With text:\n{}", contents);
}