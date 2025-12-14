-- Basic user for read operations and permission denial tests
INSERT INTO users (id, username, email, password_hash, registered_from_ip, passkey, class_name, permissions)
VALUES (100, 'user_basic', 'test_user@testdomain.com', '$argon2id$v=19$m=19456,t=2,p=1$WM6V9pJ2ya7+N+NNIUtolg$n128u9idizCHLwZ9xhKaxOttLaAVZZgvfRZlRAnfyKk', '10.10.4.88', 'd2037c66dd3e13044e0d2f9b891c3837', 'newbie', '{download_torrent}');

-- User with edit_artist permission
INSERT INTO users (id, username, email, password_hash, registered_from_ip, passkey, class_name, permissions)
VALUES (101, 'user_edit_art', 'test_user_edit_artist@testdomain.com', '$argon2id$v=19$m=19456,t=2,p=1$WM6V9pJ2ya7+N+NNIUtolg$n128u9idizCHLwZ9xhKaxOttLaAVZZgvfRZlRAnfyKk', '10.10.4.88', 'd2037c66dd3e13044e0d2f9b891c3838', 'newbie', '{edit_artist}');

-- User with edit_series permission
INSERT INTO users (id, username, email, password_hash, registered_from_ip, passkey, class_name, permissions)
VALUES (102, 'user_edit_ser', 'test_user_edit_series@testdomain.com', '$argon2id$v=19$m=19456,t=2,p=1$WM6V9pJ2ya7+N+NNIUtolg$n128u9idizCHLwZ9xhKaxOttLaAVZZgvfRZlRAnfyKk', '10.10.4.88', 'd2037c66dd3e13044e0d2f9b891c3839', 'newbie', '{edit_series}');

-- User with edit_title_group_comment permission
INSERT INTO users (id, username, email, password_hash, registered_from_ip, passkey, class_name, permissions)
VALUES (103, 'user_edit_tgc', 'test_user_edit_title_group_comment@testdomain.com', '$argon2id$v=19$m=19456,t=2,p=1$WM6V9pJ2ya7+N+NNIUtolg$n128u9idizCHLwZ9xhKaxOttLaAVZZgvfRZlRAnfyKk', '10.10.4.88', 'd2037c66dd3e13044e0d2f9b891c383a', 'newbie', '{edit_title_group_comment}');

-- User with create_css_sheet permission
INSERT INTO users (id, username, email, password_hash, registered_from_ip, passkey, class_name, permissions)
VALUES (104, 'user_css_crt', 'test_user_create_css_sheet@testdomain.com', '$argon2id$v=19$m=19456,t=2,p=1$WM6V9pJ2ya7+N+NNIUtolg$n128u9idizCHLwZ9xhKaxOttLaAVZZgvfRZlRAnfyKk', '10.10.4.88', 'd2037c66dd3e13044e0d2f9b891c383b', 'newbie', '{create_css_sheet}');

-- User with edit_css_sheet permission
INSERT INTO users (id, username, email, password_hash, registered_from_ip, passkey, class_name, permissions)
VALUES (105, 'user_css_edit', 'test_user_edit_css_sheet@testdomain.com', '$argon2id$v=19$m=19456,t=2,p=1$WM6V9pJ2ya7+N+NNIUtolg$n128u9idizCHLwZ9xhKaxOttLaAVZZgvfRZlRAnfyKk', '10.10.4.88', 'd2037c66dd3e13044e0d2f9b891c383c', 'newbie', '{edit_css_sheet}');

-- User with set_default_css_sheet permission
INSERT INTO users (id, username, email, password_hash, registered_from_ip, passkey, class_name, permissions)
VALUES (106, 'user_css_def', 'test_user_set_default_css_sheet@testdomain.com', '$argon2id$v=19$m=19456,t=2,p=1$WM6V9pJ2ya7+N+NNIUtolg$n128u9idizCHLwZ9xhKaxOttLaAVZZgvfRZlRAnfyKk', '10.10.4.88', 'd2037c66dd3e13044e0d2f9b891c383d', 'newbie', '{set_default_css_sheet}');

-- User with create_forum_category permission
INSERT INTO users (id, username, email, password_hash, registered_from_ip, passkey, class_name, permissions)
VALUES (107, 'user_cat_crt', 'test_user_create_forum_category@testdomain.com', '$argon2id$v=19$m=19456,t=2,p=1$WM6V9pJ2ya7+N+NNIUtolg$n128u9idizCHLwZ9xhKaxOttLaAVZZgvfRZlRAnfyKk', '10.10.4.88', 'd2037c66dd3e13044e0d2f9b891c383e', 'newbie', '{create_forum_category}');

-- User with edit_forum_category permission
INSERT INTO users (id, username, email, password_hash, registered_from_ip, passkey, class_name, permissions)
VALUES (108, 'user_cat_edit', 'test_user_edit_forum_category@testdomain.com', '$argon2id$v=19$m=19456,t=2,p=1$WM6V9pJ2ya7+N+NNIUtolg$n128u9idizCHLwZ9xhKaxOttLaAVZZgvfRZlRAnfyKk', '10.10.4.88', 'd2037c66dd3e13044e0d2f9b891c383f', 'newbie', '{edit_forum_category}');

-- User with create_forum_sub_category permission
INSERT INTO users (id, username, email, password_hash, registered_from_ip, passkey, class_name, permissions)
VALUES (109, 'user_sub_crt', 'test_user_create_forum_sub_category@testdomain.com', '$argon2id$v=19$m=19456,t=2,p=1$WM6V9pJ2ya7+N+NNIUtolg$n128u9idizCHLwZ9xhKaxOttLaAVZZgvfRZlRAnfyKk', '10.10.4.88', 'd2037c66dd3e13044e0d2f9b891c3840', 'newbie', '{create_forum_sub_category}');

-- User with edit_forum_sub_category permission
INSERT INTO users (id, username, email, password_hash, registered_from_ip, passkey, class_name, permissions)
VALUES (110, 'user_sub_edit', 'test_user_edit_forum_sub_category@testdomain.com', '$argon2id$v=19$m=19456,t=2,p=1$WM6V9pJ2ya7+N+NNIUtolg$n128u9idizCHLwZ9xhKaxOttLaAVZZgvfRZlRAnfyKk', '10.10.4.88', 'd2037c66dd3e13044e0d2f9b891c3841', 'newbie', '{edit_forum_sub_category}');

-- User with edit_forum_thread permission
INSERT INTO users (id, username, email, password_hash, registered_from_ip, passkey, class_name, permissions)
VALUES (111, 'user_thr_edit', 'test_user_edit_forum_thread@testdomain.com', '$argon2id$v=19$m=19456,t=2,p=1$WM6V9pJ2ya7+N+NNIUtolg$n128u9idizCHLwZ9xhKaxOttLaAVZZgvfRZlRAnfyKk', '10.10.4.88', 'd2037c66dd3e13044e0d2f9b891c3842', 'newbie', '{edit_forum_thread}');

-- User with edit_forum_post permission
INSERT INTO users (id, username, email, password_hash, registered_from_ip, passkey, class_name, permissions)
VALUES (112, 'user_post_edit', 'test_user_edit_forum_post@testdomain.com', '$argon2id$v=19$m=19456,t=2,p=1$WM6V9pJ2ya7+N+NNIUtolg$n128u9idizCHLwZ9xhKaxOttLaAVZZgvfRZlRAnfyKk', '10.10.4.88', 'd2037c66dd3e13044e0d2f9b891c3843', 'newbie', '{edit_forum_post}');

-- User with both create and edit forum category permissions (for flow tests)
INSERT INTO users (id, username, email, password_hash, registered_from_ip, passkey, class_name, permissions)
VALUES (113, 'user_cat_flow', 'test_user_cat_flow@testdomain.com', '$argon2id$v=19$m=19456,t=2,p=1$WM6V9pJ2ya7+N+NNIUtolg$n128u9idizCHLwZ9xhKaxOttLaAVZZgvfRZlRAnfyKk', '10.10.4.88', 'd2037c66dd3e13044e0d2f9b891c3844', 'newbie', '{create_forum_category,edit_forum_category}');

-- User with both create and edit forum sub category permissions (for flow tests)
INSERT INTO users (id, username, email, password_hash, registered_from_ip, passkey, class_name, permissions)
VALUES (114, 'user_sub_flow', 'test_user_sub_flow@testdomain.com', '$argon2id$v=19$m=19456,t=2,p=1$WM6V9pJ2ya7+N+NNIUtolg$n128u9idizCHLwZ9xhKaxOttLaAVZZgvfRZlRAnfyKk', '10.10.4.88', 'd2037c66dd3e13044e0d2f9b891c3845', 'newbie', '{create_forum_sub_category,edit_forum_sub_category}');
