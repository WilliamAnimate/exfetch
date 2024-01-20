// idk, stuff ////////////////
#![allow(unused_doc_comments)]
#![allow(unused_variables)]
#![allow(unused_imports)]
//////////////////////////////

use std::{thread, io::{self, Write}, process::Command};
use colored::*;

fn main() -> io::Result<()> {
    // usrname //////////////////////////
    let name_usr_thread = thread::spawn(|| {
        Command::new("whoami")
            .output()
            .expect("Can't fetch your username")
    });
    /////////////////////////////////////

    // my hope is that this code is so ludicrously bad that im strictly forbidden to write rust code ever again
    // and that there is actually a one-liner solution in bash instead of doing this monstrosity
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

    // Linux stuff, idk what to call this //
    let shell_thread = thread::spawn(|| {
        Command::new("/bin/bash")
            .arg("-c")
            .arg("echo $SHELL")
            .output()
            .expect("Can't fetch your shell")
    });

    let kernel_thread = thread::spawn(|| {
        Command::new("uname")
            .arg("-r")
            .output()
            .expect("Can't fetch your kernel ver.")
    });

    let name_usr = name_usr_thread.join().unwrap();
    let distro = distro_thread.join().unwrap();
    let shell = shell_thread.join().unwrap();
    let kernel = kernel_thread.join().unwrap();

    ///////////////////////////////////////////////////////////////////////

    let mut handle = io::stdout().lock(); // locks stdout so you can write to it with write!. this
                                          // is faster because print! and println! locks stdout,
                                          // writes, then unlocks it after, which is slow.

    write!(handle, "{}{} - {}", "x".red().bold(), "Fetch".cyan(), String::from_utf8_lossy(&name_usr.stdout)).unwrap();
    write!(handle, "    {} ~ {}", "Distro".purple(), distro.unwrap()).unwrap();
    write!(handle, "    {} ~ {}", "Shell".purple(), String::from_utf8_lossy(&shell.stdout)).unwrap();
    write!(handle, "    {} ~ {}", "Kernel".purple(), String::from_utf8_lossy(&kernel.stdout)).unwrap();

    drop(handle);

    Ok(())
}

