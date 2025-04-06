pub fn main() {
    unsafe {
        let ptr = libc::malloc(100 * size_of::<u32>()) as *mut u32;
        if ptr.is_null() {
            panic!("Memory allocation failed");
        }

        *ptr = 42; // Assign a value to the allocated memory
        libc::realloc(ptr as *mut libc::c_void, 5000 * size_of::<u32>());
        println!("Value at allocated memory: {}", *ptr);
    }
}