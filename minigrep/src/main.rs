use std::env;
use std::fs;
// cargo run abcd readme.txt 实现简易的grep命令，在readme.txt查找adbc字符串
fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args);

    let contents = fs::read_to_string(config.filename)
    .expect("Something went wrong reading the file");

    println!("With text:\n{}", contents);
}

struct Config {
    query: String,
    filename: String,
}

impl Config {
    fn new(args: &[String]) -> Config {
        let query = args[1].clone();
        let filename = args[2].clone();

        Config { query, filename }
    }
}