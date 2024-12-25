use tokio::{io::{BufReader, AsyncBufReadExt}, fs::File};

pub async fn get() -> String {
    #[cfg(target_os = "macos")] {
        "MacOS".to_owned()
    }
    #[cfg(target_os = "android")] {
        "Android".to_owned()
    }
    #[cfg(target_os = "ios")] {
        "iOS".to_owned()
    }
    #[cfg(target_os = "linux")] {
        let file = File::open("/etc/os-release").await;
        if !file.is_ok() {
            // FIXME: somehow inline within a .unwrap_or_else()?
            return String::new();
        }
        let mut reader = BufReader::new(file.unwrap());
        let (mut line, mut pretty_name) = (String::new(), String::new());

        while reader.read_line(&mut line).await.expect("Failed to read line") > 0 {
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

