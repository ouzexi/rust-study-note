fn main() {
    let my_string = String::from("Hello world");
    let word_index = first_word(&my_string[..]);

    println!("{}", word_index);

    let my_string_literal = "hello world";
    let word_index = first_word(my_string_literal);

    println!("{}", word_index);
}

fn first_word(s: &str) -> &str { // &str是字符串切片引用
    let bytes = s.as_bytes();

    for(i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}