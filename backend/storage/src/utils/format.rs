const BYTE_UNITS: [&str; 5] = ["B", "KiB", "MiB", "GiB", "TiB"];

pub fn bytes_to_readable(bytes: i64) -> String {
    let mut value = bytes as f64;
    let mut unit_index = 0;
    while value.abs() >= 1024.0 && unit_index < BYTE_UNITS.len() - 1 {
        value /= 1024.0;
        unit_index += 1;
    }
    format!("{value:.2} {}", BYTE_UNITS[unit_index])
}
