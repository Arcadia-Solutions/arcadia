INSERT INTO user_edit_change_logs (id, item_type, item_id, edited_by_id, edited_at, edits)
VALUES
    (1, 'artist', 1, 100, '2025-06-01 10:00:00+00', '{"name": {"old": "Old Name", "new": "New Name"}}'),
    (2, 'title_group', 5, 100, '2025-06-02 11:00:00+00', '{"title": {"old": "Old Title", "new": "New Title"}}'),
    (3, 'artist', 2, 100, '2025-06-03 12:00:00+00', '{"description": {"old": "Old Desc", "new": "New Desc"}}');
