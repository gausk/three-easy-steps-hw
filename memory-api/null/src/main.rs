fn main() {
    let ptr: *const i32 = std::ptr::null();
    unsafe {
        println!("Dereferencing null pointer: {}", *ptr);
    }
}
