use std::ffi::CString;
use std::io::{self, Write};
use std::os::unix::io::RawFd;
use std::process;
use libc::{c_int, close, dup2, execvp, fork, pipe, read, waitpid, STDIN_FILENO, STDOUT_FILENO};

pub fn process() {
    unsafe {
        let mut pipe_fds: [c_int; 2] = [0; 2];

        // Create a pipe
        if pipe(pipe_fds.as_mut_ptr()) == -1 {
            eprintln!("Failed to create pipe");
            process::exit(1);
        }

        let read_end: RawFd = pipe_fds[0];
        let write_end: RawFd = pipe_fds[1];

        // Fork the first child
        let pid1 = fork();
        if pid1 == -1 {
            eprintln!("Failed to fork process");
            process::exit(1);
        }

        if pid1 == 0 {
            // In the first child
            close(read_end);
            dup2(write_end, STDOUT_FILENO); // Redirect stdout to the pipe
            close(write_end);

            print!("First child says hello.");
        } else {

            // Fork the second child
            let pid2 = fork();
            if pid2 == -1 {
                eprintln!("Failed to fork process");
                process::exit(1);
            }

            if pid2 == 0 {
                // In the second child
                close(write_end);
                dup2(read_end, STDIN_FILENO); // Redirect stdin to read from the pipe
                close(read_end);

                const BUFFER_SIZE: usize = 1024;
                let mut buffer = [0u8; BUFFER_SIZE];
                let bytes_read = read(0, buffer.as_mut_ptr() as *mut _, BUFFER_SIZE);
                if bytes_read > 0 {
                    let output = String::from_utf8_lossy(&buffer[..bytes_read as usize]);
                    println!("Child 2 received: {}", output);
                }
            } else {

                // In the parent process
                close(read_end);
                close(write_end);

                waitpid(pid1, std::ptr::null_mut(), 0); // Wait for first child
                waitpid(pid2, std::ptr::null_mut(), 0); // Wait for second child
            }
        }
    }
}
