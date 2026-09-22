-- Requires the "with_test_users" fixture.
-- Two invitations sent by different users, to test the view_foreign_invitations permission.
INSERT INTO invitations (created_at, expires_at, invitation_key, message, sender_id, receiver_email)
VALUES (NOW(), NOW() + INTERVAL '7 days', 'test_invitation_key_own', 'hello', 100, 'own_invite@testdomain.com');

INSERT INTO invitations (created_at, expires_at, invitation_key, message, sender_id, receiver_email)
VALUES (NOW(), NOW() + INTERVAL '7 days', 'test_invitation_key_foreign', 'hello', 191, 'foreign_invite@testdomain.com');
