// use std::io::{{Write::BufRead}, fs::File, BufReader};
use std::{io::{BufRead, BufReader}, fs::File};

pub fn get() -> String {
    #[cfg(unix)] {
        let file = File::open("/etc/os-release").expect("Can't open /etc/os-release!");
        let mut reader = BufReader::new(file);
        let (mut line, mut pretty_name) = (String::new(), String::new());

        while reader.read_line(&mut line).expect("Failed to read line") > 0 {
            if line.starts_with("PRETTY_NAME=") {
                pretty_name = line.split_once('=').unwrap().1.to_string();
                pretty_name = pretty_name.trim().trim_matches('"').to_string();
                break;
            }
            line.clear();
        }
        pretty_name
    }
    #[cfg(windows)] {
        let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
        let subkey = hklm.open_subkey_with_flags(r#"SOFTWARE\Microsoft\Windows NT\CurrentVersion"#, KEY_READ).unwrap();
        let mut version: String = subkey.get_value("ProductName").unwrap();
        let current_build: String = subkey.get_value("CurrentBuild").unwrap();
        let display_version: String = subkey.get_value("DisplayVersion").unwrap();

        // remove pro/enterprise/home/etc from the version
        version = version.replace(" Pro", "").replace(" Home", "").replace(" Enterprise", "");

        format!("{}, {} (build {})", version, display_version, current_build)
    }
}

