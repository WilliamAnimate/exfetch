use std::{process::Command, path::Path, fs::read_dir};

macro_rules! count_files_in_directory {
    ($path:expr) => {
        std::fs::read_dir(std::path::Path::new($path)).expect("").count() as i16
    };
}

fn file_exists_in_bin(file_to_search: &str) -> bool {
    Path::new(&format!("/bin/{}", file_to_search)).exists()
            // ^ wtf does borrowing here mean?
}

pub fn get_num_packages() -> i16 {
    /*
     * ubuntu (debian based)    /var/cache/apt/archives/
     * redhat/centos/fedora     /var/cache/dnf/ and/or /var/cache/yum/
     * opensuse (zypper)        /var/cache/zypp/packages/
     * N.B. for gentoo, you will have to use the package manager or something (i never used gentoo) and figure it out
     * from there, as it doesnt store anything in specific spots.
     */

    // FIXME: this is the ugliest shit ever
    if file_exists_in_bin("pacman") {
        return count_files_in_directory!("/var/lib/pacman/local");
    } else if file_exists_in_bin("yum") {
        return count_files_in_directory!("/var/cache/yum");
    } else if file_exists_in_bin("dnf") {
        return count_files_in_directory!("/var/cache/dnf");
    } else if file_exists_in_bin("zypper") {
        return count_files_in_directory!("/var/cache/zypp/packages");
    } else if file_exists_in_bin("apt") { // i have no clue if this is right or not, i use arch not
                                          // debian.
        return count_files_in_directory!("/var/cache/apt/archives");
    } else {
        0 // TODO: make this report "unknown"
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
