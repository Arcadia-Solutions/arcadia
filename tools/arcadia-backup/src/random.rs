use anyhow::{anyhow, Result};

fn fill(buffer: &mut [u8]) -> Result<()> {
    getrandom::fill(buffer).map_err(|error| anyhow!("cannot get random bytes: {error}"))
}

pub fn hex(bytes: usize) -> Result<String> {
    let mut buffer = vec![0u8; bytes];
    fill(&mut buffer)?;
    Ok(buffer.iter().map(|byte| format!("{byte:02x}")).collect())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hex_has_two_characters_per_byte_and_varies() {
        let (a, b) = (hex(24).unwrap(), hex(24).unwrap());
        assert_eq!(a.len(), 48);
        assert!(a.bytes().all(|c| c.is_ascii_hexdigit()));
        assert_ne!(a, b);
    }
}
