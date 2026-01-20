-- Peers for user 100 (user_basic) - Client 1: qBittorrent on 192.168.1.100:6881
-- First peer entry for this client (torrent 1)
INSERT INTO peers (peer_id, ip, port, agent, uploaded, downloaded, "left", seeder, created_at, updated_at, torrent_id, user_id, active)
VALUES (E'\\x0102030405060708091011121314151617181920', '192.168.1.100', 6881, 'qBittorrent/4.5.0', 1000000, 500000, 0, true, '2025-01-01 10:00:00', '2025-01-15 12:00:00', 1, 100, true);

-- Second peer entry for same client (torrent 2) - should aggregate with above
INSERT INTO peers (peer_id, ip, port, agent, uploaded, downloaded, "left", seeder, created_at, updated_at, torrent_id, user_id, active)
VALUES (E'\\x0102030405060708091011121314151617181921', '192.168.1.100', 6881, 'qBittorrent/4.5.0', 2000000, 300000, 0, true, '2025-01-05 08:00:00', '2025-01-20 14:00:00', 2, 100, true);

-- Peers for user 100 (user_basic) - Client 2: Transmission on 192.168.1.100:51413 (same IP, different port)
INSERT INTO peers (peer_id, ip, port, agent, uploaded, downloaded, "left", seeder, created_at, updated_at, torrent_id, user_id, active)
VALUES (E'\\x2122232425262728293031323334353637383940', '192.168.1.100', 51413, 'Transmission/3.0', 500000, 100000, 0, true, '2025-01-10 09:00:00', '2025-01-18 16:00:00', 1, 100, true);

-- Peer for user 101 (user_edit_art) - should not be returned for user 100
INSERT INTO peers (peer_id, ip, port, agent, uploaded, downloaded, "left", seeder, created_at, updated_at, torrent_id, user_id, active)
VALUES (E'\\x4142434445464748495051525354555657585960', '192.168.1.101', 6883, 'Deluge/2.1.1', 750000, 250000, 0, true, '2025-01-02 11:00:00', '2025-01-12 13:00:00', 1, 101, true);
