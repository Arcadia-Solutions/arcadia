-- Add forum delete permissions to user_permissions_enum
ALTER TYPE user_permissions_enum ADD VALUE IF NOT EXISTS 'delete_forum_category';
ALTER TYPE user_permissions_enum ADD VALUE IF NOT EXISTS 'delete_forum_sub_category';
ALTER TYPE user_permissions_enum ADD VALUE IF NOT EXISTS 'delete_forum_thread';
ALTER TYPE user_permissions_enum ADD VALUE IF NOT EXISTS 'delete_forum_post';
