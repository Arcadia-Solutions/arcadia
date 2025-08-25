INSERT INTO
    invitations (
        expires_at,
        invitation_key,
        message,
        sender_id,
        receiver_email
    )
VALUES
    (
        NOW () + INTERVAL '30 days',
        'valid_key',
        'invitation message',
        1,
        'newuser@testdomain.com'
    )
