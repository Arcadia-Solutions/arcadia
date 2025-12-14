-- User with locked class
INSERT INTO users (id, username, email, password_hash, registered_from_ip, passkey, class_name, class_locked, permissions)
VALUES (999, 'locked_user', 'locked@testdomain.com', '$argon2id$v=19$m=19456,t=2,p=1$WM6V9pJ2ya7+N+NNIUtolg$n128u9idizCHLwZ9xhKaxOttLaAVZZgvfRZlRAnfyKk', '10.10.4.88', 'd2037c66dd3e13044e0d2f9b891c9999', 'newbie', TRUE, '{download_torrent}');
