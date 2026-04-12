INSERT INTO conversations (id, subject, sender_id, receiver_id, locked)
VALUES (200, 'Music recommendations', 100, 101, FALSE);

INSERT INTO conversation_messages (id, conversation_id, created_by_id, content)
VALUES (200, 200, 100, 'Have you listened to any good jazz albums lately?');

INSERT INTO conversations (id, subject, sender_id, receiver_id, locked)
VALUES (201, 'Upload help', 100, 101, FALSE);

INSERT INTO conversation_messages (id, conversation_id, created_by_id, content)
VALUES (201, 201, 101, 'Make sure you keep seeding after upload');

INSERT INTO conversations (id, subject, sender_id, receiver_id, locked)
VALUES (202, 'Jazz collection', 100, 101, FALSE);

INSERT INTO conversation_messages (id, conversation_id, created_by_id, content)
VALUES (202, 202, 100, 'I have some rare torrents to share');
