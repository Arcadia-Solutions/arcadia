-- Basic user for read operations and permission denial tests
INSERT INTO users (id, username, email, password_hash, registered_from_ip, passkey, class_name, css_sheet_name, permissions)
VALUES (100, 'user_basic', 'test_user@testdomain.com', '$argon2id$v=19$m=19456,t=2,p=1$WM6V9pJ2ya7+N+NNIUtolg$n128u9idizCHLwZ9xhKaxOttLaAVZZgvfRZlRAnfyKk', '10.10.4.88', 'd2037c66dd3e13044e0d2f9b891c3837', 'newbie', 'arcadia', '{download_torrent,upload_torrent,create_torrent_request,create_forum_thread,create_forum_post}');

-- User with edit_artist permission
INSERT INTO users (id, username, email, password_hash, registered_from_ip, passkey, class_name, css_sheet_name, permissions)
VALUES (101, 'user_edit_art', 'test_user_edit_artist@testdomain.com', '$argon2id$v=19$m=19456,t=2,p=1$WM6V9pJ2ya7+N+NNIUtolg$n128u9idizCHLwZ9xhKaxOttLaAVZZgvfRZlRAnfyKk', '10.10.4.88', 'd2037c66dd3e13044e0d2f9b891c3838', 'newbie', 'arcadia', '{edit_artist,create_forum_thread,create_forum_post,upload_torrent,create_torrent_request}');

-- User with edit_series permission
INSERT INTO users (id, username, email, password_hash, registered_from_ip, passkey, class_name, css_sheet_name, permissions)
VALUES (102, 'user_edit_ser', 'test_user_edit_series@testdomain.com', '$argon2id$v=19$m=19456,t=2,p=1$WM6V9pJ2ya7+N+NNIUtolg$n128u9idizCHLwZ9xhKaxOttLaAVZZgvfRZlRAnfyKk', '10.10.4.88', 'd2037c66dd3e13044e0d2f9b891c3839', 'newbie', 'arcadia', '{edit_series}');

-- User with edit_title_group_comment permission
INSERT INTO users (id, username, email, password_hash, registered_from_ip, passkey, class_name, css_sheet_name, permissions)
VALUES (103, 'user_edit_tgc', 'test_user_edit_title_group_comment@testdomain.com', '$argon2id$v=19$m=19456,t=2,p=1$WM6V9pJ2ya7+N+NNIUtolg$n128u9idizCHLwZ9xhKaxOttLaAVZZgvfRZlRAnfyKk', '10.10.4.88', 'd2037c66dd3e13044e0d2f9b891c383a', 'newbie', 'arcadia', '{edit_title_group_comment}');

-- User with create_css_sheet permission
INSERT INTO users (id, username, email, password_hash, registered_from_ip, passkey, class_name, css_sheet_name, permissions)
VALUES (104, 'user_css_crt', 'test_user_create_css_sheet@testdomain.com', '$argon2id$v=19$m=19456,t=2,p=1$WM6V9pJ2ya7+N+NNIUtolg$n128u9idizCHLwZ9xhKaxOttLaAVZZgvfRZlRAnfyKk', '10.10.4.88', 'd2037c66dd3e13044e0d2f9b891c383b', 'newbie', 'arcadia', '{create_css_sheet}');

-- User with edit_css_sheet permission
INSERT INTO users (id, username, email, password_hash, registered_from_ip, passkey, class_name, css_sheet_name, permissions)
VALUES (105, 'user_css_edit', 'test_user_edit_css_sheet@testdomain.com', '$argon2id$v=19$m=19456,t=2,p=1$WM6V9pJ2ya7+N+NNIUtolg$n128u9idizCHLwZ9xhKaxOttLaAVZZgvfRZlRAnfyKk', '10.10.4.88', 'd2037c66dd3e13044e0d2f9b891c383c', 'newbie', 'arcadia', '{edit_css_sheet}');

-- User with create_forum_category permission
INSERT INTO users (id, username, email, password_hash, registered_from_ip, passkey, class_name, css_sheet_name, permissions)
VALUES (107, 'user_cat_crt', 'test_user_create_forum_category@testdomain.com', '$argon2id$v=19$m=19456,t=2,p=1$WM6V9pJ2ya7+N+NNIUtolg$n128u9idizCHLwZ9xhKaxOttLaAVZZgvfRZlRAnfyKk', '10.10.4.88', 'd2037c66dd3e13044e0d2f9b891c383e', 'newbie', 'arcadia', '{create_forum_category}');

-- User with edit_forum_category permission
INSERT INTO users (id, username, email, password_hash, registered_from_ip, passkey, class_name, css_sheet_name, permissions)
VALUES (108, 'user_cat_edit', 'test_user_edit_forum_category@testdomain.com', '$argon2id$v=19$m=19456,t=2,p=1$WM6V9pJ2ya7+N+NNIUtolg$n128u9idizCHLwZ9xhKaxOttLaAVZZgvfRZlRAnfyKk', '10.10.4.88', 'd2037c66dd3e13044e0d2f9b891c383f', 'newbie', 'arcadia', '{edit_forum_category}');

-- User with create_forum_sub_category permission
INSERT INTO users (id, username, email, password_hash, registered_from_ip, passkey, class_name, css_sheet_name, permissions)
VALUES (109, 'user_sub_crt', 'test_user_create_forum_sub_category@testdomain.com', '$argon2id$v=19$m=19456,t=2,p=1$WM6V9pJ2ya7+N+NNIUtolg$n128u9idizCHLwZ9xhKaxOttLaAVZZgvfRZlRAnfyKk', '10.10.4.88', 'd2037c66dd3e13044e0d2f9b891c3840', 'newbie', 'arcadia', '{create_forum_sub_category}');

-- User with edit_forum_sub_category permission
INSERT INTO users (id, username, email, password_hash, registered_from_ip, passkey, class_name, css_sheet_name, permissions)
VALUES (110, 'user_sub_edit', 'test_user_edit_forum_sub_category@testdomain.com', '$argon2id$v=19$m=19456,t=2,p=1$WM6V9pJ2ya7+N+NNIUtolg$n128u9idizCHLwZ9xhKaxOttLaAVZZgvfRZlRAnfyKk', '10.10.4.88', 'd2037c66dd3e13044e0d2f9b891c3841', 'newbie', 'arcadia', '{edit_forum_sub_category}');

-- User with edit_forum_thread permission
INSERT INTO users (id, username, email, password_hash, registered_from_ip, passkey, class_name, css_sheet_name, permissions)
VALUES (111, 'user_thr_edit', 'test_user_edit_forum_thread@testdomain.com', '$argon2id$v=19$m=19456,t=2,p=1$WM6V9pJ2ya7+N+NNIUtolg$n128u9idizCHLwZ9xhKaxOttLaAVZZgvfRZlRAnfyKk', '10.10.4.88', 'd2037c66dd3e13044e0d2f9b891c3842', 'newbie', 'arcadia', '{edit_forum_thread}');

-- User with edit_forum_post permission
INSERT INTO users (id, username, email, password_hash, registered_from_ip, passkey, class_name, css_sheet_name, permissions)
VALUES (112, 'user_post_edit', 'test_user_edit_forum_post@testdomain.com', '$argon2id$v=19$m=19456,t=2,p=1$WM6V9pJ2ya7+N+NNIUtolg$n128u9idizCHLwZ9xhKaxOttLaAVZZgvfRZlRAnfyKk', '10.10.4.88', 'd2037c66dd3e13044e0d2f9b891c3843', 'newbie', 'arcadia', '{edit_forum_post}');

-- User with both create and edit forum category permissions (for flow tests)
INSERT INTO users (id, username, email, password_hash, registered_from_ip, passkey, class_name, css_sheet_name, permissions)
VALUES (113, 'user_cat_flow', 'test_user_cat_flow@testdomain.com', '$argon2id$v=19$m=19456,t=2,p=1$WM6V9pJ2ya7+N+NNIUtolg$n128u9idizCHLwZ9xhKaxOttLaAVZZgvfRZlRAnfyKk', '10.10.4.88', 'd2037c66dd3e13044e0d2f9b891c3844', 'newbie', 'arcadia', '{create_forum_category,edit_forum_category}');

-- User with both create and edit forum sub category permissions (for flow tests)
INSERT INTO users (id, username, email, password_hash, registered_from_ip, passkey, class_name, css_sheet_name, permissions)
VALUES (114, 'user_sub_flow', 'test_user_sub_flow@testdomain.com', '$argon2id$v=19$m=19456,t=2,p=1$WM6V9pJ2ya7+N+NNIUtolg$n128u9idizCHLwZ9xhKaxOttLaAVZZgvfRZlRAnfyKk', '10.10.4.88', 'd2037c66dd3e13044e0d2f9b891c3845', 'newbie', 'arcadia', '{create_forum_sub_category,edit_forum_sub_category}');

-- User with create_user_class permission
INSERT INTO users (id, username, email, password_hash, registered_from_ip, passkey, class_name, css_sheet_name, permissions)
VALUES (115, 'user_cls_crt', 'test_user_create_user_class@testdomain.com', '$argon2id$v=19$m=19456,t=2,p=1$WM6V9pJ2ya7+N+NNIUtolg$n128u9idizCHLwZ9xhKaxOttLaAVZZgvfRZlRAnfyKk', '10.10.4.88', 'd2037c66dd3e13044e0d2f9b891c3846', 'newbie', 'arcadia', '{create_user_class}');

-- User with edit_user_class permission
INSERT INTO users (id, username, email, password_hash, registered_from_ip, passkey, class_name, css_sheet_name, permissions)
VALUES (116, 'user_cls_edit', 'test_user_edit_user_class@testdomain.com', '$argon2id$v=19$m=19456,t=2,p=1$WM6V9pJ2ya7+N+NNIUtolg$n128u9idizCHLwZ9xhKaxOttLaAVZZgvfRZlRAnfyKk', '10.10.4.88', 'd2037c66dd3e13044e0d2f9b891c3847', 'newbie', 'arcadia', '{edit_user_class}');

-- User with delete_user_class permission
INSERT INTO users (id, username, email, password_hash, registered_from_ip, passkey, class_name, css_sheet_name, permissions)
VALUES (117, 'user_cls_del', 'test_user_delete_user_class@testdomain.com', '$argon2id$v=19$m=19456,t=2,p=1$WM6V9pJ2ya7+N+NNIUtolg$n128u9idizCHLwZ9xhKaxOttLaAVZZgvfRZlRAnfyKk', '10.10.4.88', 'd2037c66dd3e13044e0d2f9b891c3848', 'newbie', 'arcadia', '{delete_user_class}');

-- User with edit_user_permissions permission
INSERT INTO users (id, username, email, password_hash, registered_from_ip, passkey, class_name, css_sheet_name, permissions)
VALUES (118, 'user_perm_edit', 'test_user_edit_permissions@testdomain.com', '$argon2id$v=19$m=19456,t=2,p=1$WM6V9pJ2ya7+N+NNIUtolg$n128u9idizCHLwZ9xhKaxOttLaAVZZgvfRZlRAnfyKk', '10.10.4.88', 'd2037c66dd3e13044e0d2f9b891c3849', 'newbie', 'arcadia', '{edit_user_permissions}');

-- User with lock_user_class permission
INSERT INTO users (id, username, email, password_hash, registered_from_ip, passkey, class_name, css_sheet_name, permissions)
VALUES (119, 'user_lock_cls', 'test_user_lock_class@testdomain.com', '$argon2id$v=19$m=19456,t=2,p=1$WM6V9pJ2ya7+N+NNIUtolg$n128u9idizCHLwZ9xhKaxOttLaAVZZgvfRZlRAnfyKk', '10.10.4.88', 'd2037c66dd3e13044e0d2f9b891c384a', 'newbie', 'arcadia', '{lock_user_class}');

-- User with change_user_class permission
INSERT INTO users (id, username, email, password_hash, registered_from_ip, passkey, class_name, css_sheet_name, permissions)
VALUES (120, 'user_cls_chg', 'test_user_change_class@testdomain.com', '$argon2id$v=19$m=19456,t=2,p=1$WM6V9pJ2ya7+N+NNIUtolg$n128u9idizCHLwZ9xhKaxOttLaAVZZgvfRZlRAnfyKk', '10.10.4.88', 'd2037c66dd3e13044e0d2f9b891c384b', 'newbie', 'arcadia', '{change_user_class}');

-- User with edit_arcadia_settings permission
INSERT INTO users (id, username, email, password_hash, registered_from_ip, passkey, class_name, css_sheet_name, permissions)
VALUES (121, 'user_arc_set', 'test_user_arcadia_settings@testdomain.com', '$argon2id$v=19$m=19456,t=2,p=1$WM6V9pJ2ya7+N+NNIUtolg$n128u9idizCHLwZ9xhKaxOttLaAVZZgvfRZlRAnfyKk', '10.10.4.88', 'd2037c66dd3e13044e0d2f9b891c384c', 'newbie', 'arcadia', '{edit_arcadia_settings}');

-- User with create_donation permission
INSERT INTO users (id, username, email, password_hash, registered_from_ip, passkey, class_name, css_sheet_name, permissions)
VALUES (122, 'user_don_crt', 'test_user_create_donation@testdomain.com', '$argon2id$v=19$m=19456,t=2,p=1$WM6V9pJ2ya7+N+NNIUtolg$n128u9idizCHLwZ9xhKaxOttLaAVZZgvfRZlRAnfyKk', '10.10.4.88', 'd2037c66dd3e13044e0d2f9b891c384d', 'newbie', 'arcadia', '{create_donation}');

-- User with edit_donation permission
INSERT INTO users (id, username, email, password_hash, registered_from_ip, passkey, class_name, css_sheet_name, permissions)
VALUES (123, 'user_don_edit', 'test_user_edit_donation@testdomain.com', '$argon2id$v=19$m=19456,t=2,p=1$WM6V9pJ2ya7+N+NNIUtolg$n128u9idizCHLwZ9xhKaxOttLaAVZZgvfRZlRAnfyKk', '10.10.4.88', 'd2037c66dd3e13044e0d2f9b891c384e', 'newbie', 'arcadia', '{edit_donation}');

-- User with delete_donation permission
INSERT INTO users (id, username, email, password_hash, registered_from_ip, passkey, class_name, css_sheet_name, permissions)
VALUES (124, 'user_don_del', 'test_user_delete_donation@testdomain.com', '$argon2id$v=19$m=19456,t=2,p=1$WM6V9pJ2ya7+N+NNIUtolg$n128u9idizCHLwZ9xhKaxOttLaAVZZgvfRZlRAnfyKk', '10.10.4.88', 'd2037c66dd3e13044e0d2f9b891c384f', 'newbie', 'arcadia', '{delete_donation}');

-- User with search_donation permission
INSERT INTO users (id, username, email, password_hash, registered_from_ip, passkey, class_name, css_sheet_name, permissions)
VALUES (125, 'user_don_srch', 'test_user_search_donation@testdomain.com', '$argon2id$v=19$m=19456,t=2,p=1$WM6V9pJ2ya7+N+NNIUtolg$n128u9idizCHLwZ9xhKaxOttLaAVZZgvfRZlRAnfyKk', '10.10.4.88', 'd2037c66dd3e13044e0d2f9b891c3850', 'newbie', 'arcadia', '{search_donation}');

-- User with warn_user and ban_user permissions
INSERT INTO users (id, username, email, password_hash, registered_from_ip, passkey, class_name, css_sheet_name, permissions)
VALUES (126, 'user_warn_ban', 'test_user_warn_ban@testdomain.com', '$argon2id$v=19$m=19456,t=2,p=1$WM6V9pJ2ya7+N+NNIUtolg$n128u9idizCHLwZ9xhKaxOttLaAVZZgvfRZlRAnfyKk', '10.10.4.88', 'd2037c66dd3e13044e0d2f9b891c3851', 'newbie', 'arcadia', '{warn_user,ban_user}');

-- User with search_unauthorized_access permission
INSERT INTO users (id, username, email, password_hash, registered_from_ip, passkey, class_name, css_sheet_name, permissions)
VALUES (127, 'user_unauth', 'test_user_unauth_search@testdomain.com', '$argon2id$v=19$m=19456,t=2,p=1$WM6V9pJ2ya7+N+NNIUtolg$n128u9idizCHLwZ9xhKaxOttLaAVZZgvfRZlRAnfyKk', '10.10.4.88', 'd2037c66dd3e13044e0d2f9b891c3852', 'newbie', 'arcadia', '{search_unauthorized_access}');

-- User with delete_forum_category permission
INSERT INTO users (id, username, email, password_hash, registered_from_ip, passkey, class_name, css_sheet_name, permissions)
VALUES (128, 'user_cat_del', 'test_user_delete_forum_category@testdomain.com', '$argon2id$v=19$m=19456,t=2,p=1$WM6V9pJ2ya7+N+NNIUtolg$n128u9idizCHLwZ9xhKaxOttLaAVZZgvfRZlRAnfyKk', '10.10.4.88', 'd2037c66dd3e13044e0d2f9b891c3853', 'newbie', 'arcadia', '{delete_forum_category}');

-- User with delete_forum_sub_category permission
INSERT INTO users (id, username, email, password_hash, registered_from_ip, passkey, class_name, css_sheet_name, permissions)
VALUES (129, 'user_sub_del', 'test_user_delete_forum_sub_category@testdomain.com', '$argon2id$v=19$m=19456,t=2,p=1$WM6V9pJ2ya7+N+NNIUtolg$n128u9idizCHLwZ9xhKaxOttLaAVZZgvfRZlRAnfyKk', '10.10.4.88', 'd2037c66dd3e13044e0d2f9b891c3854', 'newbie', 'arcadia', '{delete_forum_sub_category}');

-- User with delete_forum_thread permission
INSERT INTO users (id, username, email, password_hash, registered_from_ip, passkey, class_name, css_sheet_name, permissions)
VALUES (130, 'user_thr_del', 'test_user_delete_forum_thread@testdomain.com', '$argon2id$v=19$m=19456,t=2,p=1$WM6V9pJ2ya7+N+NNIUtolg$n128u9idizCHLwZ9xhKaxOttLaAVZZgvfRZlRAnfyKk', '10.10.4.88', 'd2037c66dd3e13044e0d2f9b891c3855', 'newbie', 'arcadia', '{delete_forum_thread}');

-- User with delete_forum_post permission
INSERT INTO users (id, username, email, password_hash, registered_from_ip, passkey, class_name, css_sheet_name, permissions)
VALUES (131, 'user_post_del', 'test_user_delete_forum_post@testdomain.com', '$argon2id$v=19$m=19456,t=2,p=1$WM6V9pJ2ya7+N+NNIUtolg$n128u9idizCHLwZ9xhKaxOttLaAVZZgvfRZlRAnfyKk', '10.10.4.88', 'd2037c66dd3e13044e0d2f9b891c3856', 'newbie', 'arcadia', '{delete_forum_post}');

-- User with delete_artist permission
INSERT INTO users (id, username, email, password_hash, registered_from_ip, passkey, class_name, css_sheet_name, permissions)
VALUES (132, 'user_art_del', 'test_user_delete_artist@testdomain.com', '$argon2id$v=19$m=19456,t=2,p=1$WM6V9pJ2ya7+N+NNIUtolg$n128u9idizCHLwZ9xhKaxOttLaAVZZgvfRZlRAnfyKk', '10.10.4.88', 'd2037c66dd3e13044e0d2f9b891c3857', 'newbie', 'arcadia', '{delete_artist}');

-- User with set_torrent_staff_checked permission
INSERT INTO users (id, username, email, password_hash, registered_from_ip, passkey, class_name, css_sheet_name, permissions)
VALUES (133, 'user_tor_stfc', 'test_user_set_torrent_staff_checked@testdomain.com', '$argon2id$v=19$m=19456,t=2,p=1$WM6V9pJ2ya7+N+NNIUtolg$n128u9idizCHLwZ9xhKaxOttLaAVZZgvfRZlRAnfyKk', '10.10.4.88', 'd2037c66dd3e13044e0d2f9b891c3858', 'newbie', 'arcadia', '{set_torrent_staff_checked}');

-- User with search_user_edit_change_logs permission
INSERT INTO users (id, username, email, password_hash, registered_from_ip, passkey, class_name, css_sheet_name, permissions)
VALUES (134, 'user_edit_log', 'test_user_search_edit_logs@testdomain.com', '$argon2id$v=19$m=19456,t=2,p=1$WM6V9pJ2ya7+N+NNIUtolg$n128u9idizCHLwZ9xhKaxOttLaAVZZgvfRZlRAnfyKk', '10.10.4.88', 'd2037c66dd3e13044e0d2f9b891c3859', 'newbie', 'arcadia', '{search_user_edit_change_logs}');

-- User with edit_edition_group permission
INSERT INTO users (id, username, email, password_hash, registered_from_ip, passkey, class_name, css_sheet_name, permissions)
VALUES (135, 'user_edit_eg', 'test_user_edit_edition_group@testdomain.com', '$argon2id$v=19$m=19456,t=2,p=1$WM6V9pJ2ya7+N+NNIUtolg$n128u9idizCHLwZ9xhKaxOttLaAVZZgvfRZlRAnfyKk', '10.10.4.88', 'd2037c66dd3e13044e0d2f9b891c385a', 'newbie', 'arcadia', '{edit_edition_group}');

-- User with edit_collage permission
INSERT INTO users (id, username, email, password_hash, registered_from_ip, passkey, class_name, css_sheet_name, permissions)
VALUES (136, 'user_col_edit', 'test_user_edit_collage@testdomain.com', '$argon2id$v=19$m=19456,t=2,p=1$WM6V9pJ2ya7+N+NNIUtolg$n128u9idizCHLwZ9xhKaxOttLaAVZZgvfRZlRAnfyKk', '10.10.4.88', 'd2037c66dd3e13044e0d2f9b891c385b', 'newbie', 'arcadia', '{edit_collage}');

-- User with delete_collage permission
INSERT INTO users (id, username, email, password_hash, registered_from_ip, passkey, class_name, css_sheet_name, permissions)
VALUES (137, 'user_col_del', 'test_user_delete_collage@testdomain.com', '$argon2id$v=19$m=19456,t=2,p=1$WM6V9pJ2ya7+N+NNIUtolg$n128u9idizCHLwZ9xhKaxOttLaAVZZgvfRZlRAnfyKk', '10.10.4.88', 'd2037c66dd3e13044e0d2f9b891c385c', 'newbie', 'arcadia', '{delete_collage}');

-- User with staff PM permissions (read_staff_pm, reply_staff_pm, resolve_staff_pm)
INSERT INTO users (id, username, email, password_hash, registered_from_ip, passkey, class_name, css_sheet_name, permissions)
VALUES (138, 'user_staff_pm', 'test_user_staff_pm@testdomain.com', '$argon2id$v=19$m=19456,t=2,p=1$WM6V9pJ2ya7+N+NNIUtolg$n128u9idizCHLwZ9xhKaxOttLaAVZZgvfRZlRAnfyKk', '10.10.4.88', 'd2037c66dd3e13044e0d2f9b891c385d', 'newbie', 'arcadia', '{read_staff_pm,reply_staff_pm,resolve_staff_pm}');

-- User with view_torrent_peers permission
INSERT INTO users (id, username, email, password_hash, registered_from_ip, passkey, class_name, css_sheet_name, permissions)
VALUES (139, 'user_view_peers', 'test_user_view_peers@testdomain.com', '$argon2id$v=19$m=19456,t=2,p=1$WM6V9pJ2ya7+N+NNIUtolg$n128u9idizCHLwZ9xhKaxOttLaAVZZgvfRZlRAnfyKk', '10.10.4.88', 'd2037c66dd3e13044e0d2f9b891c385e', 'newbie', 'arcadia', '{view_torrent_peers}');
