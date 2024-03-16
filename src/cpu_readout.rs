fn process_cpu_name(text: &str) -> String {
    text.replace("(R)", "")
        .replace("(TM)", "")
        .replace(" @ ", "(")
        .replace("CPU", "")
        .replace("GHz", "GHz)")
}

pub fn get() -> String {
    #[cfg(unix)] {
    // TODO: fix indentation hell
        if let Ok(cpuinfo) = std::fs::read_to_string("/proc/cpuinfo") {
            for line in cpuinfo.lines() {
                if line.starts_with("model name") {
                    let parts: Vec<&str> = line.split(':').collect();
                    if parts.len() > 1 {
                        let cpu_name = parts[1].trim();
                        return process_cpu_name(cpu_name);
                    }
                }
            }
        }
        String::new() // can't read /proc/cpuinfo, return an empty string.
    }
    #[cfg(windows)] {
        let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
        if let Ok(subkey) = hklm.open_subkey_with_flags(r#"HARDWARE\DESCRIPTION\System\CentralProcessor\0"#, KEY_READ) {
            if let Ok(cpu_name) = subkey.get_value("ProcessorNameString") {
                return process_cpu_name(cpu_name);
            }
        }

        return String::new();
    }
}
