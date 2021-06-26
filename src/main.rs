use std::ffi::CStr;
use std::io::Error;

use libc;

const HOSTNAME_BUFFER_SIZE: usize = 1024;

fn main() {
    let mut x: Vec<i8> = Vec::with_capacity(HOSTNAME_BUFFER_SIZE);
    let x_ptr = x.as_mut_ptr();
    unsafe {
        let returncode = libc::gethostname(x_ptr, HOSTNAME_BUFFER_SIZE);
        if returncode == -1 {
            println!(
                "gethostname function call failed with error: {:?}",
                Error::last_os_error()
            );
        }
        let cstr = CStr::from_ptr(x_ptr);
        println!("{}", cstr.to_str().unwrap());
    }
}
