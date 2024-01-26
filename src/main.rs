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
    // pkgs ///////////////////////////
    let packages_thread = spawn(async {
        packages::get_num_packages()
    });
    ///////////////////////////////////

    // usrname //////////////////////////
    let name_thread = spawn(async {
        Command::new("/bin/sh")
            .args(["-c", "echo $USER"])
            .output()
            .expect("Can't fetch your username")
    });
    ////////////////////////////////////////////

    // OS Related stuff /////////////////////////////////////////
    let distro_thread = spawn(async {
        let distro_raw = Command::new("sh")
            .arg("-c")
            .args(["-c", "cat /etc/os-release | grep PRETTY_NAME"])
            .output()
            .expect("Can't fetch your distro");

        let distro_output = String::from_utf8(distro_raw.stdout).unwrap();
        let distro_parts: Vec<&str> = distro_output.split("=").collect();
        Ok(distro_parts[1].replace("\"", ""))
    });
    
    let arch_thread = spawn(async {
        Command::new("uname")
            .arg("-m")
            .output()
            .expect("Can't fetch your arch")
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
    /////////////////////////////////////////////////////////////
  
    //////////////////////////////////////////
    let kernel_thread = spawn(async {
        Command::new("uname")
            .arg("-r")
            .output()
            .expect("Can't fetch your kernel")
    });
    //////////////////////////////////////////

    let usr = name_thread.await.unwrap();
    let distro: Result<String, std::io::Error> = distro_thread.await.unwrap();
    let shell = shell_thread.await.unwrap();
    let kernel = kernel_thread.await.unwrap();
    let desktop = desktop_thread.await.unwrap();
    let pkg: Result<String, _> = packages_thread.await.map(|pkg| pkg.to_string()); // dumb little hack
    let arch = arch_thread.await.unwrap();
    ///////////////////////////////////////////////////////////////////////

    let mut handle = io::stdout().lock(); // locks stdout so you can write to it with write!. this
                                          // is faster because print! and println! locks stdout,
                                          // writes, then unlocks it after, which is slow.

    write!(handle, "{}{} - {}", "x".red().bold(), "Fetch".cyan(), String::from_utf8_lossy(&usr.stdout)).unwrap();
    write!(handle, "   {} ~ {}", "Distro".purple(), distro.unwrap()).unwrap();
    write!(handle, "   {} ~ {}", "Shell".purple(), String::from_utf8_lossy(&shell.stdout)).unwrap();
    write!(handle, "   {} ~ {}", "Kernel".purple(), String::from_utf8_lossy(&kernel.stdout)).unwrap();
    write!(handle, "   {} ~ {}", "Desktop".purple(), String::from_utf8_lossy(&desktop.stdout)).unwrap();
    write!(handle, "   {} ~ {}, {}", "PKGs".purple(), pkg.expect("Your distro isn't supported!"), String::from_utf8_lossy(&arch.stdout)).unwrap();
    drop(handle);
    Ok(())
}
