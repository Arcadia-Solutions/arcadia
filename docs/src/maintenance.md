# Cached counts

Multiple DB attributes act as "cached SQL COUNTs" that are automatically updated when needed.
However, if they are out of sync for whatever reason (manual db edits for example), they can be manually updated with those queries:

```SQL
-- Update artists.title_groups_amount
UPDATE artists
SET title_groups_amount = (
    SELECT COUNT(DISTINCT title_group_id)
    FROM affiliated_artists
    WHERE affiliated_artists.artist_id = artists.id
);
-- Update artists.edition_groups_amount
UPDATE artists
SET edition_groups_amount = (
    SELECT COUNT(DISTINCT eg.id)
    FROM edition_groups eg
    WHERE eg.title_group_id IN (
        SELECT DISTINCT title_group_id
        FROM affiliated_artists
        WHERE affiliated_artists.artist_id = artists.id
    )
);
-- Update artists.torrents_amount
UPDATE artists
SET torrents_amount = (
    SELECT COUNT(DISTINCT t.id)
    FROM torrents t
    JOIN edition_groups eg ON t.edition_group_id = eg.id
    WHERE eg.title_group_id IN (
        SELECT DISTINCT title_group_id
        FROM affiliated_artists
        WHERE affiliated_artists.artist_id = artists.id
    )
);
-- Update users.forum_posts
UPDATE users
SET forum_posts = (
    SELECT COUNT(*)
    FROM forum_posts
    WHERE forum_posts.created_by_id = users.id
);
-- Update users.forum_threads
UPDATE users
SET forum_threads = (
    SELECT COUNT(*)
    FROM forum_threads
    WHERE forum_threads.created_by_id = users.id
);
-- Update users.torrent_comments
UPDATE users
SET torrent_comments = (
    SELECT COUNT(*)
    FROM title_group_comments
    WHERE title_group_comments.created_by_id = users.id
);
-- Update users.requests_voted
UPDATE users
SET requests_voted = (
    SELECT COUNT(DISTINCT torrent_request_id)
    FROM torrent_request_votes
    WHERE torrent_request_votes.created_by_id = users.id
);
-- Update users.requests_filled
UPDATE users
SET requests_filled = (
    SELECT COUNT(*)
    FROM torrent_requests
    WHERE torrent_requests.filled_by_user_id = users.id
);
-- Update users.collages_started
UPDATE users
SET collages_started = (
    SELECT COUNT(*)
    FROM collage
    WHERE collage.created_by_id = users.id
);
-- Update users.title_groups
UPDATE users
SET title_groups = (
    SELECT COUNT(*)
    FROM title_groups
    WHERE title_groups.created_by_id = users.id
);
-- Update users.edition_groups
UPDATE users
SET edition_groups = (
    SELECT COUNT(*)
    FROM edition_groups
    WHERE edition_groups.created_by_id = users.id
);
-- Update users.torrents
UPDATE users
SET torrents = (
    SELECT COUNT(*)
    FROM torrents
    WHERE torrents.created_by_id = users.id
);
```
