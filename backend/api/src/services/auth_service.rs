use std::sync::LazyLock;

use arcadia_common::error::{Error, Result};
use regex::Regex;

static EMAIL_REGEX: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^[^\s@]+@[^\s@]+\.[^\s@]+$").unwrap());

static USERNAME_REGEX: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^[a-zA-Z0-9_-]{4,15}$").unwrap());

pub fn validate_email(email: &str) -> Result<()> {
    if !EMAIL_REGEX.is_match(email) {
        return Err(Error::InvalidEmailAddress);
    }
    Ok(())
}

pub fn validate_username(username: &str) -> Result<()> {
    if !USERNAME_REGEX.is_match(username) {
        return Err(Error::InvalidUsername);
    }
    Ok(())
}

pub fn validate_password(password: &str) -> Result<()> {
    if password.len() < 12 {
        return Err(Error::BadRequest(
            "Password must be at least 12 characters long".to_string(),
        ));
    }
    if !password.chars().any(|c| c.is_uppercase()) {
        return Err(Error::BadRequest(
            "Password must contain at least one uppercase letter".to_string(),
        ));
    }
    if !password.chars().any(|c| c.is_lowercase()) {
        return Err(Error::BadRequest(
            "Password must contain at least one lowercase letter".to_string(),
        ));
    }
    if !password.chars().any(|c| c.is_numeric()) {
        return Err(Error::BadRequest(
            "Password must contain at least one number".to_string(),
        ));
    }
    Ok(())
}

pub fn validate_password_verification(password: &str, password_verify: &str) -> Result<()> {
    if password != password_verify {
        return Err(Error::BadRequest("Passwords do not match".to_string()));
    }
    Ok(())
}
