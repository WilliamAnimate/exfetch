// idk, stuff ////////////////
#![allow(unused_doc_comments)]
#![allow(unused_variables)]
#![allow(unused_imports)]
//////////////////////////////

use std::env;
use std::process::Command;
use colored::*;

fn main() {
    // usrname //////////////////////////
    let name_usr = Command::new("whoami")
    .output()
    .expect("Can't fetch your username");
    /////////////////////////////////////

    // Linux stuff, idk what to call this //
    let shell = Command::new("/bin/bash")
    .arg("-c")
    .arg("echo $SHELL")
    .output()
    .expect("Can't fetch your shell");

    let kernel = Command::new("uname")
    .arg("-r")
    .output()
    .expect("Can't fetch your kernel ver.");
    ////////////////////////////////////////

///////////////////////////////////////////////////////////////////////
print!("{}{} - {}", "x".red().bold(), "Fetch".cyan(), String::from_utf8_lossy(&name_usr.stdout));
print!("    {} ~ {}", "Shell".purple(), String::from_utf8_lossy(&shell.stdout));
print!("    {} ~ {}", "Kernel".purple(), String::from_utf8_lossy(&kernel.stdout));
}

