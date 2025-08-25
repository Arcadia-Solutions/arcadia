INSERT INTO
    invitations (
        expires_at,
        invitation_key,
        message,
        sender_id,
        receiver_email
    )
VALUES
    -- invite expired 7 days ago. too late!
    (
        NOW () - INTERVAL '7 days',
        'valid_key',
        'invitation message',
        1,
        'newuser@testdomain.com'
    )
