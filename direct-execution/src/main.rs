use libc::{CPU_SET, cpu_set_t, sched_setaffinity, getpid};
use libc::{O_CREAT, O_RDWR, O_TRUNC, S_IRUSR, S_IWUSR, gettimeofday, open, read, timeval, sched_getcpu};
use std::ffi::CString;
use std::process::exit;
use std::sync::mpsc;
use std::thread;
use std::mem;

fn get_time_of_day_precision(iteration: u64) {
    let mut start = timeval {
        tv_sec: 0,
        tv_usec: 0,
    };

    let mut end = timeval {
        tv_sec: 0,
        tv_usec: 0,
    };

    unsafe {
        gettimeofday(&mut start, std::ptr::null_mut());
    }
    for _ in 0..iteration {
        unsafe {
            gettimeofday(&mut end, std::ptr::null_mut());
        }
    }
    unsafe {
        gettimeofday(&mut end, std::ptr::null_mut());
    }

    println!(
        "Average time difference for each iteration of gettimeofday: {0:.3} microseconds",
        (end.tv_usec + end.tv_sec * 1000000 - start.tv_usec - start.tv_sec * 1000000) as f64
            / iteration as f64
    );
}

pub fn system_call_time(iteration: u64) {
    let filename = CString::new("test.txt").expect("CString::new failed");
    let fd = unsafe {
        open(
            filename.as_ptr(),
            O_RDWR | O_CREAT | O_TRUNC,
            S_IRUSR | S_IWUSR,
        )
    };

    if fd < 0 {
        eprintln!("Failed to open file");
        exit(1);
    }
    const BUFFER_SIZE: usize = 1024;
    let mut buffer = [0u8; BUFFER_SIZE];

    let mut start = timeval {
        tv_sec: 0,
        tv_usec: 0,
    };
    unsafe {
        gettimeofday(&mut start, std::ptr::null_mut());
    }

    for _ in 0..iteration {
        unsafe {
            read(fd, buffer.as_mut_ptr() as *mut _, BUFFER_SIZE);
        }
    }

    let mut end = timeval {
        tv_sec: 0,
        tv_usec: 0,
    };
    unsafe {
        gettimeofday(&mut end, std::ptr::null_mut());
    }

    println!(
        "Average time difference of read systemcall: {0:.3} microseconds",
        (end.tv_usec + end.tv_sec * 1000000 - start.tv_usec - start.tv_sec * 1000000) as f64
            / iteration as f64
    );
}

fn pin_to_core(core_id: usize) {
    unsafe {
        let mut cpuset: cpu_set_t = mem::zeroed();
        CPU_SET(core_id, &mut cpuset);

        if sched_setaffinity(getpid(), mem::size_of::<cpu_set_t>(), &cpuset) != 0 {
            panic!("Failed to set thread affinity");
        }
    }
}

fn context_switch_time(iteration: u64, core_id: usize) {
    let mut start = timeval {
        tv_sec: 0,
        tv_usec: 0,
    };
    let (tx1, rx1) = mpsc::channel();
    let (tx2, rx2) = mpsc::channel();

    // Spawn a thread and pin it to the same core
    let _handle = thread::spawn(move || {
        pin_to_core(core_id);

        for _ in 0..iteration {
            let msg = rx1.recv().unwrap();
            tx2.send(msg).unwrap();
        }
    });

    // Pin the main thread to the same core
    pin_to_core(core_id);

    unsafe {
        gettimeofday(&mut start, std::ptr::null_mut());
    }
    for _ in 0..iteration {
        tx1.send(()).unwrap();
        rx2.recv().unwrap();
    }
    let mut end = timeval {
        tv_sec: 0,
        tv_usec: 0,
    };
    unsafe {
        gettimeofday(&mut end, std::ptr::null_mut());
    }

    println!(
        "Average time difference for context switch: {0:.3} microseconds",
        (end.tv_usec + end.tv_sec * 1000000 - start.tv_usec - start.tv_sec * 1000000) as f64
            / (2 * iteration) as f64
    );
}

fn get_current_core() -> usize {
    unsafe { sched_getcpu() as usize }
}


fn main() {
    const ITERATIONS: u64 = 1_000_000;

    get_time_of_day_precision(ITERATIONS);
    system_call_time(ITERATIONS);
    let num_cores = num_cpus::get();
    println!("Number of available cores: {}", num_cores);

    context_switch_time(ITERATIONS, get_current_core())
}
