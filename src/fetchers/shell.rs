use libc::getpwuid;
use nix::unistd::geteuid;
use std::ffi::CStr;

pub fn get_shell() -> String {
    let user = unsafe { getpwuid(u32::from(geteuid())).as_ref() }; // what if it's null?
    if user.is_none() {
        return String::from("Not available");
    }

    unsafe { CStr::from_ptr(user.unwrap().pw_shell).to_str().unwrap().to_string() }
}
