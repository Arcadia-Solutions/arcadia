-- Staff PM created by user 100 (standard user), resolved
INSERT INTO staff_pms (id, subject, created_by_id, resolved)
VALUES (100, 'Resolved Staff PM', 100, TRUE);

-- Initial message for the resolved staff PM
INSERT INTO staff_pm_messages (id, staff_pm_id, created_by_id, content)
VALUES (100, 100, 100, 'This is a resolved staff PM');
