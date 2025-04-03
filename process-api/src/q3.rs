use libc::{fork, wait};
use std::process::exit;

pub fn process() {
    unsafe {

        let pid = fork();
        if pid < 0 {
            eprintln!("Fork failed");
            exit(1);
        } else if pid == 0 {
            print!("hello\n");
        } else {
            let mut status = 0;
            wait(&mut status);
            print!("goodbye\n");
        }
    }
}
