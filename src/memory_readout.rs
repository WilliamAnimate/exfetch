// TODO: implement windows support
// for now, this code will return an empty string which should hint it to not output anything.
#[cfg(windows)]
pub fn get_physical() -> String {String::new()}
#[cfg(windows)]
pub fn get_virtual() -> String {String::new()}
use sysinfo_dot_h::try_collect;

#[cfg(unix)]
pub fn get_physical() -> String {
    let info = match try_collect() {
        Ok(info) => info,
        Err(_err) => return String::new(),
    };
    let totalram = info.totalram / 1024000;
    let mut output = String::from(totalram.to_string());
    output.push_str(" MB");
    // note: the sysinfo struct does not expose enough useful info to determine the amount of
    // available ram. the next best option is `free` ram (unused ram) but it isn't the same thing.
    output
}

#[cfg(unix)]
pub fn get_virtual() -> String {
    // TODO: this can be improved upon:
    // this code will collect the sysinfo twice. maybe there is a way we can share this?
    // i don't want to use dependencies like lazy static or once cell though. maybe someday.
    let info = match try_collect() {
        Ok(info) => info,
        Err(_err) => return String::new(),
    };

    let virtualram = info.totalswap / 1024000;
    let mut output = String::new();
    if virtualram != 0 {
        output = String::from(virtualram.to_string());
        output.push_str(" MB");
    } else {
        output.push_str("Disabled");
    }
    output
}

