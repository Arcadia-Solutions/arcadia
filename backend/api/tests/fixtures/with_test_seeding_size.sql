-- Torrent sizes for seeding size calculation
-- Torrent 1: 100MB, Torrent 2: 200MB
UPDATE torrents SET size = 100000000 WHERE id = 1;
UPDATE torrents SET size = 200000000 WHERE id = 2;

-- Reset stats for test users
UPDATE users SET seeding_size = 0, seeding = 0, leeching = 0, snatched = 0 WHERE id IN (100, 101, 102);

-- User 100: seeds both torrents (should have 300MB seeding_size)
-- User 101: seeds torrent 1 with multiple peers (should have 100MB, not 200MB)
-- User 102: no peers (should remain at 0)
-- User 100: leeches torrent 2 from a second peer (seeder=false)
INSERT INTO peers (peer_id, ip, port, agent, uploaded, downloaded, "left", seeder, torrent_id, user_id, active)
VALUES
    -- User 100: one seeder peer per torrent
    (E'\\xc001000000000000000000000000000000000001', '10.0.0.1', 6881, 'test', 0, 0, 0, true, 1, 100, true),
    (E'\\xc001000000000000000000000000000000000002', '10.0.0.2', 6881, 'test', 0, 0, 0, true, 2, 100, true),
    -- User 100: one leeching peer on torrent 2 (different peer_id)
    (E'\\xc001000000000000000000000000000000000006', '10.0.0.6', 6884, 'test', 0, 0, 1000, false, 2, 100, true),
    -- User 101: multiple peers for torrent 1 (should count size only once)
    (E'\\xc001000000000000000000000000000000000003', '10.0.0.3', 6881, 'test', 0, 0, 0, true, 1, 101, true),
    (E'\\xc001000000000000000000000000000000000004', '10.0.0.4', 6882, 'test', 0, 0, 0, true, 1, 101, true),
    (E'\\xc001000000000000000000000000000000000005', '10.0.0.5', 6883, 'test', 0, 0, 0, true, 1, 101, true);

-- Torrent activities for snatched counts
-- User 100: completed (snatched) torrents 1 and 2
-- User 101: completed (snatched) torrent 1
INSERT INTO torrent_activities (torrent_id, user_id, grabbed_at, completed_at)
VALUES
    (1, 100, NOW(), NOW()),
    (2, 100, NOW(), NOW()),
    (1, 101, NOW(), NOW());
