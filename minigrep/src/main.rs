use minigrep::Config;
use std::env;
use std::process;
// cargo run abcd readme.txt 实现简易的grep命令，在readme.txt查找adbc字符串
fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {  // |err|是闭包
        println!("Problem parsing arguments: {}", err);
        process::exit(1); // 退出程序
    });

    if let Err(e) = minigrep::run(config) {
        println!("Application error: {}", e);
        process::exit(1);
    }
}

