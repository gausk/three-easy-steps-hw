use libc::{fork, getpid};
use std::process::exit;

pub fn process() {
    unsafe {
        let mut x = 100;
        println!("I am process {} and the value of x is {x}", getpid());
        let pid = fork();
        if pid < 0 {
            eprintln!("Fork failed");
            exit(1);
        } else if pid == 0 {
            // Child process
            x += 20;
            println!("Child process (PID: {}) and the value of {x}", getpid());
        } else {
            // Parent process
            x += 10;
            println!("Parent process (PID: {}) and the value of {x}", getpid());
        }

        for _ in 1..5 {
            println!("process {} and the value of {x}", getpid());
            std::thread::sleep(std::time::Duration::from_secs(1));
        }
    }
}
