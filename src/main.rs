// idk, stuff ////////////////
#![allow(unused_doc_comments)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_must_use)]
//////////////////////////////

use std::{io::{self, Write}, process::Command};
use colored::Colorize;
use tokio::task::spawn;

pub mod packages;

#[tokio::main]
async fn main() -> io::Result<()> {
    // usr /////////////////////////////////////
    let name_thread = spawn(async {
        Command::new("sh")
            .args(["-c", "echo $USER"])
            .output()
            .expect("Can't fetch your username")
    });
    ////////////////////////////////////////////
    // OS Related ////////////////////////////////////////////////////////
    let distro_thread = spawn(async {
        let distro_raw = Command::new("sh")
            .args(["-c", "cat /etc/os-release | grep PRETTY_NAME"])
            .output()
            .expect("Can't fetch your distro");

        let distro_output = String::from_utf8(distro_raw.stdout).unwrap();
        let distro_parts: Vec<&str> = distro_output.split("=").collect();
        if let Some(..) = distro_parts.get(1) {
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
    //////////////////////////////////////////////////////////////////////
    // Get data from threads //////////////////////////////////////////////////////////////
    let usr = name_thread.await.unwrap();
    let distro = distro_thread.await.unwrap();
    let shell = shell_thread.await.unwrap();
    let desktop = desktop_thread.await.unwrap();
    let pkg = packages_thread.await.unwrap();
    let arch = arch_thread.await.unwrap();
    ///////////////////////////////////////////////////////////////////////////////////////
    let mut handle = io::stdout().lock();
    // check if values are empty & print if not ///////////////////////////////////////////
    write!(handle, "{}{} - {}", "x".red().bold(), "Fetch".cyan(), String::from_utf8_lossy(&usr.stdout)).unwrap();
    let sh = String::from_utf8_lossy(&shell.stdout);
    if sh != "\n" {
        write!(handle, "   {} ~ {}", "Shell".purple(), sh).unwrap();
    }
    if pkg != 0 {
        writeln!(handle, "   {} ~ {}, {}", "PKGs".purple(), pkg, arch).unwrap();
    } else if !arch.is_empty() {
        writeln!(handle, "   {} ~ {}", "Arch".purple(), arch).unwrap();
    }
    if !distro.is_empty() {
        write!(handle, "   {} ~ {}", "Distro".purple(), distro).unwrap();
    }
    let de = String::from_utf8_lossy(&desktop.stdout);
    if de != "\n" {
        write!(handle, "   {} ~ {}", "Desktop".purple(), de).unwrap();
    }
    ///////////////////////////////////////////////////////////////////////////////////////
    drop(handle);
Ok(())
}
