use std::io::Read;
use regex::Regex;

pub fn get_cpu_model_name() -> String {
    let mut cpuinfo = String::new();
    std::fs::File::open("/proc/cpuinfo").unwrap().read_to_string(&mut cpuinfo).unwrap();

    // do regex shenanigans
    let re = Regex::new(r"model name\t: (.*)").unwrap();
    let caps = re.captures(&cpuinfo).unwrap();
    caps[1].to_string()
}