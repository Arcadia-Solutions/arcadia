CREATE TABLE notifications_collages (
    id BIGSERIAL PRIMARY KEY,
    collage_id BIGINT NOT NULL,
    title_group_id INT NOT NULL,
    user_id INT NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    read_status BOOLEAN NOT NULL DEFAULT FALSE,

    FOREIGN KEY (user_id) REFERENCES users(id),
    FOREIGN KEY (collage_id) REFERENCES collage(id) ON DELETE CASCADE,
    FOREIGN KEY (title_group_id) REFERENCES title_groups(id) ON DELETE CASCADE
);