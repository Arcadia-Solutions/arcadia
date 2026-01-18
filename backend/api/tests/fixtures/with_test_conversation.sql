-- Locked conversation between user 100 and user 101
INSERT INTO conversations (id, subject, sender_id, receiver_id, locked)
VALUES (100, 'Locked Conversation', 100, 101, TRUE);

-- Initial message for the locked conversation
INSERT INTO conversation_messages (id, conversation_id, created_by_id, content)
VALUES (100, 100, 100, 'This conversation is now locked');
