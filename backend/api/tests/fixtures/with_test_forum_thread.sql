INSERT INTO
  forum_threads (id, forum_sub_category_id, name, created_at, created_by_id, posts_amount, sticky, locked)
VALUES
  (100, 100, 'Test Thread', '2025-01-01 10:00:00+00', 100, 1, false, false),
  (101, 100, 'Locked Thread', '2025-01-01 11:00:00+00', 100, 1, false, true),
  (102, 100, 'Sticky Thread', '2025-01-01 12:00:00+00', 100, 1, true, false),
  (103, 101, 'Thread in Different Sub Category', '2025-01-01 13:00:00+00', 100, 1, false, false);
