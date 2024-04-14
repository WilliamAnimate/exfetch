/// data! the things in this struct should be very self documenting from their names.
pub struct Data<'a> {
    /// username
    pub username: String,
    /// the distro or operating system being ran.
    pub distro_or_os: String,
    /// the shell that invoked this (or default?)
    pub shell_name: String,
    /// cpu name
    pub cpu_name: String,
    /// desktop environment
    pub desktop_env: String,
    /// physical ram
    pub phys_ram: String,
    /// swap memory
    pub swap_ram: String,
    /// amount of packages in the system.
    pub packages: i16,
    /// uptime formatted like 69d, 4h, 20m, 69s
    pub uptime_formatted: String,
    /// CPU Arch.
    pub cpu_architecture: &'a str,
}

