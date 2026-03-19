use omni_runner::system;

#[test]
fn test_get_cpu_usage_returns_valid_range() {
    let cpu = system::get_cpu_usage();
    assert!(cpu >= 0.0 && cpu <= 100.0, "CPU usage {} out of valid range", cpu);
}

#[test]
fn test_get_memory_total_mb_non_zero() {
    let memory = system::get_memory_total_mb();
    assert!(memory > 0, "Memory total {} should be > 0", memory);
}

#[test]
fn test_get_memory_used_mb_reasonable() {
    let used = system::get_memory_used_mb();
    let total = system::get_memory_total_mb();
    assert!(used >= 0, "Memory used {} should be >= 0", used);
    assert!(total > 0, "Memory total {} should be > 0", total);
}

#[test]
fn test_get_disk_used_gb_non_zero() {
    let disk = system::get_disk_used_gb();
    assert!(disk > 0, "Disk used {} should be > 0", disk);
}

#[test]
fn test_get_disk_total_gb_large_enough() {
    let total = system::get_disk_total_gb();
    let used = system::get_disk_used_gb();
    assert!(total > used, "Total disk {} should be > used disk {}", total, used);
}
