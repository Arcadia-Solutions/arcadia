INSERT INTO
    invitations (expires_at, invitation_key, message, sender_id, receiver_email)
VALUES
    (CURRENT_TIMESTAMP + INTERVAL '30 days', 'valid_key', 'invitation message', 1, 'newuser@testdomain.com')
