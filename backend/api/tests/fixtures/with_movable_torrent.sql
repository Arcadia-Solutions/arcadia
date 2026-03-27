-- Two edition groups in the same title group (title_group 1) for move tests
-- Edition group 1 already exists from with_test_edition_group fixture
-- Add a third edition group in title_group 1
INSERT INTO edition_groups (id, title_group_id, name, created_by_id, covers, external_links, source)
VALUES (3, 1, 'Alternative Edition', 1, '{}', '{}', 'Web');

-- A torrent owned by user_basic (id=100) created recently (NOW) in edition_group 1
INSERT INTO torrents (id, edition_group_id, created_at, updated_at, created_by_id, info_hash, info_dict, languages, release_name, release_group, description, file_amount_per_type, uploaded_as_anonymous, file_list, mediainfo, trumpable, staff_checked, container, size)
VALUES (3, 1, NOW(), NOW(), 100, '\xdeadbeefdeadbeefdeadbeefdeadbeefdeadbeef', '{}', '{}', 'Movable Torrent', '', '', '{}', FALSE, '{}', '{}', FALSE, FALSE, 'mp3', 500000);

-- An old torrent owned by user_basic (id=100) created more than 24h ago in edition_group 1
INSERT INTO torrents (id, edition_group_id, created_at, updated_at, created_by_id, info_hash, info_dict, languages, release_name, release_group, description, file_amount_per_type, uploaded_as_anonymous, file_list, mediainfo, trumpable, staff_checked, container, size)
VALUES (4, 1, NOW() - INTERVAL '48 hours', NOW() - INTERVAL '48 hours', 100, '\xfeedface0123456789abcdef0123456789abcdef', '{}', '{}', 'Old Torrent', '', '', '{}', FALSE, '{}', '{}', FALSE, FALSE, 'mp3', 500000);
