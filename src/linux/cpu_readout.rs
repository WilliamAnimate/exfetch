pub fn get() -> String {
    // TODO: fix indentation hell
    // TODO(perf): use bufreader here
    let cpuinfo = std::fs::read_to_string("/proc/cpuinfo").unwrap_or_else(|_| return String::new());

    for line in cpuinfo.lines() {
        if line.starts_with("model name") {
            let parts: Vec<&str> = line.split(':').collect();
            if parts.len() > 1 {
                let cpu_name = parts[1].trim();
                return exfetch::format_cpu_string(cpu_name);
            }
        }
    }
    String::new() // can't read /proc/cpuinfo, return an empty string.
}
