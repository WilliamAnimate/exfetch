use tokio::fs::File;

#[cfg(unix)]
static INIT_SYSTEMS_UNIX: &[(&str, &str)] = &[
    #[cfg(target_os = "linux")]
    ("/usr/lib/systemd/systemd", "systemd"), // ew
    ("/sbin/openrc", "openrc"),
    ("/sbin/dinit", "dinit"),
];

#[allow(unreachable_code)]
pub async fn get() -> String {
    #[cfg(target_os = "macos")] {
        // macos is a type of unix. if under macos, return launchd now.
        // ordering matters btw. keep this above other unix directives.
        return "launchd".to_string();
    }
    #[cfg(target_os = "android")] {
        // android is a type of unix, more specifically, linux. they have their own init
        return "init.rc".to_string();
    }
    #[cfg(unix)] {
        // other types of unixes, eg *BSD, Linux, etc
        for (init_path, init_name) in INIT_SYSTEMS_UNIX {
            if File::open(init_path).await.is_err() {
                continue;
            }
            return (*init_name).to_string();
        }

        // FALLBACK: if we cannot find the init, return the contents of /proc/1/comm
        // we dont do this by default because on openrc, /proc/1/comm is `init`. very descriptive.
        use tokio::io::{AsyncReadExt, BufReader};
        let file = File::open("/proc/1/comm").await;
        if file.is_err() {
            return String::new();
        }

        let mut reader = BufReader::new(file.unwrap());
        let mut buf = String::new();
        reader.read_to_string(&mut buf);
        buf
    }
    #[cfg(windows)] {
        "wininit".into()
    }
}

