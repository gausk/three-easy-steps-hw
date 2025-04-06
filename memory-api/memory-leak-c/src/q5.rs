pub fn main() {
    unsafe {
        let ptr = libc::malloc(100 * size_of::<u32>()) as *mut u32;
        if ptr.is_null() {
            panic!("Memory allocation failed");
        }

        *ptr.offset(100) = 42; // Assign a value to the allocated memory
        println!("Value at allocated memory: {}", *ptr.offset(100));
    }
}