// original repo was removed
// https://github.com/danclive/meminfo

use std::fs;
use std::io::{self};

#[cfg(feature = "serde")]
use serde::{Serialize, Deserialize};

#[cfg(feature = "serde")]
#[derive(Debug, Default, Eq, PartialEq, Copy, Clone, Serialize, Deserialize)]
pub struct Memory {
    pub total: u64,  // MemTotal
    pub free: u64,  // MemFree
    pub available: u64, // MemAvailable
    pub buffers: u64, // Buffers
    pub cached: u64, // Cached
    pub active: u64,  // Active
    pub inactive: u64, // Inactive
    pub swap_total: u64, // SwapTotal
    pub swap_free: u64, // SwapFree
    pub shared: u64, // Shmem
}

#[cfg(not(feature = "serde"))]
#[derive(Debug, Default, Eq, PartialEq, Copy, Clone)]
pub struct Memory {
    pub total: u64,  // MemTotal
    pub free: u64,  // MemFree
    pub available: u64, // MemAvailable
    pub buffers: u64, // Buffers
    pub cached: u64, // Cached
    pub active: u64,  // Active
    pub inactive: u64, // Inactive
    pub swap_total: u64, // SwapTotal
    pub swap_free: u64, // SwapFree
    pub shared: u64, // Shmem
}

pub fn meminfo() -> io::Result<Memory> {
    let mut memory = Memory::default();

    let content = fs::read_to_string("/proc/meminfo")?;

    for line in content.lines() {
        let first_bytes = &line.as_bytes()[..2];
        match first_bytes {
            b"Me" | b"Ac" | b"In" | b"Bu" | b"Ca" | b"Sh" | b"Sw" => {},
            _ => continue,
        };

        let mut parts = line.splitn(2, ':');
        let field = match parts.next() {
            Some("MemTotal") => &mut memory.total,
            Some("MemFree") => &mut memory.free,
            Some("MemAvailable") => &mut memory.available,
            Some("Buffers") => &mut memory.buffers,
            Some("Cached") => &mut memory.cached,
            Some("Active") => &mut memory.active,
            Some("Inactive") => &mut memory.inactive,
            Some("SwapTotal") => &mut memory.swap_total,
            Some("SwapFree") => &mut memory.swap_free,
            Some("Shmem") => &mut memory.shared,
            _ => continue,
        };

        match parts.next() {
            Some(value) => *field = {
                let bytes = match value.trim_start().splitn(2, ' ').next() {
                    Some(kbytes) => {
                        if let Ok(value) = kbytes.parse::<u64>() {
                            value * 1000
                        } else {
                            continue
                        }
                    }
                    None => continue
                };

                bytes
            },
            None => continue
        }
    }

    Ok(memory)
}
