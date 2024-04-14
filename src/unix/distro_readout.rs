use std::{io::{BufRead, BufReader}, fs::File};

pub fn get() -> String {
    let file = File::open("/etc/os-release").expect("Can't open /etc/os-release!");
    let mut reader = BufReader::new(file);
    let (mut line, mut pretty_name) = (String::new(), String::new());

    while reader.read_line(&mut line).expect("Failed to read line") > 0 {
        if line.starts_with("PRETTY_NAME=") {
            pretty_name = line.split_once('=').unwrap().1.to_string();
            pretty_name = pretty_name.trim().trim_matches('"').to_string();
            break;
        }
        line.clear();
    }
    pretty_name
}

