use crate::fetchers::os_info::get_os_info;
use crate::fetchers::host::get_hostname;
use crate::fetchers::kernel::get_kernel_info;
use crate::fetchers::uptime::get_uptime;
use crate::fetchers::shell::get_shell;
use crate::fetchers::resolution::get_resolution;
use crate::fetchers::cpu::get_cpu_model_name;
use crate::fetchers::gpu::get_gpu;
use crate::fetchers::memory::get_memory_info;

#[derive(Debug)]
pub struct SystemInformation {
    pub os_name: String,
    pub host_name: String,
    pub kernel: String,
    pub uptime: String,
    pub shell: String,
    pub resolution: String,
    pub cpu: String,
    pub gpu: String,
    pub memory: String,
}

impl SystemInformation {
    pub fn get() -> Self {
        Self {
            os_name: get_os_info(),
            host_name: get_hostname(),
            kernel: get_kernel_info(),
            uptime: get_uptime(),
            shell: get_shell(),
            resolution: get_resolution(),
            cpu: get_cpu_model_name(),
            gpu: get_gpu(),
            memory: get_memory_info()
        }
    }
}