use std::{io::{BufReader, BufRead}, fs::File};
use exfetch::prelude::*;
pub fn get() -> String {
    // TODO: fix indentation hell
    let file = File::open("/proc/cpuinfo").unwrap();
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line = unsafe { line.unwrap_unchecked() };
        if !line.starts_with("model name") {
            continue;
        }
        let parts: Vec<&str> = line.split(':').collect();
        if parts.is_empty() {
            continue;
        }
        let cpu_name = parts[1].trim();
        return format_cpu_string(cpu_name);
    }

    String::new() // can't read /proc/cpuinfo, return an empty string.
}
