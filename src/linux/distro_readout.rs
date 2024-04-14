use std::{io::{BufRead, BufReader}, fs::File};

pub fn get() -> String {
    let file = File::open("/etc/os-release").expect("Can't open /etc/os-release!");
    let mut reader = BufReader::new(file);
    let (mut line, mut pretty_name) = (String::new(), String::new());



    // let file = File::open("/etc/os-release").unwrap();
    // let mut buffer = BufReader::new(file);
    // let mut first_line = String::new();
    // buffer.read_line(&mut first_line);
    // first_line = first_line.trim().to_string();
    //
    // if line.starts_with("NAME=") {
    //     pretty_name = line.split_once('=').unwrap().1.to_string();
    //     pretty_name = pretty_name.trim().trim_matches('"').to_string();
    // }


    while reader.read_line(&mut line).expect("Failed to read line") > 0 {
        if line.starts_with("PRETTY_NAME=") {
            pretty_name = line.split_once('=').unwrap().1.to_string();
            pretty_name = pretty_name.trim().trim_matches('"').to_string();
            break;
        }
        line.clear();
    }
    pretty_name
    // String::new()
}

