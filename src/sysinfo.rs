use std::os::raw::{c_long, c_ulong, c_ushort, c_uint, c_int, c_char};

#[repr(C)]
// https://stackoverflow.com/questions/349889/how-do-you-determine-the-amount-of-linux-system-ram-in-c
pub struct sysinfo {
    pub uptime: c_long,             /* Seconds since boot */
    pub loads: [c_ulong; 3],        /* 1, 5, and 15 minute load averages */
    pub totalram: c_ulong,          /* Total usable main RAM size */
    pub freeram: c_ulong,           /* Available memory size */
    pub sharedram: c_ulong,         /* Amount of shared memory */
    pub bufferram: c_ulong,         /* Memory used by buffers */
    pub totalswap: c_ulong,         /* Total swap space size */
    pub freeswap: c_ulong,          /* Swap space still available */
    pub procs: c_ushort,            /* Number of current processes */
    pub pad: c_ushort,              /* Padding for m68k */
    pub totalhigh: c_ulong,         /* Total high memory size */
    pub freehigh: c_ulong,          /* Available high memory size */
    pub mem_unit: c_uint,           /* Memory unit size in bytes */
    pub _f: [c_char; 0],            /* Padding: libc doesn't define this field */
}

extern "C" {
    pub fn sysinfo(info: *mut sysinfo) -> c_int;
}

/// a "safe" wrapper to get you that sweet struct
///
/// this works the same way as it does in c. if you want to get the uptime, all you have to do is
/// `info.uptime` (in c, it would be `info->uptime`)
///
/// # examples
///
/// ```rust
/// let info = collect();
/// dbg!(info.uptime); // uptime in seconds
/// ```
///
/// # soundness
/// this function uses unsafe {}. please be certain that you need this implementation
pub fn collect() -> sysinfo {
    unsafe {
        let mut info: sysinfo = std::mem::zeroed();
        sysinfo(&mut info);
        return info;
    }
}
