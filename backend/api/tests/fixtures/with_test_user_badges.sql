-- Seed a category, a visible manual badge, a secret manual badge, and one earned-badge row
INSERT INTO user_badge_categories (id, name, created_by_id)
VALUES (500, 'achievements', 100);

INSERT INTO user_badges (id, name, description, image_url, category_id, badge_type, is_secret, revoke_when_criteria_unmet, criteria, created_by_id)
VALUES (
    600,
    'visible_manual',
    'visible manual badge',
    'https://example.com/visible.png',
    500,
    'manual',
    FALSE,
    FALSE,
    NULL,
    100
);

INSERT INTO user_badges (id, name, description, image_url, category_id, badge_type, is_secret, revoke_when_criteria_unmet, criteria, created_by_id)
VALUES (
    601,
    'secret_manual',
    'secret manual badge',
    'https://example.com/secret.png',
    500,
    'manual',
    TRUE,
    FALSE,
    NULL,
    100
);

INSERT INTO user_earned_badges (user_id, badge_id, awarded_by_id, note)
VALUES (100, 601, 100, 'staff award');
