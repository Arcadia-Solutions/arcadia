UPDATE arcadia_settings
SET open_signups = FALSE;

INSERT INTO
    invitations (
        expires_at,
        invitation_key,
        message,
        inviter_notes,
        sender_id,
        receiver_email
    )
VALUES
    (
        NOW () + INTERVAL '30 days',
        'valid_key',
        'invitation message',
        'some notes',
        1,
        'newuser@testdomain.com'
    )
