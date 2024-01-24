use std::{process::Command, path::Path, fs::read_dir};

pub fn get_num_packages() -> i16 {
    let entries = read_dir(Path::new("/var/lib/pacman/local")).expect("compiler forced me to add this.").count();

    (entries - 1).try_into().unwrap() // `as i16` didn't work, listening to the compiler now.
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
