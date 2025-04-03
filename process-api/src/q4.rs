use std::ffi::CString;
use libc::{fork, execl, execle, execlp, execv, c_char};
use std::process::exit;
use std::ptr;

pub fn process() {
    unsafe {
        let pid = fork();
        if pid < 0 {
            eprintln!("Fork failed");
            exit(1);
        } else if pid == 0 {
            let mut arg0 = CString::new("/bin/ls").unwrap();
            let arg1 = CString::new("/home/gauravkumar/workspace").unwrap();
            
            //execl(arg0.as_ptr(), arg0.as_ptr(), arg1.as_ptr(), ptr::null::<c_char>());

            //let envp: [*const c_char; 1] = [ptr::null::<c_char>()];
            //execle(arg0.as_ptr(), arg0.as_ptr(), arg1.as_ptr(), ptr::null::<c_char>(), envp.as_ptr());

            //arg0 = CString::new("ls").unwrap();
            //execlp(arg0.as_ptr(), arg0.as_ptr(), arg1.as_ptr(), ptr::null::<c_char>());

            let arglist = [arg0.as_ptr(), arg1.as_ptr(), ptr::null::<c_char>()];
            execv(arg0.as_ptr(), arglist.as_ptr());


            print!("This should not print in child");
        } else {
            println!("I'm the parent and I'm just printing something.");
        }
    }
}
