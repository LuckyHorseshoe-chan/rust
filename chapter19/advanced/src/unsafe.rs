let mut num = 5;

// raw pointers
let r1 = &num as *const i32;
let r2 = &mut num as *mut i32;

// dereferencing raw pointers within an unsafe block
unsafe {
    println!("r1 is: {}", *r1);
    println!("r2 is: {}", *r2);
}

// using extern functions to call external code
extern "C" {
    fn abs(input: i32) -> i32;
}
unsafe {
    println!("Absolute value of -3 according to C: {}", abs(-3));
}

// reading from or writing to a mutable static variable is unsafe
static mut COUNTER: u32 = 0;
fn add_to_count(inc: u32) {
    unsafe {
        COUNTER += inc;
    }
}
add_to_count(3);
unsafe {
    println!("COUNTER: {}", COUNTER);
}

// defining and implementing an unsafe trait
unsafe trait Foo {
    // methods go here
}

unsafe impl Foo for i32 {
    // method implementations go here
}