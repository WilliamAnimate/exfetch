// lib.rs
// OS agnostic functions

pub mod prelude {
    pub use crate::*; // code bloat lmao
}

#[inline(always)] // haha perf
pub fn get_cpu_arch() -> &'static str {
    std::env::consts::ARCH
}

#[inline(always)]
pub fn format_cpu_string(text: &str) -> String {
    text.replace("(R)", "")
        .replace("(TM)", "")
        .replace(" @ ", "(")
        .replace("CPU", "")
        .replace("GHz", "GHz)")
}

#[inline(always)]
pub fn format_memory_from_bytes(mem: u64) -> String {
    let total = mem / 1024000;
    let mut output: String;
    if total != 0 {
        output = total.to_string();
        output.push_str(" MB");
    } else {
        output = String::from("Disabled"); // swap-file only, but we don't know which is swap and
                                           // which is phys. but you should never have 0mb phys
                                           // ram, and sysinfo will return an Err type, which
                                           // should instruct the code calling this function to not
                                           // proceed with that struct.
    }

    // note: the sysinfo struct does not expose enough useful info to determine the amount of
    // available ram. the next best option is `free` ram (unused ram) but it isn't the same thing.
    output
}

#[inline(always)]
pub fn format_uptime_from_secs(raw: i64) -> String {
    // for year calculation, we use a float to be able to divide by 365.25
    // we dont need all that accuracy (accuracy != precision) but yolo
    let (years, days, hrs, mins) = (raw as f64 / (60.0 * 60.0 * 24.0 * 365.25),
                                    raw / (60 * 60 * 24),
                                    raw/ (60 * 60) % 24,
                                    raw / 60 % 60);

    let mut formatted_uptime = String::new();

    let years_no_decimal = years as i64;
    if years_no_decimal > 0 {
        formatted_uptime.push_str(&years_no_decimal.to_string());
        formatted_uptime.push_str("y, ");
    }
    if days > 0 {
        formatted_uptime.push_str(&days.to_string());
        formatted_uptime.push_str("d, ");
    }
    if hrs > 0 || days > 0 {
        formatted_uptime.push_str(&hrs.to_string());
        formatted_uptime.push_str("h, ");
    }
    if mins > 0 || hrs > 0 || days > 0 {
        formatted_uptime.push_str(&mins.to_string());
        formatted_uptime.push('m');
    } else {
        // system uptime is less than a minute. display seconds instead.
        formatted_uptime.push_str(&raw.to_string());
        formatted_uptime.push('s');
    }

    formatted_uptime
}

#[inline(always)]
pub fn generate_header_from_string(usr: String) -> String {
    let mut result = String::from("\x1B[0;31m\x1B[1mex\x1B[0;36mFetch\x1B[0m - ");

    result.push_str(&usr);
    result.push('\n');

    result
}

#[macro_export]
macro_rules! get_env_var {
    ($var:expr) => {
        std::env::var($var).unwrap_or_else(|_| String::new())
    };
}

