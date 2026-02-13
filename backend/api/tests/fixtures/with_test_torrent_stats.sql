INSERT INTO title_groups (
    id, master_group_id, name, name_aliases, created_at, updated_at, created_by_id,
    description, platform, original_language, original_release_date,
    original_release_date_only_year_known, tagline, country_from,
    covers, external_links, trailers, category, content_type, public_ratings,
    series_id, screenshots
) VALUES
(
    10, NULL, 'Stats Movie A', '{}', '2025-01-01 00:00:00', '2025-01-01 00:00:00', 100,
    'A movie for stats', NULL, 'English', '2025-01-01',
    FALSE, NULL, 'US',
    '{}', '{}', '{}', 'FeatureFilm', 'movie', '[]'::JSONB, NULL, '{}'
),
(
    11, NULL, 'Stats Album B', '{}', '2025-01-01 00:00:00', '2025-01-01 00:00:00', 100,
    'An album for stats', NULL, 'French', '2025-01-01',
    FALSE, NULL, 'FR',
    '{}', '{}', '{}', 'Album', 'music', '[]'::JSONB, NULL, '{}'
);

INSERT INTO edition_groups (
    id, title_group_id, name, release_date, release_date_only_year_known,
    created_at, updated_at, created_by_id, description, distributor,
    covers, external_links, source, additional_information
) VALUES
(
    10, 10, 'Stats Edition A', '2025-01-01', FALSE,
    '2025-01-01 00:00:00', '2025-01-01 00:00:00', 100, '', '',
    '{}', '{}', 'Blu-Ray', '{}'
),
(
    11, 11, 'Stats Edition B', '2025-01-01', FALSE,
    '2025-01-01 00:00:00', '2025-01-01 00:00:00', 100, '', '',
    '{}', '{}', 'Web', '{}'
);

-- January 2025: 2 torrents (1080p, 720p) by two different users
INSERT INTO torrents (
    id, edition_group_id, created_at, updated_at, created_by_id,
    info_hash, info_dict, languages, release_name, release_group,
    description, file_amount_per_type, uploaded_as_anonymous, file_list,
    mediainfo, trumpable, staff_checked, container, size, duration,
    audio_codec, audio_bitrate, audio_bitrate_sampling, audio_channels,
    video_codec, features, subtitle_languages, video_resolution
) VALUES
(
    10, 10, '2025-01-15 10:00:00', '2025-01-15 10:00:00', 100,
    '\xaa00000000000000000000000000000000000000',
    '{}', '{}', 'Stats Torrent 1', '', '', '{}', FALSE, '{}',
    '', FALSE, FALSE, 'mkv', 5000000000, NULL,
    'aac', NULL, NULL, NULL,
    'h264', '{}', '{}', '1080p'
),
(
    11, 10, '2025-01-20 10:00:00', '2025-01-20 10:00:00', 101,
    '\xbb00000000000000000000000000000000000000',
    '{}', '{}', 'Stats Torrent 2', '', '', '{}', FALSE, '{}',
    '', FALSE, FALSE, 'mkv', 3000000000, NULL,
    'ac3', NULL, NULL, '5.1',
    'h264', '{}', '{}', '720p'
),
-- February 2025: 1 torrent (1080p) from edition B (music, Web source)
(
    12, 11, '2025-02-10 10:00:00', '2025-02-10 10:00:00', 100,
    '\xcc00000000000000000000000000000000000000',
    '{}', '{}', 'Stats Torrent 3', '', '', '{}', FALSE, '{}',
    '', FALSE, FALSE, 'flac', 800000000, NULL,
    'flac', NULL, 'Lossless', NULL,
    NULL, '{}', '{}', NULL
),
-- Deleted torrent in January
(
    13, 10, '2025-01-25 10:00:00', '2025-01-25 10:00:00', 100,
    '\xdd00000000000000000000000000000000000000',
    '{}', '{}', 'Stats Torrent Deleted', '', '', '{}', FALSE, '{}',
    '', FALSE, FALSE, 'mkv', 2000000000, NULL,
    NULL, NULL, NULL, NULL,
    NULL, '{}', '{}', '1080p'
);

-- Mark torrent 13 as deleted
UPDATE torrents SET deleted_at = '2025-01-26 10:00:00', deleted_by_id = 100 WHERE id = 13;
