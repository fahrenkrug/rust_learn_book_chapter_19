pub fn run() {
    dereferencing_raw_points();
    unsafe_function_example();
    create_safe_abstraction_example();
    extern_example();
    mutate_static_variable_example();
    unsafe_trait_example();
}

fn dereferencing_raw_points() {
    let mut num = 5;
    let r1 = &num as *const i32; // This is an immutable raw pointer
    let r2 = &mut num as *mut i32; // This is a mutable raw pointer
    // Notice: the unsafe keyword isn't used here yet. We only need it for dereferencing.

    unsafe {
        println!("r1 is {}", *r1);
        println!("r2 is {}", *r2);
    }

    // an arbitray location in memory
    let address = 0x012345usize; // As long as we don't read/use this value this doesn't break anything
    let r = address as *const i32;

    unsafe {
        println!("What do we have here? {:?}", r);
    }
}

fn unsafe_function_example() {
    // simply calling dangerous() here won't work
    unsafe {
        dangerous();
    }
}

unsafe fn dangerous() {}

fn create_safe_abstraction_example() {
    // This is the split_at_mut functionality. It can't be implemented with pure safe rust.
    let mut v = vec![1,2,3,4,5,6];
    let r = &mut v[..];
    let (a, b) = r.split_at_mut(3);
    assert_eq!(a, &mut [1,2,3]);
    assert_eq!(b, &mut [4,5,6]);
    let (a, b) = split_at_mut(&mut v, 3);
    assert_eq!(a, &mut [1,2,3]);
    assert_eq!(b, &mut [4,5,6]);

    let address = 0x01234usize;
    let r = address as *mut i32;
    let _sl: &[i32] = unsafe { slice::from_raw_parts_mut(r, 1000) };
    // println!("Hello? {:?}", sl);
    // The last words before the crash
    /*
    What do we have here? 0x12345
    [1]    56493 segmentation fault  cargo run
     */
}

// An approach with just safe rust which won't compile. See the two &mut at the end which are
// borrowed from the same slice.
// We know that this function is okay, because what is borrowed here is different (first half vs.
// second half for example as an example).
/*fn split_at_mut(slice: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let len = slice.len();
    assert!(mid <= len);
    (&mut slice[..mid], &mut slice[mid..])
}*/

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

extern "C" {
    fn abs(input: i32) -> i32;
}

fn extern_example() {
    // Using c
    unsafe {
        println!("The absolute value of -3 according to c is {}", abs(-3));
    }
}

#[no_mangle]
pub extern "C" fn call_from_c() {
    println!("Just called a rust function from c.");
}

static HELLO_WORLD: &str = "Hello world";

static mut COUNTER: u32 = 0;

fn add_to_count(inc: u32) {
    unsafe {
        COUNTER += inc;
    }
}

fn mutate_static_variable_example() {
    println!("Hello {}", HELLO_WORLD); // all good
    add_to_count(3);
    unsafe {
        println!("Counter = {}", COUNTER);
    }
}

unsafe trait Foo {

}

unsafe impl Foo for i32 {

}

fn unsafe_trait_example() {

}