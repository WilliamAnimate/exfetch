#![allow(clippy::cast_possible_truncation)]
#![allow(unused_must_use)]

mod cpu_readout;
mod distro_readout;
mod packages_readout;

use std::io::{self, Write};
use colored::Colorize;
use tokio::{task::spawn, join};

#[cfg(windows)] use winreg::enums::*;
#[cfg(windows)] use winreg::RegKey;

macro_rules! writeln_to_handle_if_not_empty {
    ($handle:expr, $entry:expr, $value:expr, $terminal_width:expr) => {
        if !$value.is_empty() {
            writeln_to_handle!($handle, $entry, $value, $terminal_width);
        }
    };
}

macro_rules! writeln_to_handle {
    ($handle:expr, $entry:expr, $value:expr, $terminal_width:expr) => {
        let to_write = format!("│ {} ~ {}", $entry.purple(), $value);
        let padding = $terminal_width as usize - ($entry.len() + $value.len());
        writeln!($handle, "{}", format!("{}{} │", to_write, " ".repeat(padding as usize)));
    };
}

macro_rules! get_env_var {
    ($var:expr) => {
        std::env::var($var).unwrap_or_else(|_| String::new())
    };
}

/// returns the length as an i16; designed to make the code more concise.
macro_rules! getlen {
    ($to_find:expr) => {
        $to_find.len() as i16 + 6 // add 6 because of the ` ~ ` and padding between the edge of the box
    }
}

fn return_super_fancy_column_stuff(text: &str, times: i16) -> String {
    let padding = "─";
    let trailing = "─".repeat(((times + 4) - text.len() as i16).try_into().unwrap());
    format!("╭{padding}{text}{trailing}╮")
}

fn return_super_fancy_column_closure_stuff(times: i16) -> String {
    let lines = "─".repeat((times + 5).try_into().unwrap());
    format!("╰{lines}╯")
}

#[tokio::main]
async fn main() -> io::Result<()> {
    let name_thread = spawn(async {
        #[cfg(unix)] return get_env_var!("USER");
        #[cfg(windows)] return get_env_var!("USERNAME");
    });

    let distro_thread = spawn(async {
        distro_readout::get()
    });

    let cpu_name_thread = spawn(async {
        cpu_readout::get()
    });

    let desktop_thread = spawn(async {
        #[cfg(unix)] {
            get_env_var!("XDG_SESSION_DESKTOP")
        }
        #[cfg(windows)] {
            "Explorer"
        }
    });

    let shell_thread = spawn(async {
        get_env_var!("SHELL")
    });

    let packages_thread = spawn(async {
        packages_readout::get()
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
                    formatted_uptime.push_str(&format!("{days}d, "));
                }
                if hrs > 0 || days > 0 {
                    formatted_uptime.push_str(&format!("{hrs}h, "));
                }
                if mins > 0 || hrs > 0 || days > 0 {
                    formatted_uptime.push_str(&format!("{mins}m"));
                } else {
                    // system uptime is less than a minute. display seconds instead.
                    formatted_uptime.push_str(&format!("{raw}s"));
                }

                formatted_uptime
            }
            Err(_) => String::new(),
        }
    });

    // join! to await all `futures` types concurrently
    let (usr, distro, shell, cpu_name, desktop, pkg, uptime) = join!(
        name_thread,
        distro_thread,
        shell_thread,
        cpu_name_thread,
        desktop_thread,
        packages_thread,
        uptime_thread,
    );

    // and then .unwrap the results. pray that none of them contain an `Err` type & panic! the app
    // that'd be bad lol
    let usr = usr.unwrap();
    let distro = distro.unwrap();
    let shell = shell.unwrap();
    let cpu_name = cpu_name.unwrap();
    let desktop = desktop.unwrap();
    let pkg = pkg.unwrap();
    let uptime = uptime.unwrap();
    let arch = std::env::consts::ARCH;

    // adds a value to a vec!
    let mut array: Vec<i16> = Vec::new(); // array lel
    array.extend([
         getlen!(usr),
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
    writeln!(handle, "{}{} - {}", "ex".red().bold(), "Fetch".cyan(), usr).unwrap();

    writeln!(handle, "{}", return_super_fancy_column_stuff("HARDWARE", box_width));
    writeln_to_handle_if_not_empty!(handle, "CPU", cpu_name, box_width); // should never be empty smh
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

fn get_max_value_of_vec(vec: &[i16]) -> i16 {
    vec.iter().max().map_or_else(|| panic!("the entire vector is empty, wtf?"), |max| *max)
}
