pub fn get_announce_url(passkey_upper: i64, passkey_lower: i64, tracker_url: &str) -> String {
    let passkey = ((passkey_upper as u64 as u128) << 64) | (passkey_lower as u64 as u128);

    format!("{tracker_url}announce/{passkey:x}")
}

pub fn looks_like_url(s: &str) -> bool {
    let s = s.trim();
    let b = s.as_bytes();
    (b.len() >= 7 && b[..7].eq_ignore_ascii_case(b"http://"))
        || (b.len() >= 8 && b[..8].eq_ignore_ascii_case(b"https://"))
}
