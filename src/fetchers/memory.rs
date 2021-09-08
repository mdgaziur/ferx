use crate::external::meminfo;

pub fn get_memory_info() -> String {
    let meminfo = meminfo().unwrap();

    let total_mem = meminfo.total;
    let used_mem = total_mem - meminfo.free - meminfo.buffers - meminfo.cached;

    format!("{}MB / {}MB", used_mem / (1024 * 1024), total_mem / (1024 * 1024))
}