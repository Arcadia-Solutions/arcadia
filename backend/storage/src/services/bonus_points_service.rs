pub fn format_bonus_points(value: i64, decimal_places: i16) -> String {
    if decimal_places <= 0 {
        return value.to_string();
    }
    let width = decimal_places as usize;
    let scale = 10_i64.pow(decimal_places as u32);
    format!("{}.{:0>width$}", value / scale, (value % scale).abs())
        .trim_end_matches('0')
        .trim_end_matches('.')
        .to_owned()
}

#[cfg(test)]
mod tests {
    use super::format_bonus_points;

    #[test]
    fn formats_scaled_bonus_points_with_decimal_places() {
        assert_eq!(format_bonus_points(10_000_000, 4), "1000");
        assert_eq!(format_bonus_points(12_345, 4), "1.2345");
        assert_eq!(format_bonus_points(100, 2), "1");
        assert_eq!(format_bonus_points(150, 2), "1.5");
        assert_eq!(format_bonus_points(1000, 0), "1000");
    }
}
