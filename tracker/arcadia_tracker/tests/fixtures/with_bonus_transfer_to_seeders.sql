UPDATE arcadia_settings SET snatched_torrent_bonus_points_transferred_to = 'current_seeders';

-- Add a second user to be a seeder
INSERT INTO users (id, username, email, password_hash, registered_from_ip, passkey, class_name, css_sheet_name, bonus_points)
VALUES (20, 'seeder_user', 'seeder@testdomain.com', '$argon2id$v=19$m=19456,t=2,p=1$WM6V9pJ2ya7+N+NNIUtolg$n128u9idizCHLwZ9xhKaxOttLaAVZZgvfRZlRAnfyKk', '10.10.5.2', 'h6037c66dd3e13044e0d2f9b891c3841', 'newbie', 'arcadia', 0);

-- Add 2 seeders for torrent 100 (the torrent with snatch cost)
INSERT INTO peers (torrent_id, peer_id, ip, port, user_id, agent, uploaded, downloaded, "left", seeder, active, created_at, updated_at)
VALUES
    (100, '\xAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA', '10.10.5.1', 6881, 1, 'test-agent/1.0', 1000, 0, 0, true, true, NOW(), NOW()),
    (100, '\xBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBB', '10.10.5.2', 6882, 20, 'test-agent/1.0', 1000, 0, 0, true, true, NOW(), NOW());
