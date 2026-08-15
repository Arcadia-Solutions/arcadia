-- Comments written by user_basic (100), on a title group and on a torrent request
INSERT INTO title_group_comments (id, title_group_id, created_by_id, content)
VALUES (100, 1, 100, 'Comment of the basic user on a title group');

UPDATE users SET title_group_comments = 1 WHERE id = 100;

INSERT INTO torrent_request_comments (id, torrent_request_id, created_by_id, content)
VALUES (100, 1, 100, 'Comment of the basic user on a torrent request');

UPDATE users SET request_comments = 1 WHERE id = 100;
