use std::env;
use std::thread;
use std::time::Duration;

fn main() {
    // Read memory size in MB from command-line argument
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <memory_in_mb>", args[0]);
        return;
    }

    let memory_mb: usize = args[1].parse().expect("Invalid number");
    let total_bytes = memory_mb * 1024 * 1024;

    // Allocate a vector of u8s (each u8 = 1 byte)
    let mut buffer = vec![0u8; total_bytes];

    for i in (0..total_bytes).step_by(4096) {
        buffer[i] = 1;
    }

    println!("Allocated and touched {} MB of memory.", memory_mb);

    loop {
        
    }
}
