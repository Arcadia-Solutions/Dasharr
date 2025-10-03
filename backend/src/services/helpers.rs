pub fn bytes_from_size_string(s: &str) -> i64 {
    let parts: Vec<&str> = s.split(' ').collect();
    if parts.len() != 2 {
        return 0;
    }
    let value = parts[0].parse::<f64>().unwrap_or(0.0);
    let unit = parts[1];
    let multiplier: f64 = match unit {
        "KiB" => 1024.0,
        "MiB" => 1024.0_f64.powi(2),
        "GiB" => 1024.0_f64.powi(3),
        "TiB" => 1024.0_f64.powi(4),
        "PiB" => 1024.0_f64.powi(5),
        _ => 1.0,
    };
    (value * multiplier).round() as i64
}
