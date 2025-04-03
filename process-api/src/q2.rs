use std::process::exit;
use std::ffi::CString;
use libc::{fork, getpid, open, O_CREAT, O_RDWR, O_TRUNC, S_IRUSR, S_IWUSR, write, close};

pub fn process() {
    unsafe {
        let filename = CString::new("test.txt").expect("CString::new failed");
        let fd = open(filename.as_ptr(), O_RDWR | O_CREAT | O_TRUNC, S_IRUSR | S_IWUSR);

        if fd < 0 {
            eprintln!("Failed to open file");
            exit(1);
        }

        println!("I am process {}", getpid());
        let pid = fork();
        if pid < 0 {
            eprintln!("Fork failed");
            exit(1);
        } else if pid == 0 {
            // Child process
            println!("Child process (PID: {})", getpid());
            write_to_file(fd, "hello from child\n");
        } else {
            // Parent process
            println!("Parent process (PID: {}) child: {pid}", getpid());
            write_to_file(fd, "hello from parent\n");
        }
    }
}


fn write_to_file(fd: i32, content: &str) {
    let bytes_written = unsafe { write(fd, content.as_ptr() as *const _, content.len()) };
    if bytes_written < 0 {
        eprintln!("Failed to write to file");
    } else {
        println!("Wrote {} bytes to file", bytes_written);
    }
    unsafe { close(fd); }
}