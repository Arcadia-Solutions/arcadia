INSERT INTO
  forum_polls (id, forum_thread_id, question, created_at, created_by_id)
VALUES
  (100, 100, 'Favorite color?', '2025-01-02 10:00:00+00', 100);

INSERT INTO
  forum_poll_options (id, forum_poll_id, content, sort_order)
VALUES
  (100, 100, 'Red', 1),
  (101, 100, 'Blue', 2);
