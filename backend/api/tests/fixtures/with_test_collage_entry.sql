-- Collage entry in collage 1 (owned by user_basic)
INSERT INTO collage_entry (id, created_by_id, collage_id, title_group_id, note)
VALUES (1, 100, 1, 1, 'Test entry note');

-- Another collage entry in collage 1
INSERT INTO collage_entry (id, created_by_id, collage_id, title_group_id, note)
VALUES (2, 100, 1, 2, NULL);
