-- Three unpinned, unlocked threads in sub-category 100 with deliberately scrambled
-- created_at / posts_amount / views_count / name / latest-post-date values, so that
-- each thread sort mode produces a distinct ordering.
INSERT INTO
  forum_threads (id, forum_sub_category_id, name, created_at, created_by_id, posts_amount, pinned, locked, views_count)
VALUES
  (200, 100, 'Bravo Thread', '2025-02-01 08:00:00+00', 100, 3, false, false, 5),
  (201, 100, 'Alpha Thread', '2025-02-01 09:00:00+00', 100, 1, false, false, 30),
  (202, 100, 'Charlie Thread', '2025-02-01 10:00:00+00', 100, 5, false, false, 15);

INSERT INTO
  forum_posts (id, forum_thread_id, content, created_at, created_by_id)
VALUES
  (200, 200, 'Latest reply in Bravo Thread', '2025-02-01 20:00:00+00', 100),
  (201, 201, 'Latest reply in Alpha Thread', '2025-02-01 11:00:00+00', 100),
  (202, 202, 'Latest reply in Charlie Thread', '2025-02-01 15:00:00+00', 100);
