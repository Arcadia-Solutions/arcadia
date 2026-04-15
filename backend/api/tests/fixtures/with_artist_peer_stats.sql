-- Torrent 1 is affiliated (via edition_group 1 -> title_group 1 -> artist 1).
-- Torrent 2 is on edition_group 2 / title_group 2 and has no affiliated artist.
UPDATE torrents SET seeders = 5, leechers = 2, times_completed = 7 WHERE id = 1;
UPDATE torrents SET seeders = 11, leechers = 3, times_completed = 4 WHERE id = 2;

-- An extra torrent on the same edition_group as torrent 1, marked as deleted
-- to verify deleted torrents are excluded from the aggregation.
INSERT INTO torrents (
    id, edition_group_id, created_at, updated_at, created_by_id,
    info_hash, info_dict, languages, release_name, release_group,
    description, file_amount_per_type, uploaded_as_anonymous, file_list,
    mediainfo, trumpable, staff_checked, container, size, duration,
    audio_codec, audio_bitrate, audio_bitrate_sampling, audio_channels,
    video_codec, features, subtitle_languages, video_resolution,
    seeders, leechers, times_completed, deleted_at, deleted_by_id
) VALUES (
    100, 1, NOW(), NOW(), 1,
    '\xdeadbeef00000000000000000000000000000001',
    '{}', '{}', 'Deleted Affiliated Torrent', '', '', '{}', FALSE, '{}',
    '', FALSE, FALSE, 'flac', 1000, NULL,
    'flac', NULL, 'Lossless', NULL,
    NULL, '{}', '{}', NULL,
    99, 99, 99, NOW(), 1
);
