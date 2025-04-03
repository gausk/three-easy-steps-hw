use libc::{close, fork, wait, STDOUT_FILENO};
use std::process::exit;

pub fn process() {
    unsafe {

        let pid = fork();
        if pid < 0 {
            eprintln!("Fork failed");
            exit(1);
        } else if pid == 0 {
            let status = close(STDOUT_FILENO);
            println!("child called print and std output file close status is {status}");
        } else {
            let mut status = 0;
            let status = wait(&mut status);
            print!("goodbye from parent to child with pid: {pid} wait output {status}\n");
        }
    }
}
