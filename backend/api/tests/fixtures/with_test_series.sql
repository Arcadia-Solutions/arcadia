INSERT INTO
    series (id, name, description, tags, covers, banners, created_by_id, created_at, updated_at)
VALUES
    (
        1,
        'Test Series',
        'A series used for testing',
        '{test,series}',
        '{https://example.com/cover.jpg}',
        '{https://example.com/banner.jpg}',
        1,
        NOW(),
        NOW()
    );

