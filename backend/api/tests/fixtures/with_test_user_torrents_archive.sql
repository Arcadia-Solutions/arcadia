-- Make the basic user (id 100) both the uploader and a snatcher of torrent 1,
-- which has a valid info_dict, so the uploaded and snatched archives are non-empty.
UPDATE torrents SET created_by_id = 100 WHERE id = 1;

INSERT INTO
  torrent_activities (torrent_id, user_id, grabbed_at, completed_at)
VALUES
  (1, 100, NOW(), NOW());
