fn main() {
    let string1 = String::from("abcd");
    let result;
    {
        let string2 = String::from("xyz");
        result = longest(string1.as_str(), string2.as_str());
    }
    println!("The longest string is {}", result);
     // 这里会报错，因为str1和str2都是用同一个生命周期，而他们会取两者较小的一个
     // 所以会选str2的生命周期，它离开第7行就失效了
     // 因为返回值的生命周期与函数参数的生命周期一样，所以不能在第8行使用result
     // 因为result的生命周期与str2一样在第7行就结束了
}

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}