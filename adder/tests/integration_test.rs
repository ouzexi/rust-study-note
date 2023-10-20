use adder;

#[test] // 不需要#[cfg(test)]，rust会把tests目录下的rs文件仅当cargo test时才编译
fn it_adds_two() {
    assert_eq!(4, adder::add_two(2))
}