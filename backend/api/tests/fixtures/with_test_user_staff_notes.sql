-- Requires the "with_test_users" fixture.
-- Three notes left on the user 100, to test the view_foreign_user_staff_notes permission and
-- the 24 hours window during which the author of a note may still edit it.
INSERT INTO user_staff_notes (id, user_id, created_at, content, created_by_id)
VALUES (1000, 100, NOW(), 'Recent note of the note writer', 192);

INSERT INTO user_staff_notes (id, user_id, created_at, content, created_by_id)
VALUES (1001, 100, NOW() - INTERVAL '25 hours', 'Old note of the note writer', 192);

INSERT INTO user_staff_notes (id, user_id, created_at, content, created_by_id)
VALUES (1002, 100, NOW() - INTERVAL '1 hour', 'Note of another staff member', 194);
