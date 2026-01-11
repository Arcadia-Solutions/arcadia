-- Active seeder peer for user 100 (user_basic)
INSERT INTO peers (peer_id, ip, port, agent, uploaded, downloaded, "left", seeder, torrent_id, user_id, active)
VALUES (E'\\x0102030405060708091011121314151617181920', '192.168.1.100', 6881, 'qBittorrent/4.5.0', 1073741824, 0, 0, true, 1, 100, true);

-- Active seeder peer for user 139 (user_view_peers)
INSERT INTO peers (peer_id, ip, port, agent, uploaded, downloaded, "left", seeder, torrent_id, user_id, active)
VALUES (E'\\x2122232425262728293031323334353637383940', '192.168.1.139', 6882, 'Transmission/3.0', 536870912, 0, 0, true, 1, 139, true);

-- Active leecher peer for user 101 (user_edit_art)
INSERT INTO peers (peer_id, ip, port, agent, uploaded, downloaded, "left", seeder, torrent_id, user_id, active)
VALUES (E'\\x4142434445464748495051525354555657585960', '192.168.1.101', 6883, 'Deluge/2.1.1', 0, 268435456, 805306368, false, 1, 101, true);

-- Inactive peer (should not be returned)
INSERT INTO peers (peer_id, ip, port, agent, uploaded, downloaded, "left", seeder, torrent_id, user_id, active)
VALUES (E'\\x6162636465666768697071727374757677787980', '192.168.1.102', 6884, 'rtorrent/0.9.8', 0, 0, 1073741824, false, 1, 102, false);
