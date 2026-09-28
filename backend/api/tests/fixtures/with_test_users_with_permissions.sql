-- Test users for the permission filter of the user search, each with a different set of permissions
INSERT INTO users (id, username, email, password_hash, registered_from_ip, passkey, class_name, css_sheet_name, permissions, warned, banned)
VALUES
(210, 'perm_none', 'perm_none@testdomain.com', '$argon2id$v=19$m=19456,t=2,p=1$WM6V9pJ2ya7+N+NNIUtolg$n128u9idizCHLwZ9xhKaxOttLaAVZZgvfRZlRAnfyKk', '10.10.4.88', 'd2037c66dd3e13044e0d2f9b891c3910', 'newbie', 'arcadia', '{}', false, false),
(211, 'perm_download', 'perm_download@testdomain.com', '$argon2id$v=19$m=19456,t=2,p=1$WM6V9pJ2ya7+N+NNIUtolg$n128u9idizCHLwZ9xhKaxOttLaAVZZgvfRZlRAnfyKk', '10.10.4.88', 'd2037c66dd3e13044e0d2f9b891c3911', 'newbie', 'arcadia', '{download_torrent}', false, false),
(212, 'perm_upload', 'perm_upload@testdomain.com', '$argon2id$v=19$m=19456,t=2,p=1$WM6V9pJ2ya7+N+NNIUtolg$n128u9idizCHLwZ9xhKaxOttLaAVZZgvfRZlRAnfyKk', '10.10.4.88', 'd2037c66dd3e13044e0d2f9b891c3912', 'newbie', 'arcadia', '{upload_torrent}', false, false),
(213, 'perm_both', 'perm_both@testdomain.com', '$argon2id$v=19$m=19456,t=2,p=1$WM6V9pJ2ya7+N+NNIUtolg$n128u9idizCHLwZ9xhKaxOttLaAVZZgvfRZlRAnfyKk', '10.10.4.88', 'd2037c66dd3e13044e0d2f9b891c3913', 'newbie', 'arcadia', '{download_torrent,upload_torrent}', false, false);
