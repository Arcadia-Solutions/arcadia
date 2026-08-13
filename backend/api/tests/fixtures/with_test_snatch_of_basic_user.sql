-- The basic user (id 100) snatched the torrent they uploaded publicly
INSERT INTO
  torrent_activities (torrent_id, user_id, grabbed_at, completed_at)
VALUES
  (901, 100, NOW(), NOW());
