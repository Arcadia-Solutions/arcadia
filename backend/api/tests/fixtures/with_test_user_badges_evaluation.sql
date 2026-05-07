-- Auto badges for periodic-task evaluation
INSERT INTO user_badge_categories (id, name, created_by_id)
VALUES (700, 'auto_eval_category', 100);

-- Auto badge that user 100 qualifies for (2+ forum posts >= 30 chars)
INSERT INTO user_badges (id, name, description, image_url, category_id, badge_type, is_secret, revoke_when_criteria_unmet, criteria, created_by_id)
VALUES (
    700,
    'forum_poster',
    'awarded for 2+ posts of 30+ chars',
    'https://example.com/poster.png',
    700,
    'forum_posts',
    FALSE,
    TRUE,
    '{"type":"forum_posts","minimum_post_character_count":30,"required_substring":null,"minimum_post_amount":2}'::jsonb,
    100
);

-- Auto badge that nobody qualifies for (100+ posts)
INSERT INTO user_badges (id, name, description, image_url, category_id, badge_type, is_secret, revoke_when_criteria_unmet, criteria, created_by_id)
VALUES (
    701,
    'forum_centurion',
    'awarded for 100+ posts',
    'https://example.com/centurion.png',
    700,
    'forum_posts',
    FALSE,
    TRUE,
    '{"type":"forum_posts","minimum_post_character_count":1,"required_substring":null,"minimum_post_amount":100}'::jsonb,
    100
);

-- Auto badge that only matches posts containing the substring "doesnotexist" (nobody qualifies)
INSERT INTO user_badges (id, name, description, image_url, category_id, badge_type, is_secret, revoke_when_criteria_unmet, criteria, created_by_id)
VALUES (
    702,
    'no_revoke_when_unmet',
    'auto badge that does NOT revoke',
    'https://example.com/never.png',
    700,
    'forum_posts',
    FALSE,
    FALSE,
    '{"type":"forum_posts","minimum_post_character_count":1,"required_substring":"doesnotexist","minimum_post_amount":1}'::jsonb,
    100
);

-- Torrents-uploaded auto badge that user 1 (the seed "creator", 2 uploads in with_test_torrent fixture) qualifies for
INSERT INTO user_badges (id, name, description, image_url, category_id, badge_type, is_secret, revoke_when_criteria_unmet, criteria, created_by_id)
VALUES (
    800,
    'uploader_two',
    '2+ uploads',
    'https://example.com/two.png',
    700,
    'torrents_uploaded',
    FALSE,
    TRUE,
    '{"type":"torrents_uploaded","search":{"title_group_include_empty_groups":false,"page":1,"page_size":1,"order_by_column":"torrent_created_at","order_by_direction":"desc"},"minimum_title_group_amount":2}'::jsonb,
    100
);

-- Filtered torrents-uploaded badge: user 1's title groups narrowed by name substring "Love"
-- + content_type "music" + category "Single" yield exactly 1 match (TG 1), so threshold 2 is unmet.
INSERT INTO user_badges (id, name, description, image_url, category_id, badge_type, is_secret, revoke_when_criteria_unmet, criteria, created_by_id)
VALUES (
    801,
    'love_singles_two',
    '2+ music single uploads with "Love" in the name',
    'https://example.com/love2.png',
    700,
    'torrents_uploaded',
    FALSE,
    TRUE,
    '{"type":"torrents_uploaded","search":{"title_group_name":"Love","title_group_content_type":["music"],"title_group_category":["Single"],"title_group_include_empty_groups":false,"page":1,"page_size":1,"order_by_column":"torrent_created_at","order_by_direction":"desc"},"minimum_title_group_amount":2}'::jsonb,
    100
);

-- Same filter as 801 but threshold 1 — user 1 qualifies (TG 1 matches the filter)
INSERT INTO user_badges (id, name, description, image_url, category_id, badge_type, is_secret, revoke_when_criteria_unmet, criteria, created_by_id)
VALUES (
    802,
    'love_singles_one',
    '1+ music single upload with "Love" in the name',
    'https://example.com/love1.png',
    700,
    'torrents_uploaded',
    FALSE,
    TRUE,
    '{"type":"torrents_uploaded","search":{"title_group_name":"Love","title_group_content_type":["music"],"title_group_category":["Single"],"title_group_include_empty_groups":false,"page":1,"page_size":1,"order_by_column":"torrent_created_at","order_by_direction":"desc"},"minimum_title_group_amount":1}'::jsonb,
    100
);
