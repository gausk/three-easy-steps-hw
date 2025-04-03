use libc::{fork, getpid, wait, waitpid};
use std::process::exit;

pub fn process() {
    unsafe {

        let pid = fork();
        if pid < 0 {
            eprintln!("Fork failed");
            exit(1);
        } else if pid == 0 {
            let mut status = 0;
            let status = waitpid(-1, &mut status, 0);
            println!("Hello I am child with pid: {} and wait output {status}", getpid());
        } else {
            let mut status = 0;
            let status = waitpid(-1, &mut status, 0);
            println!("goodbye from parent {} to child with pid: {pid} wait output {status}", getpid());
        }
    }
}
