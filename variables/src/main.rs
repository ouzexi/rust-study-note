fn main() {
    // let x = 5;

    /* let y = { // y就是等于x+3的值
        let x = 1;
        x + 3
    }; */

    let y = plus_five(6);

    println!("The value of y is: {}", y);
}

fn plus_five(x: i32) -> i32 {
    x + 5 // 不能加分号，否则就是语句不是表达式了
}

// 在 -> 符号后边声明函数返回值的类型，但是不可以为返回值命名
// 返回值就是函数体里面最后一个表达式的值
// 若想提前返回，需使用return关键字并指定一个值