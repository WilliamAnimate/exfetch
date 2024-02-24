#![allow(unused_must_use)]
use std::{io::{self, Write, BufRead}, fs::File};
use colored::Colorize;
use tokio::{task::spawn, join};

pub mod packages;

macro_rules! writeln_to_handle_if_not_empty {
    ($handle:expr, $entry:expr, $value:expr, $terminal_width:expr) => {
        if !$value.is_empty() {
            writeln_to_handle!($handle, $entry, $value, $terminal_width);
        }
    };
}

macro_rules! writeln_to_handle {
    ($handle:expr, $entry:expr, $value:expr, $terminal_width:expr) => {
        // use std::fmt::Write;
        let to_write = format!("│ {} ~ {}", $entry.purple(), $value);
        let padding = $terminal_width as usize - ($entry.len() + $value.len()) + 3;
        // dbg!(&padding);
        writeln!($handle, "{}", format!("{}{} │", to_write, " ".repeat(padding)));
    };
}

macro_rules! get_env_var {
    ($var:expr) => {
        std::env::var($var).unwrap_or_else(|_| String::new())
    };
}

/// returns the length as an i32; designed to make the code more concise.
macro_rules! getlen {
    ($to_find:expr) => {
        $to_find.len() as i16
    }
}

fn return_super_fancy_column_stuff(text: &str, times: i16) -> String {
    let padding = "─".repeat(1);
    let trailing = "─".repeat(((times + 7) - text.len() as i16).try_into().unwrap());
    return format!("╭{}{}{}╮", padding, text, trailing);
}

fn return_super_fancy_column_closure_stuff(times: i16) -> String {
    let lines = "─".repeat(((times + 8)).try_into().unwrap());
    return format!("╰{}╯", lines);
}

#[tokio::main]
async fn main() -> io::Result<()> {
    let name_thread = spawn(async {
        get_env_var!("USER")
    });

    let distro_thread = spawn(async {
        let file = File::open("/etc/os-release").expect("Can't open /etc/os-release!");
        let mut reader = io::BufReader::new(file);
        let (mut line, mut pretty_name) = (String::new(), String::new());

        while reader.read_line(&mut line).expect("Failed to read line") > 0 {
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
        get_env_var!("XDG_SESSION_DESKTOP")
    });

    let shell_thread = spawn(async {
        get_env_var!("SHELL")
    });

    let packages_thread = spawn(async {
        packages::get_num_packages()
    });

    let uptime_thread = spawn(async {
        match uptime_lib::get() {
            Ok(uptime) => {
                let raw = uptime.as_secs();
                let (days, hrs, mins) = (raw / (60 * 60 * 24),
                                         raw/ (60 * 60) % 24,
                                         raw / 60 % 60);

                let mut formatted_uptime = String::new();

                if days > 0 {
                    formatted_uptime.push_str(&format!("{}d, ", days));
                }
                if hrs > 0 || days > 0 {
                    formatted_uptime.push_str(&format!("{}h, ", hrs));
                }
                if mins > 0 || hrs > 0 || days > 0 {
                    formatted_uptime.push_str(&format!("{}m", mins));
                } else {
                    // system uptime is less than a minute. display seconds instead.
                    formatted_uptime.push_str(&format!("{}s", raw));
                }

                formatted_uptime
            }
            Err(_) => String::new(),
        }
    });

    // join! to await all `futures` types concurrently
    let (usr, distro, shell, desktop, pkg, uptime) = join!(
        name_thread,
        distro_thread,
        shell_thread,
        desktop_thread,
        packages_thread,
        uptime_thread,
    );

    // and then .unwrap the results. pray that none of them contain an `Err` type & panic! the app
    // that'd be bad lol
    let usr = usr.unwrap();
    let distro = distro.unwrap();
    let shell = shell.unwrap();
    let desktop = desktop.unwrap();
    let pkg = pkg.unwrap();
    let uptime = uptime.unwrap();
    let arch = std::env::consts::ARCH;

    // adds a value to a vec!
    let mut array: Vec<i16> = Vec::new(); // array lel
    array.extend([getlen!(usr), getlen!(distro), getlen!(shell), getlen!(desktop), getlen!(uptime), getlen!(arch)]);

    // and then finds the biggest number in a vec!
    // this is important because we don't want the fancy af box to go to the edge of the screen.
    let box_width = get_max_value_of_vec(array) + 7;
    // HACK ALERT: the longest field is "desktop", so we add how long desktop is (7 chars.)
    // this is hardcoded. good luck maintaining :3

    let mut handle = io::stdout().lock(); // lock stdout for slightly faster writing
    // the actual printing
    writeln!(handle, "{}{} - {}", "ex".red().bold(), "Fetch".cyan(), usr).unwrap();
    /*
╭───────┬─────────╮
│ Name  ┆ NonFree │
╞═══════╪═════════╡
│ r8168 ┆ false   │
╰───────┴─────────╯
*/
    writeln!(handle, "{}", return_super_fancy_column_stuff("HARDWARE", box_width));
    writeln_to_handle_if_not_empty!(handle, "Uptime", uptime, box_width);
    writeln!(handle, "{}", return_super_fancy_column_closure_stuff(box_width));
    writeln!(handle, "{}", return_super_fancy_column_stuff("SOFTWARE", box_width));
    writeln_to_handle_if_not_empty!(handle, "Shell", shell, box_width);
    if pkg != 0 {
        writeln_to_handle_if_not_empty!(handle, "PKGs", format!("{}, {}", pkg, arch), box_width);
    } else {
        writeln_to_handle_if_not_empty!(handle, "Arch", arch, box_width);
    }
    writeln_to_handle_if_not_empty!(handle, "Distro", distro, box_width);
    writeln_to_handle_if_not_empty!(handle, "Desktop", desktop, box_width);
    writeln!(handle, "{}", return_super_fancy_column_closure_stuff(box_width));

    drop(handle);
    Ok(())
}

fn get_max_value_of_vec(vec: Vec<i16>) -> i16 {
    match vec.iter().max() {
        Some(max) => *max,
        None => panic!("the entire vector is empty, wtf?"),
    }
}
