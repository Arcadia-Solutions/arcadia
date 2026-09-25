-- user 100: its peer announced successfully after the error, so the error is resolved
INSERT INTO peers (peer_id, ip, port, agent, uploaded, downloaded, "left", seeder, torrent_id, user_id, active, created_at, updated_at)
VALUES (E'\\x0101010101010101010101010101010101010101', '192.168.1.100', 6881, 'qBittorrent/5.0.0', 0, 0, 0, true, 1, 100, true, NOW() AT TIME ZONE 'UTC', NOW() AT TIME ZONE 'UTC');
INSERT INTO announce_errors (user_id, info_hash, torrent_id, error_code, peer_id, occurrences, first_seen_at, last_seen_at)
VALUES (100, E'\\x112233445566778899aabbccddeeff0011223344', 1, 'peers_per_torrent_per_user_limit', E'\\x0101010101010101010101010101010101010101', 4, NOW() - INTERVAL '2 hours', NOW() - INTERVAL '1 hour');

-- user 100: not seen for 2 days, so the error is stale
INSERT INTO announce_errors (user_id, info_hash, torrent_id, error_code, peer_id, occurrences, first_seen_at, last_seen_at)
VALUES (100, E'\\xffffffffffffffffffffffffffffffffffffffff', NULL, 'info_hash_not_found', E'\\x0101010101010101010101010101010101010101', 12, NOW() - INTERVAL '3 days', NOW() - INTERVAL '2 days');

-- user 101: only another peer of the user announced successfully, so the error is still current
INSERT INTO peers (peer_id, ip, port, agent, uploaded, downloaded, "left", seeder, torrent_id, user_id, active, created_at, updated_at)
VALUES (E'\\x0202020202020202020202020202020202020202', '192.168.1.101', 6882, 'Deluge/2.1.1', 0, 0, 0, true, 1, 101, true, NOW() AT TIME ZONE 'UTC', NOW() AT TIME ZONE 'UTC');
INSERT INTO announce_errors (user_id, info_hash, torrent_id, error_code, peer_id, occurrences, first_seen_at, last_seen_at)
VALUES (101, E'\\x112233445566778899aabbccddeeff0011223344', 1, 'peers_per_torrent_per_user_limit', E'\\x0303030303030303030303030303030303030303', 2, NOW() - INTERVAL '2 hours', NOW() - INTERVAL '1 hour');
