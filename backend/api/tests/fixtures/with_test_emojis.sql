INSERT INTO
  emojis (id, name, unicode_character, sort_order)
VALUES
  (100, 'thumbs_up', '👍', 1),
  (101, 'thumbs_down', '👎', 2);

INSERT INTO
  emojis (id, name, image, image_mime_type, sort_order)
VALUES
  (102, 'custom_smile', '\x89504e470d0a1a0a', 'image/png', 3);

SELECT setval('emojis_id_seq', 200, false);
