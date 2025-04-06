fn main() {
    let leak = Box::new([0u8; 100]);  // Allocates heap memory
    std::mem::forget(leak);           // Leaks it by preventing deallocation
}
