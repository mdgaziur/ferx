use std::io::Read;

pub fn get_os_info() -> String {
    // try to open /etc/os-release
    let mut os_release = String::new();
    std::fs::File::open("/etc/os-release")
        .unwrap()
        .read_to_string(&mut os_release)
        .unwrap();

    // start getting the os name from " to "
    // hacky way
    // start from index 6
    // and stop on "
    let mut idx = 6;
    let bytes = os_release.as_bytes();
    let mut os_name = String::new();

    loop {
        let ch = bytes[idx] as char;

        if ch == '"' {
            break;
        }
        os_name += &ch.to_string();
        idx += 1;
    }

    os_name
}
