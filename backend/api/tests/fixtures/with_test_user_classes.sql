-- Test user classes
INSERT INTO user_classes (name, default_permissions)
VALUES ('test_class', '{upload_torrent,download_torrent}');

INSERT INTO user_classes (name, default_permissions)
VALUES ('empty_class', '{}');
