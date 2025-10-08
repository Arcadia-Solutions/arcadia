pub fn get_announce_url(passkey: String, tracker_url: &str) -> String {
    format!("{tracker_url}announce/{passkey}")
}

pub fn looks_like_url(s: &str) -> bool {
    let s = s.trim();
    let b = s.as_bytes();
    (b.len() >= 7 && b[..7].eq_ignore_ascii_case(b"http://"))
        || (b.len() >= 8 && b[..8].eq_ignore_ascii_case(b"https://"))
}
