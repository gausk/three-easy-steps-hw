use libc::{fork, getpid, wait};
use std::process::exit;

pub fn process() {
    unsafe {

        let pid = fork();
        if pid < 0 {
            eprintln!("Fork failed");
            exit(1);
        } else if pid == 0 {
            let mut status = 0;
            let status = wait(&mut status);
            println!("Hello I am child with pid: {} and wait output {status}", getpid());
        } else {
            let mut status = 0;
            let status = wait(&mut status);
            print!("goodbye from parent to child with pid: {pid} wait output {status}\n");
        }
    }
}
