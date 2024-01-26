// This is a fork from Fetchit for xFetch.

use std::process::{Command, Stdio};

pub fn get_num_packages() -> i16 {
    let num_packages = packages_generic("pacman", &["-Q"])
        .or_else(|_| packages_generic("yum", &["list", "installed"]))
        .or_else(|_| packages_generic("apt", &["list", "--installed"]))
        .or_else(|_| packages_generic("pkg", &["info", "-a"]))
        .or_else(|_| packages_generic("xbps-query", &["-l"]))
        .or_else(|_| packages_generic("rpm", &["-qa"]))
        .unwrap_or_else(|_| "0".to_string());


    // Count the total number of packages
    num_packages.lines().count() as i16
}

pub fn packages_generic(cmd: &str, options: &[&str]) -> Result<String, String> {
    let packages = Command::new(cmd).args(options).output();
    match packages {
        Ok(x) => Ok(String::from_utf8(x.stdout).unwrap()),
        Err(e) => Err(e.to_string()),
    }
}
