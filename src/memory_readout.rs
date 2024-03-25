pub fn format_memory_from_bytes(mem: u64) -> String {
    let total = mem / 1024000;
    let mut output: String;
    if total != 0 {
        output = total.to_string();
        output.push_str(" MB");
    } else {
        output = String::from("Disabled"); // swap-file only.
    }

    // note: the sysinfo struct does not expose enough useful info to determine the amount of
    // available ram. the next best option is `free` ram (unused ram) but it isn't the same thing.
    output
}

