-- Create a user class hierarchy with buyable promotion
INSERT INTO user_classes (name, new_permissions, previous_user_class, promotion_cost_bonus_points)
VALUES ('member', '{upload_torrent}', 'newbie', 5000);
