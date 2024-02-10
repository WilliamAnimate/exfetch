#![allow(unused_must_use)]
use std::{io::{self, Write}, process::Command};
use colored::Colorize;
use tokio::task::spawn;
use tokio::join;

pub mod packages;

macro_rules! write_to_handle_if_not_empty {
    ($handle:expr, $entry:expr, $value:expr) => {
        if $value != "\n" || $value.is_empty() {
            write!($handle, "   {} ~ {}", $entry.purple(), $value);
        }
    }
}
macro_rules! writeln_to_handle_if_not_empty {
    ($handle:expr, $entry:expr, $value:expr) => {
        if $value != "\n" || $value.is_empty() {
            writeln!($handle, "   {} ~ {}", $entry.purple(), $value);
        }
    }
}

#[tokio::main]
async fn main() -> io::Result<()> {
    let name_thread = spawn(async {
        Command::new("sh")
            .args(["-c", "echo $USER"])
            .output()
            .expect("Can't fetch your username")
    });

    let distro_thread = spawn(async {
        let distro_raw = Command::new("sh")
            .args(["-c", "cat /etc/os-release | grep PRETTY_NAME"])
            .output()
            .expect("Can't fetch your distro");

        let distro_output = String::from_utf8(distro_raw.stdout).unwrap();
        let distro_parts: Vec<&str> = distro_output.split("=").collect();
        if let Some(_) = distro_parts.get(1) {
            distro_parts[1].replace("\"", "")
        } else {
            String::new() // an empty string
        }
    });

    let desktop_thread = spawn(async {
        Command::new("sh")
            .args(["-c", "echo $XDG_SESSION_DESKTOP"])
            .output()
            .expect("Can't fetch your desktop")
    });

    let shell_thread = spawn(async {
        Command::new("sh")
            .args(["-c", "echo $SHELL"])
            .output()
            .expect("Can't fetch your shell")
    });

    let packages_thread = spawn(async {
        packages::get_num_packages()
    });

    let arch_thread = spawn(async {
        std::env::consts::ARCH
    });

    // join! to await all `futures` types concurrently
    let (usr, distro, shell, desktop, pkg, arch) = join!(name_thread, distro_thread, shell_thread, desktop_thread, packages_thread, arch_thread);

    // and then .unwrap the results. pray that none of them contain an `Err` type & panic! the app
    // that'd be bad lol
    let usr = usr.unwrap();
    let distro = distro.unwrap();
    let shell = shell.unwrap();
    let desktop = desktop.unwrap();
    let pkg = pkg.unwrap();
    let arch = arch.unwrap();

    let mut handle = io::stdout().lock(); // lock stdout for slightly faster writing
    // the actual printing
    write!(handle, "{}{} - {}", "x".red().bold(), "Fetch".cyan(), String::from_utf8_lossy(&usr.stdout)).unwrap();
    write_to_handle_if_not_empty!(handle, "Shell", String::from_utf8_lossy(&shell.stdout));
    if pkg != 0 { // odd one out; too lazy to properly implement this lol
        writeln!(handle, "   {} ~ {}, {}", "PKGs".purple(), pkg, arch).unwrap();
    } else {
        writeln_to_handle_if_not_empty!(handle, "Arch", arch);
    }
    write_to_handle_if_not_empty!(handle, "Distro", distro);
    write_to_handle_if_not_empty!(handle, "Desktop", String::from_utf8_lossy(&desktop.stdout));

    drop(handle);
    Ok(())
}
