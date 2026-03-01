-- Inactive user: last seen 200 days ago, should be banned
INSERT INTO users (id, username, email, password_hash, registered_from_ip, passkey, class_name, css_sheet_name, permissions, last_seen)
VALUES (200, 'usr_inactive', 'inactive@testdomain.com', '$argon2id$v=19$m=19456,t=2,p=1$WM6V9pJ2ya7+N+NNIUtolg$n128u9idizCHLwZ9xhKaxOttLaAVZZgvfRZlRAnfyKk', '10.10.4.88', 'd2037c66dd3e13044e0d2f9b891c3900', 'newbie', 'arcadia', '{download_torrent}', NOW() - INTERVAL '200 days');

-- Active user: last seen 10 days ago, should NOT be banned
INSERT INTO users (id, username, email, password_hash, registered_from_ip, passkey, class_name, css_sheet_name, permissions, last_seen)
VALUES (201, 'usr_active', 'active@testdomain.com', '$argon2id$v=19$m=19456,t=2,p=1$WM6V9pJ2ya7+N+NNIUtolg$n128u9idizCHLwZ9xhKaxOttLaAVZZgvfRZlRAnfyKk', '10.10.4.88', 'd2037c66dd3e13044e0d2f9b891c3901', 'newbie', 'arcadia', '{download_torrent}', NOW() - INTERVAL '10 days');

-- Inactive user with immune_activity_pruning: last seen 200 days ago, should NOT be banned
INSERT INTO users (id, username, email, password_hash, registered_from_ip, passkey, class_name, css_sheet_name, permissions, last_seen)
VALUES (202, 'usr_immune', 'immune@testdomain.com', '$argon2id$v=19$m=19456,t=2,p=1$WM6V9pJ2ya7+N+NNIUtolg$n128u9idizCHLwZ9xhKaxOttLaAVZZgvfRZlRAnfyKk', '10.10.4.88', 'd2037c66dd3e13044e0d2f9b891c3902', 'newbie', 'arcadia', '{download_torrent,immune_activity_pruning}', NOW() - INTERVAL '200 days');

-- Already banned user: last seen 200 days ago, should NOT be counted
INSERT INTO users (id, username, email, password_hash, registered_from_ip, passkey, class_name, css_sheet_name, permissions, last_seen, banned)
VALUES (203, 'usr_banned', 'already_banned@testdomain.com', '$argon2id$v=19$m=19456,t=2,p=1$WM6V9pJ2ya7+N+NNIUtolg$n128u9idizCHLwZ9xhKaxOttLaAVZZgvfRZlRAnfyKk', '10.10.4.88', 'd2037c66dd3e13044e0d2f9b891c3903', 'newbie', 'arcadia', '{download_torrent}', NOW() - INTERVAL '200 days', true);
