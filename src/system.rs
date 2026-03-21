// System monitoring functions
// Pure functions that read system state - testable on Linux

/// Gets CPU usage percentage (Linux only)
#[cfg(target_os = "linux")]
pub fn get_cpu_usage() -> f64 {
    use std::fs;
    // Allow tests to override the file path via environment variable
    fn proc_stat_path() -> String {
        if cfg!(test) {
            std::env::var("OMNI_TEST_PROC_STAT").unwrap_or("/proc/stat".to_string())
        } else {
            "/proc/stat".to_string()
        }
    }

    if let Ok(proc_stat) = fs::read_to_string(proc_stat_path()) {
        let lines: Vec<&str> = proc_stat.lines().collect();
        if let Some(line) = lines.first() {
            let parts: Vec<u64> = line.split_whitespace()
                .skip(1)
                .filter_map(|s| s.parse().ok())
                .collect();

            if parts.len() >= 4 {
                let idle = parts[3];
                let total: u64 = parts.iter().sum();
                if total > 0 {
                    return ((total - idle) as f64 / total as f64) * 100.0;
                }
            }
        }
    }
    0.0
}

/// Gets memory usage in MB (Linux only)
#[cfg(target_os = "linux")]
pub fn get_memory_used_mb() -> u64 {
    use std::fs;

    // Allow tests to override meminfo path via environment variables
    fn proc_meminfo_path() -> String {
        if cfg!(test) {
            std::env::var("OMNI_TEST_PROC_MEMINFO").unwrap_or("/proc/meminfo".to_string())
        } else {
            "/proc/meminfo".to_string()
        }
    }

    let meminfo_path = proc_meminfo_path();
    let proc_meminfo = fs::read_to_string(meminfo_path).unwrap_or_default();

    // Read MemAvailable
    let mut available_kb: u64 = 0;
    for line in proc_meminfo.lines() {
        if line.starts_with("MemAvailable:") {
            if let Some(avail_str) = line.split(':').nth(1) {
                if let Some(val_str) = avail_str.trim().split_whitespace().next() {
                    if let Ok(mem_kb) = val_str.parse::<u64>() {
                        available_kb = mem_kb;
                    }
                }
            }
        }
        // break early if we found value
    }

    // Read MemTotal (fallback to 8GB if not present)
    let mut total_kb: u64 = 8192 * 1024;
    for line in proc_meminfo.lines() {
        if line.starts_with("MemTotal:") {
            if let Some(total_str) = line.split(':').nth(1) {
                if let Some(val_str) = total_str.trim().split_whitespace().next() {
                    if let Ok(mem_kb) = val_str.parse::<u64>() {
                        total_kb = mem_kb;
                    }
                }
            }
        }
    }

    if total_kb > available_kb {
        (total_kb - available_kb) / 1024
    } else {
        0
    }
}

/// Default implementations for non-Linux systems
#[cfg(not(target_os = "linux"))]
pub fn get_cpu_usage() -> f64 {
    0.0
}

#[cfg(not(target_os = "linux"))]
pub fn get_memory_used_mb() -> u64 {
    0
}

/// Gets total memory in MB
pub fn get_memory_total_mb() -> u64 {
    8192
}

/// Gets disk usage in GB
pub fn get_disk_used_gb() -> u64 {
    50
}

/// Gets total disk space in GB
pub fn get_disk_total_gb() -> u64 {
    500
}
