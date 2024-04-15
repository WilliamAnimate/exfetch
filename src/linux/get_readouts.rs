use tokio::{task::spawn, join};
use crate::*; // FIXME: this
use crate::data::Data;
use exfetch::prelude::*;

pub async fn obtain() -> crate::data::Data<'static> {
    // this is an I/O operation: reading how much files are in a specific dir. this is SLOW, so
    // always start this thread **FIRST**.
    let packages_thread = spawn(async {
        packages_readout::get()
    });

    let header_thread = spawn(async {
        let usr = get_env_var!("USER");
        generate_header_from_string(usr)
    });

    // this is an I/O operation: reading /etc/os-release
    let distro_thread = spawn(async {
        distro_readout::get()
    });

    let cpu_name_thread = spawn(async {
        cpu_readout::get()
    });

    let desktop = get_env_var!("XDG_SESSION_DESKTOP");

    let shell = get_env_var!("SHELL");

    let sysinfo = sysinfo_dot_h::try_collect();
    let (phys_mem, swap_mem, uptime) = match sysinfo {
        Ok(sysinfo) => {
            (format_memory_from_bytes(sysinfo.totalram),
             format_memory_from_bytes(sysinfo.totalswap),
             format_uptime_from_secs(sysinfo.uptime))
        }
        Err(_) => (String::new(), String::new(), String::new()),
    };

    // join! to await all `futures` types concurrently
    let (header, distro, cpu, pkg) = join!(
        header_thread,
        distro_thread,
        cpu_name_thread,
        packages_thread,
    );

    // and then .unwrap the results. pray that none of them contain an `Err` type & panic! the app
    // that'd be bad lol
    let header = header.unwrap();
    let distro = distro.unwrap();
    let shell = shell;
    let cpu = cpu.unwrap();
    let desktop = desktop;
    let pkg = pkg.unwrap();
    let arch = exfetch::get_cpu_arch();

    Data {
        username: header,
        distro_or_os: distro,
        shell_name: shell,
        cpu_name: cpu,
        desktop_env: desktop,
        phys_ram: phys_mem,
        swap_ram: swap_mem,
        packages: pkg,
        uptime_formatted: uptime,
        cpu_architecture: arch,
    }
}
