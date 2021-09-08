use nix::sys::utsname::uname;

pub fn get_hostname() -> String {
    uname().nodename().to_string()
}
