use arcadia_api::services::auth_service::{
    validate_email, validate_password, validate_password_verification, validate_username,
};
use arcadia_common::error::Error;

#[test]
fn test_validate_email() {
    // Valid emails
    assert!(validate_email("test@example.com").is_ok());
    assert!(validate_email("user.name@domain.co.uk").is_ok());
    assert!(validate_email("user+tag@example.org").is_ok());

    // Invalid emails
    assert!(matches!(
        validate_email(""),
        Err(Error::InvalidEmailAddress)
    ));
    assert!(matches!(
        validate_email("   "),
        Err(Error::InvalidEmailAddress)
    ));
    assert!(matches!(
        validate_email("invalid-email"),
        Err(Error::InvalidEmailAddress)
    ));
    assert!(matches!(
        validate_email("@example.com"),
        Err(Error::InvalidEmailAddress)
    ));
    assert!(matches!(
        validate_email("user@"),
        Err(Error::InvalidEmailAddress)
    ));
}

#[test]
fn test_validate_username() {
    // Valid usernames
    assert!(validate_username("user123").is_ok());
    assert!(validate_username("test_user").is_ok());
    assert!(validate_username("user-name").is_ok());
    assert!(validate_username("user123name").is_ok());
    assert!(validate_username("a".repeat(4).as_str()).is_ok());
    assert!(validate_username("a".repeat(15).as_str()).is_ok());

    // Invalid usernames
    assert!(matches!(validate_username(""), Err(Error::InvalidUsername)));
    assert!(matches!(
        validate_username("   "),
        Err(Error::InvalidUsername)
    ));
    assert!(matches!(
        validate_username("abc"),
        Err(Error::InvalidUsername)
    )); // too short
    assert!(matches!(
        validate_username("a".repeat(16).as_str()),
        Err(Error::InvalidUsername)
    )); // too long
    assert!(matches!(
        validate_username("user@name"),
        Err(Error::InvalidUsername)
    )); // invalid char
    assert!(matches!(
        validate_username("user name"),
        Err(Error::InvalidUsername)
    )); // space
    assert!(matches!(
        validate_username("Guest-1234"),
        Err(Error::InvalidUsername)
    )); // reserved for irc guests
    assert!(matches!(
        validate_username("guest-abc"),
        Err(Error::InvalidUsername)
    )); // reserved for irc guests, case insensitive
    assert!(validate_username("guest123").is_ok()); // only the guest- prefix is reserved
}

#[test]
fn test_validate_password() {
    // Valid passwords
    assert!(validate_password("Password1234").is_ok());
    assert!(validate_password("MySecurePass123").is_ok());
    assert!(validate_password(&("a".repeat(12) + "A1")).is_ok());

    // Invalid passwords
    assert!(matches!(
        validate_password(""),
        Err(Error::InvalidPassword(_))
    ));
    assert!(matches!(
        validate_password("short"),
        Err(Error::InvalidPassword(_))
    )); // too short
    assert!(matches!(
        validate_password("nouppercase123"),
        Err(Error::InvalidPassword(_))
    )); // no uppercase
    assert!(matches!(
        validate_password("NOLOWERCASE123"),
        Err(Error::InvalidPassword(_))
    )); // no lowercase
    assert!(matches!(
        validate_password("NoNumbers"),
        Err(Error::InvalidPassword(_))
    )); // no numbers
}

#[test]
fn test_validate_password_verification() {
    // Valid password verification
    assert!(validate_password_verification("Password1234", "Password1234").is_ok());

    // Invalid password verification
    assert!(matches!(
        validate_password_verification("Password1234", ""),
        Err(Error::PasswordsDoNotMatch)
    ));
    assert!(matches!(
        validate_password_verification("Password1234", "DifferentPass123"),
        Err(Error::PasswordsDoNotMatch)
    ));
}
