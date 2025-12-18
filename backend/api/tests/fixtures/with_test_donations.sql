-- Test donations created by user_don_crt (id 122)
-- Donations from user 100 (user_basic)
INSERT INTO donations (id, donated_by_id, donated_at, created_by_id, amount, note)
VALUES (1, 100, '2024-01-15 10:00:00+00', 122, 50.0, 'First donation from user_basic');

INSERT INTO donations (id, donated_by_id, donated_at, created_by_id, amount, note)
VALUES (2, 100, '2024-02-20 14:30:00+00', 122, 75.50, 'Second donation from user_basic');

-- Donations from user 101 (user_edit_art)
INSERT INTO donations (id, donated_by_id, donated_at, created_by_id, amount, note)
VALUES (3, 101, '2024-01-20 09:15:00+00', 122, 100.0, 'Generous donation');

-- Donations from user 102 (user_edit_ser)
INSERT INTO donations (id, donated_by_id, donated_at, created_by_id, amount, note)
VALUES (4, 102, '2024-03-10 16:45:00+00', 122, 25.0, NULL);

INSERT INTO donations (id, donated_by_id, donated_at, created_by_id, amount, note)
VALUES (5, 102, '2024-03-15 11:20:00+00', 122, 30.0, 'Follow-up donation');

-- Donation created by different user (user 100)
INSERT INTO donations (id, donated_by_id, donated_at, created_by_id, amount, note)
VALUES (6, 103, '2024-04-01 08:00:00+00', 100, 150.0, 'Created by basic user');

-- Reset sequence
SELECT setval('donations_id_seq', 6);
