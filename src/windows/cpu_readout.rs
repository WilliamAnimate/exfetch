pub fn get() {
    let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
    if let Ok(subkey) = hklm.open_subkey_with_flags(r#"HARDWARE\DESCRIPTION\System\CentralProcessor\0"#, KEY_READ) {
        if let Ok(cpu_name) = subkey.get_value("ProcessorNameString") {
            return exfetch::format_cpu_string(cpu_name);
        }
    }

    return String::new();
}
