-- Torrent sizes for seeding size calculation
-- Torrent 1: 100MB, Torrent 2: 200MB
UPDATE torrents SET size = 100000000 WHERE id = 1;
UPDATE torrents SET size = 200000000 WHERE id = 2;

-- Reset seeding_size for test users
UPDATE users SET seeding_size = 0 WHERE id IN (100, 101, 102);

-- User 100: seeds both torrents (should have 300MB seeding_size)
-- User 101: seeds torrent 1 with multiple peers (should have 100MB, not 200MB)
-- User 102: no peers (should remain at 0)
INSERT INTO peers (peer_id, ip, port, agent, uploaded, downloaded, "left", seeder, torrent_id, user_id, active)
VALUES
    -- User 100: one peer per torrent
    (E'\\xc001000000000000000000000000000000000001', '10.0.0.1', 6881, 'test', 0, 0, 0, true, 1, 100, true),
    (E'\\xc001000000000000000000000000000000000002', '10.0.0.2', 6881, 'test', 0, 0, 0, true, 2, 100, true),
    -- User 101: multiple peers for torrent 1 (should count size only once)
    (E'\\xc001000000000000000000000000000000000003', '10.0.0.3', 6881, 'test', 0, 0, 0, true, 1, 101, true),
    (E'\\xc001000000000000000000000000000000000004', '10.0.0.4', 6882, 'test', 0, 0, 0, true, 1, 101, true),
    (E'\\xc001000000000000000000000000000000000005', '10.0.0.5', 6883, 'test', 0, 0, 0, true, 1, 101, true);
