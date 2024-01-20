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
    let shell = shell_thread.join().unwrap();
    let kernel = kernel_thread.join().unwrap();

    ///////////////////////////////////////////////////////////////////////

    let mut handle = io::stdout().lock(); // locks stdout so any further writes (eg print) will be
                                          // faster since it does not have to lock stdout and then
                                          // unlock it on a write.

    write!(handle, "{}{} - {}", "x".red().bold(), "Fetch".cyan(), String::from_utf8_lossy(&name_usr.stdout)).unwrap();
    write!(handle, "    {} ~ {}", "Shell".purple(), String::from_utf8_lossy(&shell.stdout)).unwrap();
    write!(handle, "    {} ~ {}", "Kernel".purple(), String::from_utf8_lossy(&kernel.stdout)).unwrap();

    drop(handle);

    Ok(())
}

