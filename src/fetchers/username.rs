use std::ffi::CStr;
use libc::getpwuid;
use nix::unistd::getuid;

pub fn get_username() -> String {
    let uname = unsafe { getpwuid(getuid().as_raw()).as_ref() };

    if uname.is_none() {
        return String::from("user");
    }

    unsafe { CStr::from_ptr(uname.unwrap().pw_name) }.to_str().unwrap().to_string()
}