-- User 100 seeds two torrents with qBittorrent: it must be counted once.
INSERT INTO peers (peer_id, ip, port, agent, uploaded, downloaded, "left", seeder, created_at, updated_at, torrent_id, user_id, active)
VALUES (E'\\x0102030405060708091011121314151617181920', '192.168.1.100', 6881, 'qBittorrent/4.5.0', 1000000, 500000, 0, true, '2025-01-01 10:00:00', '2025-01-15 12:00:00', 1, 100, true);

INSERT INTO peers (peer_id, ip, port, agent, uploaded, downloaded, "left", seeder, created_at, updated_at, torrent_id, user_id, active)
VALUES (E'\\x2122232425262728293031323334353637383940', '192.168.1.100', 6881, 'qBittorrent/4.5.0', 2000000, 300000, 0, true, '2025-01-05 08:00:00', '2025-01-20 14:00:00', 2, 100, true);

-- User 101 seeds a single torrent with Deluge.
INSERT INTO peers (peer_id, ip, port, agent, uploaded, downloaded, "left", seeder, created_at, updated_at, torrent_id, user_id, active)
VALUES (E'\\x4142434445464748495051525354555657585960', '192.168.1.101', 6883, 'Deluge/2.1.1', 750000, 250000, 0, true, '2025-01-02 11:00:00', '2025-01-12 13:00:00', 1, 101, true);
