-- Seed a sub-category used by the test threads/posts (category id=1 already exists).
INSERT INTO forum_sub_categories (id, forum_category_id, name, sort_order, created_by_id)
VALUES (100, 1, 'Stats Sub Category', 10, 100);

-- January 2025: 2 threads by different users
INSERT INTO forum_threads (id, forum_sub_category_id, name, created_at, created_by_id, posts_amount, views_count)
VALUES
    (1000, 100, 'Stats Thread Alpha', '2025-01-10 10:00:00', 100, 0, 50),
    (1001, 100, 'Stats Thread Beta', '2025-01-20 10:00:00', 101, 0, 25);

-- February 2025: 1 thread
INSERT INTO forum_threads (id, forum_sub_category_id, name, created_at, created_by_id, posts_amount, views_count)
VALUES
    (1002, 100, 'Stats Thread Gamma', '2025-02-05 10:00:00', 100, 0, 10);

-- Posts: 3 in January (Alpha gets 2, Beta gets 1), 2 in February (Gamma gets 2)
INSERT INTO forum_posts (id, forum_thread_id, created_at, updated_at, created_by_id, content)
VALUES
    (2000, 1000, '2025-01-10 10:00:00', '2025-01-10 10:00:00', 100, 'hello'),
    (2001, 1000, '2025-01-11 10:00:00', '2025-01-11 10:00:00', 101, 'world!!'),
    (2002, 1001, '2025-01-20 10:00:00', '2025-01-20 10:00:00', 101, 'ping'),
    (2003, 1002, '2025-02-05 10:00:00', '2025-02-05 10:00:00', 100, 'february'),
    (2004, 1002, '2025-02-06 10:00:00', '2025-02-06 10:00:00', 100, 'again');
