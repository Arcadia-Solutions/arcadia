-- The user with the edit_artist permission (id 101) snatched the torrent
-- uploaded anonymously by the basic user (id 100)
INSERT INTO
  torrent_activities (torrent_id, user_id, grabbed_at, completed_at)
VALUES
  (902, 101, NOW(), NOW());
