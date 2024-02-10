#![allow(unused_must_use)]
use std::{io::{self, Write, BufRead}, fs::File, process::Command};
use colored::Colorize;
use tokio::{task::spawn, join};

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
        let file = File::open("/etc/os-release").expect("can't open /etc/os-release");
        let mut reader = io::BufReader::new(file);
        let mut line = String::new();
        let mut pretty_name = String::new();

        while reader.read_line(&mut line).expect("failed to read line") > 0 {
            if line.starts_with("PRETTY_NAME=") {
                pretty_name = line.splitn(2, '=').nth(1).unwrap().to_string();
                pretty_name = pretty_name.trim().trim_matches('"').to_string();
                break;
            }
            line.clear();
        }
        pretty_name
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
    writeln_to_handle_if_not_empty!(handle, "Distro", distro);
    write_to_handle_if_not_empty!(handle, "Desktop", String::from_utf8_lossy(&desktop.stdout));

    drop(handle);
    Ok(())
}
