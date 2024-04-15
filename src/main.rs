#![allow(clippy::cast_possible_truncation)]
#![allow(unused_must_use)]

mod data;
#[cfg(target_os = "linux")] mod linux;
#[cfg(target_os = "linux")] use crate::linux::*;

#[cfg(windows)] mod windows;
#[cfg(windows)] use crate::windows::*;

use std::io::{self, Write, BufWriter};

macro_rules! writeln_to_handle_if_not_empty {
    ($handle:expr, $entry:expr, $value:expr, $terminal_width:expr) => {
        if !$value.is_empty() {
            writeln_to_handle!($handle, $entry, $value, $terminal_width);
        }
    };
}

macro_rules! writeln_to_handle_if_not_empty_i16 {
    ($handle:expr, $entry:expr, $value:expr, $terminal_width:expr) => {
        if $value != 0 {
            writeln_to_handle!($handle, $entry, $value.to_string(), $terminal_width);
        }
    }
}

macro_rules! writeln_to_handle {
    ($handle:expr, $entry:expr, $value:expr, $terminal_width:expr) => {
        let padding = $terminal_width as usize - ($entry.len() + $value.len());
        let output = format!("│\x1B[0;35m {}\x1B[0m ~ {}{} │\n", $entry, $value, " ".repeat(padding));
        $handle.write_all(output.as_bytes());
    };
}

/// returns the length as an i16; designed to make the code more concise.
macro_rules! getlen {
    ($to_find:expr) => {
        $to_find.len() as i16 + 6 // add 6 because of the ` ~ ` and padding between the edge of the box
    }
}

fn return_super_fancy_column_stuff(text: &str, times: i16) -> String {
    let trailing = "─".repeat(((times + 4) - text.len() as i16).try_into().unwrap());
    let mut output = String::from("╭");
    output.push('─');
    output.push_str(text);
    output.push_str(&trailing);
    output.push_str("╮\n");
    output
}

fn return_super_fancy_column_closure_stuff(times: i16) -> String {
    let lines = "─".repeat((times + 5).try_into().unwrap());
    let mut output = String::from("╰");
    output.push_str(&lines);
    output.push_str("╯\n");
    output
}

#[tokio::main]
async fn main() -> io::Result<()> {
    let readouts = get_readouts::obtain().await;
    let header = readouts.username;
    let distro = readouts.distro_or_os;
    let shell = readouts.shell_name;
    let cpu_name = readouts.cpu_name;
    let desktop = readouts.desktop_env;
    let phys_mem = readouts.phys_ram;
    let swap_mem = readouts.swap_ram;
    let pkg = readouts.packages;
    let uptime = readouts.uptime_formatted;
    let arch = readouts.cpu_architecture;

    // adds a value to a vec!
    let mut array: Vec<i16> = Vec::new(); // array lel
    array.extend([
         getlen!(distro),
         getlen!(shell),
         getlen!(cpu_name) - 3, // hack fix, i don't know why this is needed.
         getlen!(desktop),
         getlen!(uptime),
         getlen!(arch)
    ]);

    // and then finds the biggest number in a vec!
    // this is important because we don't want the fancy af box to go to the edge of the screen.
    let box_width = get_max_value_of_vec(&array);

    let mut handle = io::stdout().lock(); // lock stdout for slightly faster writing
    let mut writer = BufWriter::new(&mut handle); // buffer it for even faster writing
    // TODO: somehow put this .as_bytes() inside the thread that gets it, for concurrency & speed
    // something is causing lifetime errors or something.
    writer.write_all(header.as_bytes());

    writer.write_all(return_super_fancy_column_stuff("HARDWARE", box_width).as_bytes());
    writeln_to_handle_if_not_empty!(&mut writer, "CPU", &cpu_name, box_width); // should never be empty smh
    writeln_to_handle_if_not_empty!(&mut writer, "Phys Mem", &phys_mem, box_width);
    writeln_to_handle_if_not_empty!(&mut writer, "Arch", &arch, box_width);
    writeln_to_handle_if_not_empty!(&mut writer, "Uptime", &uptime, box_width);
    writer.write_all(return_super_fancy_column_closure_stuff(box_width).as_bytes());
    writer.write_all(return_super_fancy_column_stuff("SOFTWARE", box_width).as_bytes());
    writeln_to_handle_if_not_empty!(&mut writer, "Shell", &shell, box_width);
    writeln_to_handle_if_not_empty_i16!(&mut writer, "PKGs", pkg, box_width);
    writeln_to_handle_if_not_empty!(&mut writer, "Distro", &distro, box_width);
    writeln_to_handle_if_not_empty!(&mut writer, "Desktop", &desktop, box_width);
    writeln_to_handle_if_not_empty!(&mut writer, "Swap", &swap_mem, box_width);
    writer.write_all(return_super_fancy_column_closure_stuff(box_width).as_bytes());

    Ok(())
}

fn get_max_value_of_vec(vec: &[i16]) -> i16 {
    vec.iter().max().map_or_else(|| panic!("the entire vector is empty, wtf?"), |max| *max)
}
