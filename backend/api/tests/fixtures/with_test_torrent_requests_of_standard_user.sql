-- Torrent request created by user_basic (100), only voted on by its author
INSERT INTO torrent_requests (id, title_group_id, created_by_id, description)
VALUES (2, 1, 100, 'Request created by the basic user');

INSERT INTO torrent_request_votes (id, torrent_request_id, created_by_id, bounty_upload, bounty_bonus_points)
VALUES (2, 2, 100, 1000, 50);

-- Torrent request created by user_basic (100), also voted on by another user (101)
INSERT INTO torrent_requests (id, title_group_id, created_by_id, description)
VALUES (3, 1, 100, 'Request created by the basic user and voted on by someone else');

INSERT INTO torrent_request_votes (id, torrent_request_id, created_by_id, bounty_upload, bounty_bonus_points)
VALUES (3, 3, 100, 2000, 100), (4, 3, 101, 3000, 200);

-- Comment written on that request by the other user (101)
INSERT INTO torrent_request_comments (id, torrent_request_id, created_by_id, content)
VALUES (1, 3, 101, 'Comment on the request of the basic user');

UPDATE users SET request_comments = 1 WHERE id = 101;

-- Torrent request created by user_basic (100) and already filled by another user (101)
INSERT INTO torrent_requests (id, title_group_id, created_by_id, description, filled_by_user_id, filled_at)
VALUES (4, 1, 100, 'Request created by the basic user and already filled', 101, NOW());

INSERT INTO torrent_request_votes (id, torrent_request_id, created_by_id, bounty_upload, bounty_bonus_points)
VALUES (5, 4, 100, 1000, 50);

-- Reset sequences
SELECT setval('torrent_requests_id_seq', 4);
SELECT setval('torrent_request_votes_id_seq', 5);
SELECT setval('torrent_request_comments_id_seq', 1);
