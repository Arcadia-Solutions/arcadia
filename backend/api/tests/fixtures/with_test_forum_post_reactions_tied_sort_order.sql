-- Reactions on forum post 100 using the two tied-sort_order emojis from
-- with_test_emojis_tied_sort_order, with explicit, interleaved created_at values so that
-- naive ordering (without an emoji_id tiebreaker) would interleave the two emojis' rows
-- instead of keeping each emoji's reactors contiguous.
INSERT INTO
  forum_post_reactions (forum_post_id, user_id, emoji_id, created_at)
VALUES
  (100, 100, 100, '2025-01-01 10:00:00+00'), -- emoji 100, 1st reactor
  (100, 102, 101, '2025-01-01 10:01:00+00'), -- emoji 101, 1st reactor
  (100, 101, 100, '2025-01-01 10:02:00+00'), -- emoji 100, 2nd reactor
  (100, 103, 101, '2025-01-01 10:03:00+00'), -- emoji 101, 2nd reactor
  (100, 104, 101, '2025-01-01 10:04:00+00'); -- emoji 101, 3rd reactor
