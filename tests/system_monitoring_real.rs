use omni_runner::system;

#[test]
fn test_real_system_monitoring_cpu() {
    let cpu = system::get_cpu_usage();
    assert!(cpu >= 0.0);
    assert!(cpu <= 100.0);
    assert!(cpu.is_finite());
}

#[test]
fn test_real_system_monitoring_memory() {
    let used = system::get_memory_used_mb();
    let total = system::get_memory_total_mb();
    
    assert!(used >= 0);
    assert!(total > 0);
}

#[test]
fn test_real_system_monitoring_disk() {
    let used = system::get_disk_used_gb();
    let total = system::get_disk_total_gb();
    
    assert!(used > 0);
    assert!(total > 0);
    assert!(used < total);
}

#[test]
fn test_real_system_monitoring_multiple_reads() {
    let cpu1 = system::get_cpu_usage();
    std::thread::sleep(std::time::Duration::from_millis(50));
    let cpu2 = system::get_cpu_usage();
    
    assert!(cpu1.is_finite());
    assert!(cpu2.is_finite());
    assert!((cpu1..=100.0).contains(&cpu2));
}

#[test]
fn test_real_system_monitoring_consistency() {
    let cpu = system::get_cpu_usage();
    let mem_used = system::get_memory_used_mb();
    let mem_total = system::get_memory_total_mb();
    let disk_used = system::get_disk_used_gb();
    let disk_total = system::get_disk_total_gb();
    
    assert!((0.0..=100.0).contains(&cpu));
    assert!((disk_used < disk_total) || (disk_used > 0 && disk_total > 0));
}

#[test]
fn test_real_system_resources_struct() {
    let sys_res = omni_runner::models::SystemResources {
        cpu_percent: system::get_cpu_usage(),
        memory_used_mb: system::get_memory_used_mb(),
        memory_total_mb: system::get_memory_total_mb(),
        disk_used_gb: system::get_disk_used_gb(),
        disk_total_gb: system::get_disk_total_gb(),
    };
    
    assert!((0.0..=100.0).contains(&sys_res.cpu_percent));
    assert!(sys_res.memory_used_mb >= 0);
}

#[test]
fn test_real_heartbeat_with_real_resources() {
    let heartbeat = omni_runner::heartbeat::create_heartbeat("real-test-device").unwrap();
    
    assert!((0.0..=100.0).contains(&heartbeat.system_resources.cpu_percent));
    assert!(heartbeat.system_resources.memory_total_mb > 0);
    assert!(heartbeat.system_resources.disk_total_gb > 0);
}

#[test]
fn test_real_system_default_values() {
    let cpu = system::get_cpu_usage();
    let mem_total = system::get_memory_total_mb();
    let disk_total = system::get_disk_total_gb();
    
    assert!(cpu >= 0.0);
    assert!(mem_total > 0);
    assert!(disk_total > 0);
}
