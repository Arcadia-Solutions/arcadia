-- Two emojis sharing the same sort_order, to exercise the tiebreak in
-- find_forum_post_reaction_users's ORDER BY. sort_order is admin-supplied and has no
-- uniqueness constraint, so two emojis legitimately can land on the same value.
INSERT INTO
  emojis (id, name, unicode_character, sort_order)
VALUES
  (100, 'tied_a', '🅰', 1),
  (101, 'tied_b', '🅱', 1);

SELECT setval('emojis_id_seq', 200, false);
