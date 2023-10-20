use std::env;
use std::fs;
use std::process;
// cargo run abcd readme.txt 实现简易的grep命令，在readme.txt查找adbc字符串
fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {  // |err|是闭包
        println!("Problem parsing arguments: {}", err);
        process::exit(1); // 退出程序
    });

    let contents = fs::read_to_string(config.filename)
    .expect("Something went wrong reading the file");

    println!("With text:\n{}", contents);
}

struct Config {
    query: String,
    filename: String,
}

impl Config {
    fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }
        
        let query = args[1].clone();
        let filename = args[2].clone();

        Ok(Config { query, filename })
    }
}