-- User with an expired warning (no ban)
INSERT INTO users (id, username, email, password_hash, registered_from_ip, passkey, class_name, css_sheet_name, permissions, warned)
VALUES (300, 'usr_exp_warn', 'expired_warn@testdomain.com', '$argon2id$v=19$m=19456,t=2,p=1$WM6V9pJ2ya7+N+NNIUtolg$n128u9idizCHLwZ9xhKaxOttLaAVZZgvfRZlRAnfyKk', '10.10.4.88', 'd2037c66dd3e13044e0d2f9b891c3a00', 'newbie', 'arcadia', '{download_torrent}', true);
INSERT INTO user_warnings (id, user_id, expires_at, reason, created_by_id, ban)
VALUES (300, 300, NOW() - INTERVAL '1 day', 'Expired warning', 1, false);

-- User with an active (non-expired) warning
INSERT INTO users (id, username, email, password_hash, registered_from_ip, passkey, class_name, css_sheet_name, permissions, warned)
VALUES (301, 'usr_act_warn', 'active_warn@testdomain.com', '$argon2id$v=19$m=19456,t=2,p=1$WM6V9pJ2ya7+N+NNIUtolg$n128u9idizCHLwZ9xhKaxOttLaAVZZgvfRZlRAnfyKk', '10.10.4.88', 'd2037c66dd3e13044e0d2f9b891c3a01', 'newbie', 'arcadia', '{download_torrent}', true);
INSERT INTO user_warnings (id, user_id, expires_at, reason, created_by_id, ban)
VALUES (301, 301, NOW() + INTERVAL '7 days', 'Active warning', 1, false);

-- User with a permanent warning (no expiry)
INSERT INTO users (id, username, email, password_hash, registered_from_ip, passkey, class_name, css_sheet_name, permissions, warned)
VALUES (302, 'usr_perm_warn', 'perm_warn@testdomain.com', '$argon2id$v=19$m=19456,t=2,p=1$WM6V9pJ2ya7+N+NNIUtolg$n128u9idizCHLwZ9xhKaxOttLaAVZZgvfRZlRAnfyKk', '10.10.4.88', 'd2037c66dd3e13044e0d2f9b891c3a02', 'newbie', 'arcadia', '{download_torrent}', true);
INSERT INTO user_warnings (id, user_id, expires_at, reason, created_by_id, ban)
VALUES (302, 302, NULL, 'Permanent warning', 1, false);

-- User with an expired ban-warning
INSERT INTO users (id, username, email, password_hash, registered_from_ip, passkey, class_name, css_sheet_name, permissions, warned, banned)
VALUES (303, 'usr_exp_ban', 'expired_ban@testdomain.com', '$argon2id$v=19$m=19456,t=2,p=1$WM6V9pJ2ya7+N+NNIUtolg$n128u9idizCHLwZ9xhKaxOttLaAVZZgvfRZlRAnfyKk', '10.10.4.88', 'd2037c66dd3e13044e0d2f9b891c3a03', 'newbie', 'arcadia', '{download_torrent}', true, true);
INSERT INTO user_warnings (id, user_id, expires_at, reason, created_by_id, ban)
VALUES (303, 303, NOW() - INTERVAL '1 day', 'Expired ban', 1, true);

-- User with an expired ban-warning AND an active ban-warning (should stay banned)
INSERT INTO users (id, username, email, password_hash, registered_from_ip, passkey, class_name, css_sheet_name, permissions, warned, banned)
VALUES (304, 'usr_mix_ban', 'mixed_ban@testdomain.com', '$argon2id$v=19$m=19456,t=2,p=1$WM6V9pJ2ya7+N+NNIUtolg$n128u9idizCHLwZ9xhKaxOttLaAVZZgvfRZlRAnfyKk', '10.10.4.88', 'd2037c66dd3e13044e0d2f9b891c3a04', 'newbie', 'arcadia', '{download_torrent}', true, true);
INSERT INTO user_warnings (id, user_id, expires_at, reason, created_by_id, ban)
VALUES (304, 304, NOW() - INTERVAL '1 day', 'Expired ban', 1, true);
INSERT INTO user_warnings (id, user_id, expires_at, reason, created_by_id, ban)
VALUES (305, 304, NOW() + INTERVAL '7 days', 'Active ban', 1, true);

-- User with an expired warning AND an active warning (should stay warned)
INSERT INTO users (id, username, email, password_hash, registered_from_ip, passkey, class_name, css_sheet_name, permissions, warned)
VALUES (306, 'usr_mix_warn', 'mixed_warn@testdomain.com', '$argon2id$v=19$m=19456,t=2,p=1$WM6V9pJ2ya7+N+NNIUtolg$n128u9idizCHLwZ9xhKaxOttLaAVZZgvfRZlRAnfyKk', '10.10.4.88', 'd2037c66dd3e13044e0d2f9b891c3a06', 'newbie', 'arcadia', '{download_torrent}', true);
INSERT INTO user_warnings (id, user_id, expires_at, reason, created_by_id, ban)
VALUES (306, 306, NOW() - INTERVAL '1 day', 'Expired warning', 1, false);
INSERT INTO user_warnings (id, user_id, expires_at, reason, created_by_id, ban)
VALUES (307, 306, NOW() + INTERVAL '7 days', 'Active warning', 1, false);
