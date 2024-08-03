#![allow(clippy::cast_possible_truncation)]
#![allow(unused_must_use)]

mod cpu_readout;
mod distro_readout;
mod packages_readout;
mod memory_readout;
mod uptime_readout;

use std::io::{self, Write, BufWriter};
use tokio::{task::spawn, join};

#[cfg(windows)] use winreg::enums::*;
#[cfg(windows)] use winreg::RegKey;

macro_rules! writeln_to_handle_if_not_empty {
    ($handle:expr, $entry:expr, $value:expr, $terminal_width:expr) => {
        if !$value.is_empty() {
            if !$value.is_empty() {
                writeln_to_handle!($handle, $entry, $value, $terminal_width);
            }
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

        let mut to_write = String::from("│\x1B[0;35m ");
        to_write.push_str($entry);
        to_write.push_str("\x1B[0m ~ ");
        to_write.push_str($value.to_string().as_str());

        let mut output = String::from(to_write);
        output.push_str(&" ".repeat(padding as usize));
        output.push_str(" │\n");

        $handle.write_all(output.as_bytes());
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
    let header_thread = spawn(async {
        let usr: String;
        #[cfg(unix)] {usr = get_env_var!("USER");}
        #[cfg(windows)] {usr = get_env_var!("USERNAME");}
        let mut result = String::from("\x1B[0;31m\x1B[1mex\x1B[0;36mFetch\x1B[0m - ");

        result.push_str(&usr);
        result.push('\n');

        result
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

    let mut phys_mem = String::new();
    let mut swap_mem = String::new();
    let mut uptime = String::new();
    #[cfg(unix)] {
        let sysinfo = sysinfo_dot_h::try_collect();
        if let Ok(sysinfo) = sysinfo {
            phys_mem = memory_readout::format_memory_from_bytes(sysinfo.totalram);
            swap_mem = memory_readout::format_memory_from_bytes(sysinfo.totalswap);
            uptime = uptime_readout::format_uptime_from_secs(sysinfo.uptime);
        }
    }

    // join! to await all `futures` types concurrently
    let (header, distro, shell, cpu_name, desktop, pkg) = join!(
        header_thread,
        distro_thread,
        shell_thread,
        cpu_name_thread,
        desktop_thread,
        packages_thread,
    );

    // and then .unwrap the results. pray that none of them contain an `Err` type & panic! the app
    // that'd be bad lol
    let header = header.unwrap();
    let distro = distro.unwrap();
    let shell = shell.unwrap();
    let cpu_name = cpu_name.unwrap();
    let desktop = desktop.unwrap();
    let pkg = pkg.unwrap();
    let arch = std::env::consts::ARCH;

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
