use nix::sys::utsname::uname;

pub fn get_kernel_info() -> String {
    uname().release().to_string()
}
