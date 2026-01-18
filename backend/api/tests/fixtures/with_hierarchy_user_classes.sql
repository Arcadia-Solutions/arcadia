-- User class hierarchy for testing 1-hop promotion/demotion
-- Hierarchy: basic_class -> advanced_class

INSERT INTO user_classes (name, new_permissions, previous_user_class)
VALUES ('basic_class', '{download_torrent}', NULL);

INSERT INTO user_classes (name, new_permissions, previous_user_class)
VALUES ('advanced_class', '{upload_torrent}', 'basic_class');

-- Test users for hierarchy promotion/demotion tests
INSERT INTO users (id, username, email, password_hash, passkey, class_name, permissions, registered_from_ip, css_sheet_name)
VALUES (1000, 'test_user', 'test@example.com', 'hash', 'passkey123', 'basic_class', '{download_torrent}', '127.0.0.1', 'arcadia');

INSERT INTO users (id, username, email, password_hash, passkey, class_name, permissions, registered_from_ip, css_sheet_name)
VALUES (1001, 'advanced_user', 'advanced@example.com', 'hash', 'passkey456', 'advanced_class', '{download_torrent, upload_torrent}', '127.0.0.1', 'arcadia');

INSERT INTO users (id, username, email, password_hash, passkey, class_name, permissions, registered_from_ip, css_sheet_name)
VALUES (1002, 'lateral_user', 'lateral@example.com', 'hash', 'passkey789', 'basic_class', '{download_torrent}', '127.0.0.1', 'arcadia');

INSERT INTO users (id, username, email, password_hash, passkey, class_name, permissions, registered_from_ip, css_sheet_name)
VALUES (1003, 'duplicate_user', 'dup@example.com', 'hash', 'passkey999', 'basic_class', '{download_torrent, download_torrent, upload_torrent}', '127.0.0.1', 'arcadia');
