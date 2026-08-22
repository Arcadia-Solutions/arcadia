-- Move a few of the test users into a known period (every other user keeps its
-- default created_at of NOW(), which is outside the tested range).
UPDATE users SET created_at = '2025-01-10 10:00:00+00' WHERE id = 100;
UPDATE users SET created_at = '2025-01-20 10:00:00+00' WHERE id = 101;
UPDATE users SET created_at = '2025-02-05 10:00:00+00' WHERE id = 102;
