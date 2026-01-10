INSERT INTO
  title_group_comments (
    id,
    content,
    created_at,
    updated_at,
    created_by_id,
    title_group_id,
    locked,
    refers_to_torrent_id,
    answers_to_comment_id
  )
VALUES
  (
    1,
    'This is a great album!',
    '2025-03-30 16:35:06.418293+00',
    '2025-03-30 16:35:06.418293+00',
    1,
    1,
    FALSE,
    NULL,
    NULL
  ),
  (
    2,
    'Love this classic Beatles single.',
    '2025-03-30 16:36:00+00',
    '2025-03-30 16:36:00+00',
    1,
    1,
    FALSE,
    NULL,
    NULL
  ),
  (
    3,
    'RollerCoaster Tycoon is amazing!',
    '2025-03-30 16:40:00+00',
    '2025-03-30 16:40:00+00',
    1,
    2,
    FALSE,
    NULL,
    NULL
  ),
  (
    4,
    'Best simulation game ever made.',
    '2025-03-30 16:41:00+00',
    '2025-03-30 16:41:00+00',
    1,
    2,
    FALSE,
    NULL,
    NULL
  ),
  (
    5,
    'The music in this game is AMAZING too!',
    '2025-03-30 16:42:00+00',
    '2025-03-30 16:42:00+00',
    1,
    2,
    FALSE,
    NULL,
    NULL
  );
