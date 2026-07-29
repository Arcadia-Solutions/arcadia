INSERT INTO title_group_tags (id, name, synonyms, created_by_id)
VALUES (10, 'science.fiction', '{"scifi","sci.fi"}', 100);

INSERT INTO title_group_tags (id, name, synonyms, created_by_id, deleted_at, deleted_by_id, deletion_reason)
VALUES (11, 'blu.ray', '{}', 100, NOW(), 100, 'not a genre');
