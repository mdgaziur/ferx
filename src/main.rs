pub mod fetchers;
pub mod sysinfo;
pub mod external;

fn main() {
    let info = sysinfo::SystemInformation::get();

    let args = std::env::args().collect::<Vec<String>>();

    let mut art;

    if !args.contains(&String::from("--universal")) {
        // try to get the art for current os
        art = std::fs::read_to_string(format!("{}/.config/ferx/arts/{}.txt", std::env::var("HOME").unwrap(), info.os_name));
        if art.is_err() {
            art = std::fs::read_to_string(format!("{}/.config/ferx/arts/{}.txt", std::env::var("HOME").unwrap(), "universal"));
        }
    } else {
        art = std::fs::read_to_string(format!("{}/.config/ferx/arts/{}.txt", std::env::var("HOME").unwrap(), "universal"));
    }

    println!("\x1b{}\x1b[1;0m\n\n", art.unwrap());
    println!("    OS:               {}", info.os_name);
    println!("    Host:             {}", info.host_name);
    println!("    Kernel:           {}", info.kernel);
    println!("    Uptime:           {}", info.uptime);
    println!("    Shell:            {}", info.shell);
    println!("    Resolution:       {}", info.resolution);
    println!("    CPU:              {}", info.cpu);
    println!("    GPU:              {}", info.gpu);
    println!("    Memory:           {}", info.memory);
}
