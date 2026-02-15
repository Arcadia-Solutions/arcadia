-- Restricted sub-category where only allowed posters can create threads
INSERT INTO forum_sub_categories (id, forum_category_id, name, created_by_id, new_threads_restricted)
VALUES (102, 100, 'Restricted Sub Category', 100, TRUE);

-- Allow user_basic (id=100) to post in the restricted sub-category
INSERT INTO forum_sub_category_allowed_posters (forum_sub_category_id, user_id)
VALUES (102, 100);
