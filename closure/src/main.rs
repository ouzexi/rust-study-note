use std::slice;

fn split_at_mut(slice: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let len = slice.len();
    let ptr = slice.as_mut_ptr();

    assert!(mid <= len);

    unsafe {
        (
            slice::from_raw_parts_mut(ptr, mid),
            slice::from_raw_parts_mut(ptr.add(mid), len - mid),
        )
    }
}

extern "C" { // ABI
    fn abs(input: i32) -> i32;
}

static mut COUNTER: u32 = 0;

fn add_to_count(inc: u32) {
    unsafe {
        COUNTER += inc;
    }
}

unsafe trait Foo {

}

unsafe impl Foo for i32 {
    
}

fn main() {
    // Unsafe Rust没有强制内存安全保证，可执行4个动作
    // 1 解引用原始指针
    let mut num = 5;
    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;
    unsafe {
        println!("r1: {}", *r1);
        println!("r2: {}", *r2);
    }


    // 2 调用unsafe函数或方法
    let address = 0x012345usize;
    let r = address as *mut i32;

    let slice: &[i32] = unsafe {
        slice::from_raw_parts_mut(r, 10000) // 不安全，无法保证有10000个元素
    };



    unsafe {
        println!("Absolute value of -3 according to C: {}", abs(-3));
    }
    // 3 访问或修改可变的静态变量
    add_to_count(3);
    unsafe {
        println!("COUNTER: {}", COUNTER);
    }

    // 4 实现unsafe trait
}