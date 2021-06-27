use std::ffi::CStr;
use std::io::Error;

use libc;

const HOSTNAME_BUFFER_SIZE: usize = 1024;

fn main() {
    let mut buffer: Vec<i8> = Vec::with_capacity(HOSTNAME_BUFFER_SIZE);
    let buffer_ptr = buffer.as_mut_ptr();
    unsafe {
        let returncode = libc::gethostname(buffer_ptr, HOSTNAME_BUFFER_SIZE);
        if returncode == -1 {
            println!(
                "gethostname system call failed with error: {:?}",
                Error::last_os_error()
            );
        }
        let cstr = CStr::from_ptr(buffer_ptr);
        println!("{}", cstr.to_str().unwrap());
    }
}
