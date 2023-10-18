mod front_of_house {
    pub mod hosting { // 函数、方法、mod、变量等默认是私有的，要使用pub暴露
        pub fn add_to_waitlist() {}
    }
}

pub fn eat_at_restaurant() {
    crate::front_of_house::hosting::add_to_waitlist();

    front_of_house::hosting::add_to_waitlist();
}