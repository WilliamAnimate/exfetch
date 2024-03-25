/// formats a given i64 to a more human-readable format.
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
