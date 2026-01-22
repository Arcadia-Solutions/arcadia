INSERT INTO
    users (username, email, password_hash, registered_from_ip, passkey, class_name, css_sheet_name, max_snatches_per_day)
VALUES
    ('limited_user', 'test_limited@testdomain.com', '$argon2id$v=19$m=19456,t=2,p=1$WM6V9pJ2ya7+N+NNIUtolg$n128u9idizCHLwZ9xhKaxOttLaAVZZgvfRZlRAnfyKk', '10.10.4.88', 'e3037c66dd3e13044e0d2f9b891c3838', 'newbie', 'arcadia', 2);
