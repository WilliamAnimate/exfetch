// idk, stuff ////////////////
#![allow(unused_doc_comments)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_must_use)]
//////////////////////////////

use std::{thread, io::{self, Write}, process::Command};
use colored::*;

pub mod packages;

fn main() -> io::Result<()> {
    // usrname //////////////////////////
    let name_thread = thread::spawn(|| {
        Command::new("whoami")
            .output()
            .expect("Can't fetch your username")
    });
    /////////////////////////////////////

    // OS Related stuff /////////////////////////////////////////
    let distro_thread = thread::spawn(|| -> Result<String, ()> {
        let raw = Command::new("/bin/sh")
            .arg("-c")
            .arg("cat /etc/os-release | grep PRETTY_NAME")
            .output()
            .expect("Can't fetch your distro");

        let output = String::from_utf8(raw.stdout).unwrap();
        let parts: Vec<&str> = output.split("=").collect();
        Ok(parts[1].replace("\"", ""))
    });
    
    let arch_thread = thread::spawn(|| {
        Command::new("uname")
            .arg("-m")
            .output()
            .expect("Can't fetch your shell")
    });

    let desktop_thread = thread::spawn(|| {
        Command::new("/bin/sh")
            .arg("-c")
            .arg("echo $XDG_SESSION_DESKTOP")
            .output()
            .expect("Can't fetch your desktop")
    });

    let shell_thread = thread::spawn(|| {
        Command::new("/bin/sh")
            .arg("-c")
            .arg("echo $SHELL | head -n1 | cut -d '/' -f4")
            .output()
            .expect("Can't fetch your shell")
    });

    let kernel_thread = thread::spawn(|| {
        Command::new("uname")
            .arg("-r")
            .output()
            .expect("Can't fetch your kernel")
    });
    /////////////////////////////////////////////////////////////

    let usr = name_thread.join().unwrap();
    let distro = distro_thread.join().unwrap();
    let shell = shell_thread.join().unwrap();
    let kernel = kernel_thread.join().unwrap();
    let desktop = desktop_thread.join().unwrap();
    let pkg = packages::get_num_packages().to_string();
    let arch = arch_thread.join().unwrap();

    ///////////////////////////////////////////////////////////////////////

    let mut handle = io::stdout().lock(); // locks stdout so you can write to it with write!. this
                                          // is faster because print! and println! locks stdout,
                                          // writes, then unlocks it after, which is slow.

    write!(handle, "{}{} - {}", "x".red().bold(), "Fetch".cyan(), String::from_utf8_lossy(&usr.stdout)).unwrap();
    write!(handle, "   {} ~ {}", "Distro".purple(), distro.unwrap()).unwrap();
    write!(handle, "   {} ~ {}", "Shell".purple(), String::from_utf8_lossy(&shell.stdout)).unwrap();
    write!(handle, "   {} ~ {}", "Kernel".purple(), String::from_utf8_lossy(&kernel.stdout)).unwrap();
    write!(handle, "   {} ~ {}", "Desktop".purple(), String::from_utf8_lossy(&desktop.stdout));
    write!(handle, "   {} ~ {}, {}", "PKGs".purple(), pkg, String::from_utf8_lossy(&arch.stdout)).unwrap();
    drop(handle);
    Ok(())
}