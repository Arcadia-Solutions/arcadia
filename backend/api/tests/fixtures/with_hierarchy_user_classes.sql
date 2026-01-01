-- User class hierarchy for testing 1-hop promotion/demotion
-- Hierarchy: basic_class -> advanced_class

INSERT INTO user_classes (name, new_permissions, previous_user_class)
VALUES ('basic_class', '{download_torrent}', NULL);

INSERT INTO user_classes (name, new_permissions, previous_user_class)
VALUES ('advanced_class', '{upload_torrent}', 'basic_class');
