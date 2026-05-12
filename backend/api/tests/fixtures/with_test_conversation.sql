-- Locked conversation between user 100 and user 101
INSERT INTO conversations (id, subject, sender_id, receiver_id, locked)
VALUES (100, 'Locked Conversation', 100, 101, TRUE);

-- Initial message for the locked conversation
INSERT INTO conversation_messages (id, conversation_id, created_by_id, content)
VALUES (100, 100, 100, 'This conversation is now locked');

-- Unlocked conversation between user 100 (sender) and user 101 (receiver), used to
-- test that reading the conversation correctly bumps last_seen_at for the reader.
-- last_seen_at fields are anchored to an old timestamp so the test can detect a bump.
INSERT INTO conversations (id, subject, sender_id, receiver_id, sender_last_seen_at, receiver_last_seen_at, locked)
VALUES (101, 'Read status test', 100, 101, '2020-01-01 00:00:00+00', NULL, FALSE);

INSERT INTO conversation_messages (id, conversation_id, created_by_id, content)
VALUES (101, 101, 100, 'Hello');

-- Unlocked conversation where the sender (user 160) holds read_all_conversations.
-- Used to verify that the read status still updates for a member who also has that permission.
INSERT INTO conversations (id, subject, sender_id, receiver_id, sender_last_seen_at, receiver_last_seen_at, locked)
VALUES (102, 'Read status test (read-all member)', 160, 100, '2020-01-01 00:00:00+00', NULL, FALSE);

INSERT INTO conversation_messages (id, conversation_id, created_by_id, content)
VALUES (102, 102, 100, 'Reply from receiver');
