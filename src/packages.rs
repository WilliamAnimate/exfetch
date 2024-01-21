// @Author: Ruturajn <nanotiruturaj@gmail.com>
// This is a fork for xFetch

use std::process::{Command, Stdio}; // For executing shell commands.

pub fn get_num_packages() -> u32 {
    let num_packages = packages_generic("pacman", &["-Q"])
        .or_else(|_| packages_generic("yum", &["list", "installed"]))
        .or_else(|_| packages_generic("dpkg-query", &["-l"]))
        .or_else(|_| packages_generic("pkg", &["info"]))
        .or_else(|_| packages_generic("ls", &["-d", "var/db/pkg/*/*"]))
        .or_else(|_| packages_generic("ls", &["-d", "/var/lib/scratchpkg/db/*"]))
        .or_else(|_| packages_generic("ls", &["/var/lib/eopkg/package/"]))
        .or_else(|_| packages_generic("xbps-query", &["-l"]))
        .or_else(|_| packages_generic("rpm", &["-qa"]))
        .or_else(|_| packages_nixos_based())
        .unwrap_or_else(|_| "Unknown".to_string());

    // Count the total number of packages
    let mut total_count: u32 = 0;
    for _ in num_packages.lines() {
        total_count += 1;
    }

    total_count
}

pub fn packages_generic(cmd: &str, options: &[&str]) -> Result<String, String> {
    // Use `pkg info` to list@ext:GitHub.vscode-pull-request-github remotes the installed packages.
    let packages = Command::new(cmd).args(options).output();

    // Check if the above command executed, successfully,
    // if so, unwrap the output from stdout, and return it.
    match packages {
        Ok(x) => Ok(String::from_utf8(x.stdout).unwrap()),
        Err(e) => Err(e.to_string()),
    }
}

pub fn packages_nixos_based() -> Result<String, String> {
    // Use `nix-store -qR /run/current-system/sw/ 2>/dev/null && nix-store -qR ~/.nix-profile/`
    // to get the list of installed packages. So, we will first get the output for the
    // first command in the shell chain.
    let packages = Command::new("nix-store")
        .args(["-qR", "/run/current-system/sw/"])
        .output();

    match packages {
        Ok(x) => {
            // Once the first chain command succeeds, we will add it's output,
            // to the second chain command.
            let packages_output = String::from_utf8(x.stdout).unwrap();
            match Command::new("nix-store")
                .args(["-qR", "~/.nix-profile/"])
                .output()
            {
                Ok(y) => {
                    let prev_output = String::from_utf8(y.stdout).unwrap();
                    Ok(format!("{}{}", packages_output, prev_output))
                }
                Err(e) => Err(e.to_string()),
            }
        }
        Err(e) => Err(e.to_string()),
    }
}
