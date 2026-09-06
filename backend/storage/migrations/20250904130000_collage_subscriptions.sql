CREATE TABLE subscriptions_collages (
    id BIGSERIAL PRIMARY KEY,
    collage_id BIGINT NOT NULL,
    user_id INT NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),

    FOREIGN KEY (collage_id) REFERENCES collage(id) ON DELETE CASCADE,
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE,

    UNIQUE (collage_id, user_id)
);