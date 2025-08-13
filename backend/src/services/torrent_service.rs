pub fn get_announce_url(passkey_upper: i64, passkey_lower: i64, tracker_url: &str) -> String {
    let passkey = ((passkey_upper as u64 as u128) << 64) | (passkey_lower as u64 as u128);

    format!("{tracker_url}announce/{passkey:x}")
}

pub fn looks_like_url(s: &str) -> bool {
    let s = s.trim();
    (s.len() >= 7 && s[..7].eq_ignore_ascii_case("http://"))
        || (s.len() >= 8 && s[..8].eq_ignore_ascii_case("https://"))
}
