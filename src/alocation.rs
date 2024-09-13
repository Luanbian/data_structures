use std::alloc::{alloc, dealloc, Layout};
use std::ptr;

pub fn main() {
    let x = 42; // Stack
    let y = String::from("Hello"); // Heap

    println!("{:p}, {:p}", &x , y.as_ptr());

    unsafe {
        let layout = Layout::new::<i32>();
        let value = alloc(layout);
        if value.is_null() {
            panic!("Failed to allocate memory");
        }

        ptr::write(value, 42);
        println!("Allocated in {:p}, value: {}", value, *value);

        dealloc(value, layout);
    }
}
