use std::slice;
unsafe fn dangerous() {}

fn split_at_mut(values: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let len = values.len();
    let ptr = values.as_mut_ptr();

    assert!(mid <= len);

    unsafe {
        (
            slice::from_raw_parts_mut(ptr, mid),
            slice::from_raw_parts_mut(ptr.add(mid), len - mid),
        )
    }
}

unsafe extern "C" {
    fn abs(input: i32) -> i32;
}

static HELLO_WORLD: &str = "Hello, world!";
static mut COUNTER: u32 = 0;

/// SAFETY: 同时在多个线程调用这个方法是未定义的行为，所以你*必须*保证同一时间只
/// 有一个线程在调用它。
fn add_to_count(inc: u32) {
    unsafe {
        COUNTER += inc;
    }
}

fn main() {
    let mut num = 8;
    let r1 = &raw const num;
    let r2 = &raw mut num;
    let address = 0x012345usize;
    let _r = address as *const i32;
    unsafe {
        println!("r1 is: {}", *r1);
        // unsafe中对不可变进行修改依旧会报错
        // *r1 = 9;
        // 下面的指针地址赋值会引发 SEG FAULT
        // *(0x1234 as *mut i32) = 8;
        *r2 = 9;
        println!("r2 is: {}", *r2);
    }

    unsafe {
        dangerous();
    }

    let mut v = vec![1, 2, 3, 4, 5, 6];
    let r = &mut v[..];
    let (a, b) = split_at_mut(r, 3);
    assert_eq!(a, &mut [1, 2, 3]);
    assert_eq!(b, &mut [4, 5, 6]);

    unsafe {
        println!("Absolute value of -3 according to C: {}", abs(-3));
    }

    println!("name is: {HELLO_WORLD}");

    // SAFETY: 它只在 `main` 这一个线程被调用。
    add_to_count(3);
    unsafe {
        // 编译器不允许创建可变static变量的引用!!!
        // println!("COUNTER: {}", COUNTER);
        println!("COUNTER: {}", *(&raw const COUNTER));
    }
}
