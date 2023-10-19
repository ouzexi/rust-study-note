use std::fmt::Display;

pub trait Summary {
    fn summarize(&self) -> String {     // trait中方法的默认实现
        String::from("(Read more...)")
    }
}

pub struct  NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    /* fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    } */
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

// trait作为函数参数 且 实现多个trait
pub fn notify1(item1: impl Summary + Display) {
    println!("Breaking news! {}", item1.summarize());
}

// 可以使用泛型简化参数类型
pub fn notify<T: Summary + Display>(item1: T, item2: T) {
    println!("Breaking news! {}", item1.summarize());
}

fn main() {
    let article = NewsArticle {
        headline: String::from("headline"),
        content: String::from("contenttttt"),
        author: String::from("author"),
        location: String::from("location"),
    };

    print!("1 new tweet: {}", article.summarize())
}