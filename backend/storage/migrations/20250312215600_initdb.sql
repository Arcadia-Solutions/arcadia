CREATE TYPE user_class_enum AS ENUM (
    'newbie',
    'staff',
    'tracker'
);

CREATE TABLE users (
    id BIGSERIAL PRIMARY KEY,
    username VARCHAR(20) UNIQUE NOT NULL,
    avatar TEXT,
    email VARCHAR(255) UNIQUE NOT NULL,
    password_hash VARCHAR(255) NOT NULL,
    registered_from_ip INET NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    description TEXT NOT NULL DEFAULT '',
    uploaded BIGINT NOT NULL DEFAULT 0,
    real_uploaded BIGINT NOT NULL DEFAULT 0,
    -- 1 byte downloaded
    downloaded BIGINT NOT NULL DEFAULT 1,
    real_downloaded BIGINT NOT NULL DEFAULT 1,
    ratio FLOAT NOT NULL DEFAULT 0.0,
    required_ratio FLOAT NOT NULL DEFAULT 0.0,
    last_seen TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    class user_class_enum NOT NULL DEFAULT 'newbie',
    forum_posts INTEGER NOT NULL DEFAULT 0,
    forum_threads INTEGER NOT NULL DEFAULT 0,
    torrent_comments INTEGER NOT NULL DEFAULT 0,
    request_comments INTEGER NOT NULL DEFAULT 0,
    artist_comments BIGINT NOT NULL DEFAULT 0,
    seeding INTEGER NOT NULL DEFAULT 0,
    leeching INTEGER NOT NULL DEFAULT 0,
    snatched INTEGER NOT NULL DEFAULT 0,
    seeding_size BIGINT NOT NULL DEFAULT 0,
    requests_filled BIGINT NOT NULL DEFAULT 0,
    collages_started BIGINT NOT NULL DEFAULT 0,
    requests_voted BIGINT NOT NULL DEFAULT 0,
    average_seeding_time BIGINT NOT NULL DEFAULT 0,
    invited BIGINT NOT NULL DEFAULT 0,
    invitations SMALLINT NOT NULL DEFAULT 0,
    bonus_points BIGINT NOT NULL DEFAULT 0,
    freeleech_tokens INT NOT NULL DEFAULT 0,
    settings JSONB NOT NULL DEFAULT '{}',
    passkey_upper BIGINT NOT NULL,
    passkey_lower BIGINT NOT NULL,
    warned BOOLEAN NOT NULL DEFAULT FALSE,
    banned BOOLEAN NOT NULL DEFAULT FALSE,
    staff_note TEXT NOT NULL DEFAULT '',

    UNIQUE(passkey_upper, passkey_lower)
);
INSERT INTO users (username, email, password_hash, registered_from_ip, settings, passkey_upper, passkey_lower)
VALUES ('creator', 'none@domain.com', 'none', '127.0.0.1', '{}'::jsonb, '1', '1');
CREATE TABLE api_keys (
    id BIGSERIAL PRIMARY KEY,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    name VARCHAR(30) NOT NULL,
    value VARCHAR(40) NOT NULL UNIQUE,
    user_id BIGINT NOT NULL REFERENCES users(id) ON DELETE CASCADE
);
CREATE TYPE user_application_status_enum AS ENUM (
    'pending',
    'accepted',
    'rejected'
);
CREATE TABLE user_applications (
    id BIGSERIAL PRIMARY KEY,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    body TEXT NOT NULL,
    referral TEXT NOT NULL,
    email TEXT NOT NULL,
    staff_note TEXT NOT NULL DEFAULT '',
    status user_application_status_enum NOT NULL DEFAULT 'pending'
);
CREATE TABLE invitations (
    id BIGSERIAL PRIMARY KEY,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    expires_at TIMESTAMP WITH TIME ZONE NOT NULL,
    invitation_key VARCHAR(50) NOT NULL,
    message TEXT NOT NULL,
    sender_id BIGINT NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    receiver_email VARCHAR(255) NOT NULL,
    user_application_id BIGINT REFERENCES user_applications(id) ON DELETE SET NULL,
    receiver_id BIGINT REFERENCES users(id) ON DELETE SET NULL
);

CREATE TABLE user_warnings (
    id BIGSERIAL PRIMARY KEY,
    user_id BIGINT NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    expires_at TIMESTAMP WITH TIME ZONE,
    reason TEXT NOT NULL,
    ban boolean NOT NULL,
    created_by_id BIGINT NOT NULL REFERENCES users(id) ON DELETE CASCADE
);
CREATE TABLE gifts (
    id BIGSERIAL PRIMARY KEY,
    sent_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    message TEXT NOT NULL,
    sender_id BIGINT NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    receiver_id BIGINT NOT NULL REFERENCES users(id) ON DELETE SET NULL,
    bonus_points BIGINT NOT NULL DEFAULT 0,
    freeleech_tokens INT NOT NULL DEFAULT 0
);
CREATE TABLE artists (
    id BIGSERIAL PRIMARY KEY,
    name VARCHAR(255) UNIQUE NOT NULL,
    description TEXT NOT NULL,
    pictures TEXT [] NOT NULL,
    created_by_id BIGINT NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    title_groups_amount INT NOT NULL DEFAULT 0,
    edition_groups_amount INT NOT NULL DEFAULT 0,
    torrents_amount INT NOT NULL DEFAULT 0,
    seeders_amount INT NOT NULL DEFAULT 0,
    leechers_amount INT NOT NULL DEFAULT 0,
    snatches_amount INT NOT NULL DEFAULT 0,
    FOREIGN KEY (created_by_id) REFERENCES users(id) ON DELETE CASCADE
);
CREATE TABLE similar_artists (
    artist_1_id BIGINT NOT NULL,
    artist_2_id BIGINT NOT NULL,
    PRIMARY KEY (artist_1_id, artist_2_id),
    FOREIGN KEY (artist_1_id) REFERENCES artists(id) ON DELETE CASCADE,
    FOREIGN KEY (artist_2_id) REFERENCES artists(id) ON DELETE CASCADE
);
CREATE TABLE master_groups (
    id BIGSERIAL PRIMARY KEY,
    name VARCHAR(255),
    -- name_aliases VARCHAR(255)[],
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    created_by_id BIGINT NOT NULL,
    -- description TEXT NOT NULL,
    -- original_language VARCHAR(50) NOT NULL,
    -- country_from VARCHAR(50) NOT NULL,
    -- tags VARCHAR(50)[] NOT NULL,
    -- category VARCHAR(25), -- should only be used for TV-Shows (scripted, reality-tv, etc.)
    -- covers TEXT[],
    -- banners TEXT[],
    -- fan_arts TEXT[],
    FOREIGN KEY (created_by_id) REFERENCES users(id) ON DELETE
    SET NULL
);
CREATE TABLE similar_master_groups (
    group_1_id BIGINT NOT NULL,
    group_2_id BIGINT NOT NULL,
    PRIMARY KEY (group_1_id, group_2_id),
    FOREIGN KEY (group_1_id) REFERENCES master_groups(id) ON DELETE CASCADE,
    FOREIGN KEY (group_2_id) REFERENCES master_groups(id) ON DELETE CASCADE
);
CREATE TABLE series (
    id BIGSERIAL PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    description TEXT NOT NULL,
    tags TEXT [] NOT NULL,
    covers TEXT [] NOT NULL,
    banners TEXT [] NOT NULL,
    created_by_id BIGINT NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    FOREIGN KEY (created_by_id) REFERENCES users(id) ON DELETE CASCADE
);
CREATE TYPE content_type_enum AS ENUM (
    'movie',
    'video',
    'tv_show',
    'music',
    'podcast',
    'software',
    'book',
    'collection'
);
CREATE TYPE title_group_category_enum AS ENUM (
    'Ep',
    'Album',
    'Single',
    'Soundtrack',
    'Anthology',
    'Compilation',
    'Remix',
    'Bootleg',
    'Mixtape',
    'ConcertRecording',
    'DjMix',
    'FeatureFilm',
    'ShortFilm',
    'Game',
    'Program',
    'Illustrated',
    'Periodical',
    'Book',
    'Article',
    'Manual',
    'Other'
);
CREATE TYPE platform_enum AS ENUM(
    'Linux',
    'MacOS',
    'Windows',
    'Xbox'
);
CREATE TYPE language_enum AS ENUM(
   'Albanian',
   'Arabic',
   'Belarusian',
   'Bengali',
   'Bosnian',
   'Bulgarian',
   'Cantonese',
   'Catalan',
   'Chinese',
   'Croatian',
   'Czech',
   'Danish',
   'Dutch',
   'English',
   'Estonian',
   'Finnish',
   'French',
   'German',
   'Greek',
   'Hebrew',
   'Hindi',
   'Hungarian',
   'Icelandic',
   'Indonesian',
   'Italian',
   'Japanese',
   'Kannada',
   'Korean',
   'Macedonian',
   'Malayalam',
   'Mandarin',
   'Nepali',
   'Norwegian',
   'Persian',
   'Polish',
   'Portuguese',
   'Romanian',
   'Russian',
   'Serbian',
   'Spanish',
   'Swedish',
   'Tamil',
   'Tagalog',
   'Telugu',
   'Thai',
   'Turkish',
   'Ukrainian',
   'Vietnamese',
   'Wolof',
   'Other'
);
CREATE TABLE title_groups (
    id BIGSERIAL PRIMARY KEY,
    master_group_id BIGINT,
    name TEXT NOT NULL,
    name_aliases TEXT [],
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    created_by_id BIGINT NOT NULL,
    description TEXT NOT NULL,
    platform platform_enum,
    original_language language_enum,
    original_release_date TIMESTAMP WITH TIME ZONE NOT NULL,
    tagline TEXT,
    tags VARCHAR(50) [] NOT NULL,
    country_from TEXT,
    covers TEXT [] NOT NULL,
    external_links TEXT [] NOT NULL,
    embedded_links JSONB NOT NULL,
    category title_group_category_enum,
    content_type content_type_enum NOT NULL,
    public_ratings JSONB NOT NULL,
    screenshots TEXT[] NOT NULL,
    series_id BIGINT,
    FOREIGN KEY (master_group_id) REFERENCES master_groups(id) ON DELETE
    SET NULL,
        FOREIGN KEY (created_by_id) REFERENCES users(id) ON DELETE
    SET NULL,
        FOREIGN KEY (series_id) REFERENCES series(id) ON DELETE
    SET NULL
);
CREATE TABLE similar_title_groups (
    group_1_id BIGINT NOT NULL,
    group_2_id BIGINT NOT NULL,
    PRIMARY KEY (group_1_id, group_2_id),
    FOREIGN KEY (group_1_id) REFERENCES title_groups(id) ON DELETE CASCADE,
    FOREIGN KEY (group_2_id) REFERENCES title_groups(id) ON DELETE CASCADE
);
CREATE TYPE artist_role_enum AS ENUM (
    'main',
    'guest',
    'producer',
    'director',
    'cinematographer',
    'actor',
    'writer',
    'composer',
    'remixer',
    'conductor',
    'dj_compiler',
    'arranger',
    'host',
    'author',
    'illustrator',
    'editor',
    'developer',
    'designer'
);
CREATE TABLE affiliated_artists (
    id BIGSERIAL PRIMARY KEY,
    title_group_id BIGINT NOT NULL,
    artist_id BIGINT NOT NULL,
    roles artist_role_enum[] NOT NULL,
    nickname VARCHAR(255),
    created_by_id BIGINT NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    FOREIGN KEY (title_group_id) REFERENCES title_groups(id) ON DELETE CASCADE,
    FOREIGN KEY (artist_id) REFERENCES artists(id) ON DELETE CASCADE,
    FOREIGN KEY (created_by_id) REFERENCES users(id) ON DELETE
    SET NULL
);
-- for web: if it is a DL or a RIP should be specified at the torrent level
CREATE TYPE source_enum AS ENUM (
    'CD',
    'DVD5',
    'DVD9',
    'Vinyl',
    'Web',
    'Soundboard',
    'SACD',
    'DAT',
    'Cassette',
    'Blu-Ray',
    'LaserDisc',
    'HD-DVD',
    'HDTV',
    'PDTV',
    'TV',
    'VHS',
    'Mixed',
    'Physical Book'
);
CREATE TABLE edition_groups (
    id BIGSERIAL PRIMARY KEY,
    title_group_id BIGINT NOT NULL,
    name TEXT,
    release_date TIMESTAMP WITH TIME ZONE NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    created_by_id BIGINT NOT NULL,
    description TEXT,
    distributor VARCHAR(255),
    covers TEXT [] NOT NULL,
    external_links TEXT [] NOT NULL,
    source source_enum,
    additional_information JSONB,
    FOREIGN KEY (title_group_id) REFERENCES title_groups(id) ON DELETE CASCADE,
    FOREIGN KEY (created_by_id) REFERENCES users(id) ON DELETE
    SET NULL
);
CREATE TYPE audio_codec_enum AS ENUM (
    'mp2',
    'mp3',
    'aac',
    'ac3',
    'dts',
    'flac',
    'pcm',
    'true-hd',
    'opus',
    'dsd'
);

CREATE TYPE video_resolution_enum AS ENUM (
    'Other',
    '480p',
    '480i',
    '576p',
    '576i',
    '720p',
    '1080p',
    '1080i',
    '1440p',
    '2160p',
    '4320p'
);

CREATE TYPE audio_bitrate_sampling_enum AS ENUM(
    '64',
    '128',
    '192',
    '256',
    '320',
    'APS (VBR)',
    'V2 (VBR)',
    'V1 (VBR)',
    'APX (VBR)',
    'V0 (VBR)',
    'Lossless',
    '24bit Lossless',
    'DSD64',
    'DSD128',
    'DSD256',
    'DSD512',
    'other'
);
CREATE TYPE audio_channels_enum AS ENUM (
    '1.0',
    '2.0',
    '2.1',
    '5.0',
    '5.1',
    '7.1'
);
CREATE TYPE video_codec_enum AS ENUM(
    'mpeg1',
    'mpeg2',
    'Xvid',
    'divX',
    'h264',
    'h265',
    'vc-1',
    'vp9',
    'BD50',
    'UHD100'
);
CREATE TYPE features_enum AS ENUM('HDR', 'HDR 10', 'HDR 10+', 'DV', 'Commentary', 'Remux', '3D', 'Cue', 'OCR');
CREATE TYPE extras_enum AS ENUM('booklet', 'manual', 'behind_the_scenes', 'deleted_scenes', 'featurette', 'trailer', 'other');
CREATE TABLE torrents (
    id BIGSERIAL PRIMARY KEY,
    upload_factor FLOAT NOT NULL DEFAULT 1.0,
    download_factor FLOAT NOT NULL DEFAULT 1.0,
    seeders BIGINT NOT NULL DEFAULT 0,
    leechers BIGINT NOT NULL DEFAULT 0,
    completed BIGINT NOT NULL DEFAULT 0,
    snatched BIGINT NOT NULL DEFAULT 0,
    edition_group_id BIGINT NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    created_by_id BIGINT NOT NULL,
    info_hash BYTEA NOT NULL CHECK(octet_length(info_hash) = 20),
    info_dict BYTEA NOT NULL,
    languages language_enum[] NOT NULL,
    release_name TEXT NOT NULL,
    -- maybe change the size
    release_group VARCHAR(30),
    description TEXT,
    file_amount_per_type JSONB NOT NULL,
    uploaded_as_anonymous BOOLEAN NOT NULL DEFAULT FALSE,
    file_list JSONB NOT NULL,
    -- maybe change the size to the max length of a file name in a torrent
    mediainfo TEXT,
    trumpable TEXT,
    staff_checked BOOLEAN NOT NULL DEFAULT FALSE,
    container VARCHAR(8) NOT NULL,
    -- in bytes
    size BIGINT NOT NULL,

    -- audio
    duration INT,
    -- in seconds
    audio_codec audio_codec_enum,
    audio_bitrate INT,
    -- in kb/s, taken from mediainfo
    audio_bitrate_sampling audio_bitrate_sampling_enum,
    audio_channels audio_channels_enum,
    -- audio

    -- video
    video_codec video_codec_enum,
    features features_enum [] NOT NULL,
    subtitle_languages language_enum[] NOT NULL,
    video_resolution video_resolution_enum,
    video_resolution_other_x INT,
    video_resolution_other_y INT,

    extras extras_enum[] DEFAULT ARRAY[]::extras_enum[],

    FOREIGN KEY (edition_group_id) REFERENCES edition_groups(id) ON DELETE CASCADE,
    FOREIGN KEY (created_by_id) REFERENCES users(id) ON DELETE SET NULL,
    UNIQUE (info_hash)
);
CREATE TABLE deleted_torrents (
    LIKE torrents INCLUDING CONSTRAINTS, -- INCLUDING DEFAULTS INCLUDING INDEXES,
    deleted_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    deleted_by_id BIGINT NOT NULL,
    reason TEXT NOT NULL,

    FOREIGN KEY (deleted_by_id) REFERENCES users(id)
);
CREATE TABLE title_group_comments (
    id BIGSERIAL PRIMARY KEY,
    content TEXT NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    created_by_id BIGINT NOT NULL,
    title_group_id BIGINT NOT NULL,
    refers_to_torrent_id BIGINT,
    answers_to_comment_id BIGINT,
    FOREIGN KEY (created_by_id) REFERENCES users(id) ON DELETE CASCADE,
    FOREIGN KEY (title_group_id) REFERENCES title_groups(id) ON DELETE CASCADE,
    FOREIGN KEY (refers_to_torrent_id) REFERENCES torrents(id) ON DELETE SET NULL,
    FOREIGN KEY (answers_to_comment_id) REFERENCES title_group_comments(id) ON DELETE SET NULL
);
CREATE TABLE torrent_requests (
    id BIGSERIAL PRIMARY KEY,
    title_group_id BIGINT NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    created_by_id BIGINT NOT NULL,
    filled_by_user_id BIGINT,
    filled_by_torrent_id BIGINT,
    filled_at TIMESTAMP WITH TIME ZONE,
    edition_name TEXT,
    source source_enum[] NOT NULL DEFAULT ARRAY[]::source_enum[],
    release_group VARCHAR(20),
    description TEXT,
    languages language_enum[] NOT NULL DEFAULT ARRAY[]::language_enum[],
    container VARCHAR(8)[] NOT NULL DEFAULT ARRAY[]::VARCHAR(8)[],
    -- Audio
    audio_codec audio_codec_enum[] NOT NULL DEFAULT ARRAY[]::audio_codec_enum[],
    audio_channels audio_channels_enum[] NOT NULL DEFAULT ARRAY[]::audio_channels_enum[],
    audio_bitrate_sampling audio_bitrate_sampling_enum[] NOT NULL DEFAULT ARRAY[]::audio_bitrate_sampling_enum[],
    -- Video
    video_codec video_codec_enum[] NOT NULL DEFAULT ARRAY[]::video_codec_enum[],
    features features_enum[] NOT NULL DEFAULT ARRAY[]::features_enum[],
    subtitle_languages language_enum[] NOT NULL DEFAULT ARRAY[]::language_enum[],
    video_resolution video_resolution_enum[] NOT NULL DEFAULT ARRAY[]::video_resolution_enum[],
    video_resolution_other_x INT,
    video_resolution_other_y INT,
    FOREIGN KEY (title_group_id) REFERENCES title_groups(id) ON DELETE CASCADE,
    FOREIGN KEY (created_by_id) REFERENCES users(id),
    FOREIGN KEY (filled_by_user_id) REFERENCES users(id),
    FOREIGN KEY (filled_by_torrent_id) REFERENCES torrents(id)
);
CREATE TABLE torrent_request_votes(
    id BIGSERIAL PRIMARY KEY,
    torrent_request_id BIGINT NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    created_by_id BIGINT NOT NULL,
    bounty_upload BIGINT NOT NULL DEFAULT 0,
    bounty_bonus_points BIGINT NOT NULL DEFAULT 0,
    FOREIGN KEY (torrent_request_id) REFERENCES torrent_requests(id) ON DELETE CASCADE,
    FOREIGN KEY (created_by_id) REFERENCES users(id) ON DELETE CASCADE
);
CREATE TABLE torrent_reports (
    id BIGSERIAL PRIMARY KEY,
    reported_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    reported_by_id BIGINT NOT NULL,
    description TEXT NOT NULL,
    reported_torrent_id BIGINT NOT NULL,
    FOREIGN KEY (reported_by_id) REFERENCES users(id) ON DELETE CASCADE,
    FOREIGN KEY (reported_torrent_id) REFERENCES torrents(id) ON DELETE CASCADE
);

CREATE TYPE peer_status_enum AS ENUM('seeding', 'leeching');
CREATE TABLE peers (
    id BIGINT GENERATED ALWAYS AS IDENTITY,
    user_id BIGINT NOT NULL,
    torrent_id BIGINT NOT NULL,
    peer_id BYTEA NOT NULL CHECK(octet_length(peer_id) = 20),
    ip INET NOT NULL,
    port INTEGER NOT NULL,
    first_seen_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP ,
    last_seen_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    real_uploaded BIGINT NOT NULL DEFAULT 0,
    real_downloaded BIGINT NOT NULL DEFAULT 0,
    user_agent TEXT,
    status peer_status_enum NOT NULL,

    PRIMARY KEY (id),

    FOREIGN KEY (torrent_id) REFERENCES torrents(id) ON DELETE CASCADE,
    FOREIGN KEY (user_id) REFERENCES users(id),

    UNIQUE (torrent_id, peer_id, ip, port)
);
CREATE TABLE torrent_activities (
    id BIGSERIAL PRIMARY KEY,
    torrent_id BIGINT NOT NULL,
    user_id BIGINT NOT NULL,
    snatched_at TIMESTAMP WITH TIME ZONE,
    first_seen_seeding_at TIMESTAMP WITH TIME ZONE,
    last_seen_seeding_at TIMESTAMP WITH TIME ZONE,
    total_seed_time BIGINT NOT NULL DEFAULT 0,

    FOREIGN KEY (torrent_id) REFERENCES torrents(id) ON DELETE CASCADE,
    FOREIGN KEY (user_id) REFERENCES users(id),

    UNIQUE (torrent_id, user_id)
);
CREATE TABLE entities (
    id BIGSERIAL PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    description TEXT NOT NULL,
    pictures TEXT[] NOT NULL,
    created_by_id BIGINT NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    title_groups_amount INT NOT NULL DEFAULT 0,
    edition_groups_amount INT NOT NULL DEFAULT 0,
    torrents_amount INT NOT NULL DEFAULT 0,
    seeders_amount INT NOT NULL DEFAULT 0,
    leechers_amount INT NOT NULL DEFAULT 0,
    snatches_amount INT NOT NULL DEFAULT 0,
    FOREIGN KEY (created_by_id) REFERENCES users(id)
);
CREATE TYPE entity_role_enum AS ENUM (
    'producer',
    'developer',
    'designer',
    'label'
);
CREATE TABLE affiliated_entities (
    id BIGSERIAL PRIMARY KEY,
    title_group_id BIGINT NOT NULL,
    entity_id BIGINT NOT NULL,
    created_by_id BIGINT NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    roles entity_role_enum[] NOT NULL,
    FOREIGN KEY (title_group_id) REFERENCES title_groups(id) ON DELETE CASCADE,
    FOREIGN KEY (entity_id) REFERENCES entities(id) ON DELETE CASCADE,
    FOREIGN KEY (created_by_id) REFERENCES users(id) ON DELETE SET NULL
);
CREATE TYPE collage_category_enum AS ENUM (
    'Personal',
    'Staff Picks',
    'External',
    'Theme'
);
CREATE TYPE collage_type_enum AS ENUM (
    'Artist',
    'Entity',
    'Title'
);
CREATE TABLE collage (
    id BIGSERIAL PRIMARY KEY,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    created_by_id BIGINT NOT NULL,
    name VARCHAR NOT NULL,
    covers VARCHAR NOT NULL,
    description TEXT NOT NULL,
    tags VARCHAR[] NOT NULL,
    category collage_category_enum NOT NULL,
    section collage_type_enum NOT NULL,
    FOREIGN KEY (created_by_id) REFERENCES users(id)
);
CREATE TABLE collage_title_group_entry (
    id BIGSERIAL PRIMARY KEY,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    created_by_id BIGINT NOT NULL,
    title_group_id BIGINT NOT NULL,
    collage_id BIGINT NOT NULL,
    FOREIGN KEY (collage_id) REFERENCES users(id),
    FOREIGN KEY (title_group_id) REFERENCES title_groups(id),
    FOREIGN KEY (created_by_id) REFERENCES users(id)
);
CREATE TABLE collage_artist_entry (
    id BIGSERIAL PRIMARY KEY,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    created_by_id BIGINT NOT NULL,
    artist_id BIGINT NOT NULL,
    collage_id BIGINT NOT NULL,
    FOREIGN KEY (artist_id) REFERENCES artists(id),
    FOREIGN KEY (collage_id) REFERENCES users(id),
    FOREIGN KEY (created_by_id) REFERENCES users(id)
);
CREATE TABLE collage_entity_entry (
    id BIGSERIAL PRIMARY KEY,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    created_by_id BIGINT NOT NULL,
    entity_id BIGINT NOT NULL,
    collage_id BIGINT NOT NULL,
    FOREIGN KEY (entity_id) REFERENCES entities(id),
    FOREIGN KEY (collage_id) REFERENCES users(id),
    FOREIGN KEY (created_by_id) REFERENCES users(id)
);
CREATE TABLE forum_categories (
    id SERIAL PRIMARY KEY,
    name TEXT NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    created_by_id BIGINT NOT NULL,

    FOREIGN KEY (created_by_id) REFERENCES users(id)
);
INSERT INTO forum_categories (created_by_id, name) VALUES (1, 'Site');
CREATE TABLE forum_sub_categories (
    id SERIAL PRIMARY KEY NOT NULL,
    forum_category_id INT NOT NULL,
    name TEXT NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    created_by_id BIGINT,
    threads_amount BIGINT NOT NULL DEFAULT 0,
    posts_amount BIGINT NOT NULL DEFAULT 0,
    forbidden_classes VARCHAR(50) [] NOT NULL DEFAULT ARRAY[]::VARCHAR(50)[],

    FOREIGN KEY (created_by_id) REFERENCES users(id),
    FOREIGN KEY (forum_category_id) REFERENCES forum_categories(id)
);
INSERT INTO forum_sub_categories (created_by_id, forum_category_id,name, threads_amount, posts_amount) VALUES (1, 1, 'Announcements', 1, 1);
CREATE TABLE forum_threads (
    id BIGSERIAL PRIMARY KEY,
    forum_sub_category_id INT NOT NULL,
    name TEXT NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    created_by_id BIGINT NOT NULL,
    posts_amount BIGINT NOT NULL DEFAULT 0,
    sticky BOOLEAN NOT NULL DEFAULT FALSE,
    locked BOOLEAN NOT NULL DEFAULT FALSE,

    FOREIGN KEY (created_by_id) REFERENCES users(id),
    FOREIGN KEY (forum_sub_category_id) REFERENCES forum_sub_categories(id)
);
INSERT INTO forum_threads (created_by_id, forum_sub_category_id, name, posts_amount) VALUES (1, 1, 'Welcome to the site!', 1);
CREATE TABLE forum_posts (
    id BIGSERIAL PRIMARY KEY,
    forum_thread_id BIGINT NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    created_by_id BIGINT NOT NULL,
    content TEXT NOT NULL,
    sticky BOOLEAN NOT NULL DEFAULT FALSE,

    FOREIGN KEY (created_by_id) REFERENCES users(id),
    FOREIGN KEY (forum_thread_id) REFERENCES forum_threads(id)
);
INSERT INTO forum_posts (created_by_id, forum_thread_id, content) VALUES (1, 1, 'Welcome!');
CREATE TABLE wiki_articles (
    id BIGSERIAL PRIMARY KEY,
    title TEXT NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    created_by_id BIGINT NOT NULL,
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    updated_by_id BIGINT NOT NULL,
    body TEXT NOT NULL,

    FOREIGN KEY (created_by_id) REFERENCES users(id)
);
CREATE TABLE conversations (
    id BIGSERIAL PRIMARY KEY,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW() NOT NULL,
    subject VARCHAR(255) NOT NULL,
    sender_id BIGINT NOT NULL,
    receiver_id BIGINT NOT NULL,
    sender_last_seen_at TIMESTAMP WITH TIME ZONE DEFAULT NOW() NOT NULL,
    receiver_last_seen_at TIMESTAMP WITH TIME ZONE,

    FOREIGN KEY (sender_id) REFERENCES users(id),
    FOREIGN KEY (receiver_id) REFERENCES users(id)
);
CREATE TABLE conversation_messages (
    id BIGSERIAL PRIMARY KEY,
    conversation_id BIGINT NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW() NOT NULL,
    created_by_id BIGINT NOT NULL,
    content TEXT NOT NULL,

    FOREIGN KEY (conversation_id) REFERENCES conversations(id),
    FOREIGN KEY (created_by_id) REFERENCES users(id)
);
CREATE TYPE notification_reason_enum AS ENUM (
    'TorrentUploadedInSubscribedTitleGroup',
    'SeedingTorrentDeleted',
    'TitleGroupAddedForSubscribedArtist',
    'ThreadAddedInSubscribedForumSubCategory',
    'TitleGroupAddedInSubscribedCollage'
);
CREATE TABLE subscriptions (
    id BIGSERIAL PRIMARY KEY,
    title_group_id BIGINT,
    artist_id BIGINT,
    forum_thread_id BIGINT,
    forum_sub_category_id BIGINT,
    subscribed_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    subscriber_id BIGINT NOT NULL,
    FOREIGN KEY (title_group_id) REFERENCES title_groups(id) ON DELETE CASCADE,
    FOREIGN KEY (artist_id) REFERENCES artists(id) ON DELETE CASCADE,
    FOREIGN KEY (forum_thread_id) REFERENCES forum_threads(id) ON DELETE CASCADE,
    FOREIGN KEY (forum_sub_category_id) REFERENCES forum_sub_categories(id) ON DELETE CASCADE,
    FOREIGN KEY (subscriber_id) REFERENCES users(id) ON DELETE SET NULL,
    UNIQUE (title_group_id, subscriber_id)
);
CREATE TABLE notifications (
    id BIGSERIAL PRIMARY KEY,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    receiver_id BIGINT NOT NULL,
    reason notification_reason_enum NOT NULL,
    message TEXT,
    read_status BOOLEAN NOT NULL DEFAULT FALSE,
    title_group_id BIGINT,
    torrent_id BIGINT,
    artist_id BIGINT,
    -- collage_id BIGINT,
    forum_thread_id BIGINT,
    FOREIGN KEY (receiver_id) REFERENCES users(id),
    FOREIGN KEY (title_group_id) REFERENCES title_groups(id) ON DELETE CASCADE,
    FOREIGN KEY (torrent_id) REFERENCES torrents(id) ON DELETE CASCADE,
    FOREIGN KEY (artist_id) REFERENCES artists(id) ON DELETE CASCADE,
    -- FOREIGN KEY (collage_id) REFERENCES collages(id) ON DELETE CASCADE,
    FOREIGN KEY (forum_thread_id) REFERENCES forum_threads(id) ON DELETE CASCADE
);

-- Views

CREATE VIEW torrents_and_reports AS
SELECT
    t.id,
    t.upload_factor,
    t.download_factor,
    t.seeders,
    t.leechers,
    t.completed,
    t.edition_group_id,
    t.created_at,
    t.updated_at,
    -- Always keep the actual created_by_id for internal queries
    t.created_by_id,
    -- Add display fields that respect anonymity
    CASE
        WHEN t.uploaded_as_anonymous THEN NULL
        ELSE t.created_by_id
    END as display_created_by_id,
    CASE
        WHEN t.uploaded_as_anonymous THEN NULL
        ELSE json_build_object('id', u.id, 'username', u.username)
    END AS display_created_by,
    t.info_hash,
    t.extras,
    t.languages,
    t.release_name,
    t.release_group,
    t.description,
    t.file_amount_per_type,
    t.uploaded_as_anonymous,
    t.file_list,
    t.mediainfo,
    t.trumpable,
    t.staff_checked,
    t.container,
    t.size,
    t.duration,
    t.audio_codec,
    t.audio_bitrate,
    t.audio_bitrate_sampling,
    t.audio_channels,
    t.video_codec,
    t.features,
    t.subtitle_languages,
    t.video_resolution,
    t.video_resolution_other_x,
    t.video_resolution_other_y,
    CASE
        WHEN EXISTS (SELECT 1 FROM torrent_reports WHERE reported_torrent_id = t.id) THEN json_agg(row_to_json(tr))
        ELSE '[]'::json
    END AS reports
FROM
    torrents t
LEFT JOIN
    users u ON t.created_by_id = u.id
LEFT JOIN
    torrent_reports tr ON t.id = tr.reported_torrent_id
LEFT JOIN
    peers p ON t.id = p.torrent_id
GROUP BY
    t.id, u.id, u.username
ORDER BY
    t.id;

    CREATE FUNCTION get_title_groups_and_edition_group_and_torrents_lite(
        p_title_group_name TEXT DEFAULT NULL,
        p_torrent_staff_checked BOOLEAN DEFAULT NULL,
        p_torrent_reported BOOLEAN DEFAULT NULL,
        p_include_empty_groups BOOLEAN DEFAULT TRUE,
        p_sort_by TEXT DEFAULT 'title_group_original_release_date',
        p_order TEXT DEFAULT 'desc',
        p_limit BIGINT DEFAULT NULL,
        p_offset BIGINT DEFAULT NULL,
        p_torrent_created_by_id BIGINT DEFAULT NULL,
        p_torrent_snatched_by_id BIGINT DEFAULT NULL,
        p_requesting_user_id BIGINT DEFAULT NULL,
        p_external_link TEXT DEFAULT NULL
    )
    RETURNS TABLE (
        title_group_id BIGINT,
        title_group_data JSONB
    )
    LANGUAGE plpgsql
    AS $$
    BEGIN
        RETURN QUERY
        WITH snatched_torrents AS (
            SELECT torrent_id, snatched_at
            FROM torrent_activities
            WHERE p_torrent_snatched_by_id IS NOT NULL AND user_id = p_torrent_snatched_by_id
        ),
        filtered_torrents AS (
            SELECT t.*, st.snatched_at
            FROM torrents_and_reports t
            LEFT JOIN snatched_torrents st ON t.id = st.torrent_id
            WHERE (p_torrent_staff_checked IS NULL OR t.staff_checked = p_torrent_staff_checked)
            AND (
                    p_torrent_reported IS NULL
                OR (p_torrent_reported = TRUE AND t.reports::jsonb <> '[]'::jsonb)
                OR (p_torrent_reported = FALSE AND t.reports::jsonb = '[]'::jsonb)
            )
            AND (p_torrent_created_by_id IS NULL OR
                (t.created_by_id = p_torrent_created_by_id AND
                (NOT t.uploaded_as_anonymous OR t.created_by_id = p_requesting_user_id)))
            AND (p_torrent_snatched_by_id IS NULL OR st.torrent_id IS NOT NULL)
        ),
        edition_groups_with_torrents AS (
            SELECT
                eg.id AS eg_id,
                eg.title_group_id,
                jsonb_strip_nulls(jsonb_build_object(
                    'id', eg.id,
                    'title_group_id', eg.title_group_id,
                    'name', eg.name,
                    'release_date', eg.release_date,
                    'distributor', eg.distributor,
                    'covers', eg.covers,
                    'source', eg.source,
                    'additional_information', eg.additional_information,
                    'torrents', COALESCE(jsonb_agg(
                        jsonb_strip_nulls(jsonb_build_object(
                            'id', ft.id, 'upload_factor', ft.upload_factor, 'download_factor', ft.download_factor,
                            'seeders', ft.seeders, 'leechers', ft.leechers, 'completed', ft.completed,
                            'edition_group_id', ft.edition_group_id, 'created_at', ft.created_at, 'extras', ft.extras,
                            'release_name', ft.release_name, 'release_group', ft.release_group,
                            'file_amount_per_type', ft.file_amount_per_type, 'trumpable', ft.trumpable,
                            'staff_checked', ft.staff_checked, 'languages', ft.languages,
                            'container', ft.container, 'size', ft.size, 'duration', ft.duration,
                            'audio_codec', ft.audio_codec, 'audio_bitrate', ft.audio_bitrate,
                            'audio_bitrate_sampling', ft.audio_bitrate_sampling, 'audio_channels', ft.audio_channels,
                            'video_codec', ft.video_codec, 'features', ft.features,
                            'subtitle_languages', ft.subtitle_languages, 'video_resolution', ft.video_resolution,
                            'video_resolution_other_x', ft.video_resolution_other_x, 'video_resolution_other_y', ft.video_resolution_other_y,
                            'reports', ft.reports, 'snatched_at', ft.snatched_at, -- 'peer_status', ft.peer_status,
                            -- Handle anonymity: show creator info only if requesting user is the uploader or if not anonymous
                            'created_by_id', CASE
                                WHEN ft.uploaded_as_anonymous AND (p_requesting_user_id IS NULL OR ft.created_by_id != p_requesting_user_id) THEN NULL
                                ELSE ft.created_by_id
                            END,
                            'created_by', CASE
                                WHEN ft.uploaded_as_anonymous AND (p_requesting_user_id IS NULL OR ft.created_by_id != p_requesting_user_id) THEN NULL
                                ELSE ft.display_created_by
                            END,
                            'uploaded_as_anonymous', ft.uploaded_as_anonymous
                        )) ORDER BY ft.id
                    ) FILTER (WHERE ft.id IS NOT NULL), '[]'::jsonb)
                )) AS eg_data,
                MIN(ft.created_at) AS min_torrent_created_at,
                MAX(ft.created_at) AS max_torrent_created_at,
                MIN(ft.size) AS min_torrent_size,
                MAX(ft.size) AS max_torrent_size,
                MIN(ft.snatched_at) AS min_torrent_snatched_at,
                MAX(ft.snatched_at) AS max_torrent_snatched_at
            FROM edition_groups eg
            LEFT JOIN filtered_torrents ft ON eg.id = ft.edition_group_id
            GROUP BY eg.id
        ),
        title_groups_with_relevance AS (
            SELECT
                tg.id,
                tg.name,
                tg.covers,
                tg.category,
                tg.content_type,
                tg.tags,
                tg.original_release_date,
                CASE
                    WHEN p_external_link IS NOT NULL THEN 1.0
                    WHEN p_title_group_name IS NOT NULL THEN
                        ts_rank_cd(
                            to_tsvector('simple', tg.name || ' ' || coalesce(array_to_string(tg.name_aliases, ' '), '')),
                            plainto_tsquery('simple', p_title_group_name)
                        )
                    ELSE NULL
                END AS relevance_score,
                to_tsvector('simple', tg.name || ' ' || coalesce(array_to_string(tg.name_aliases, ' '), '')) AS search_vector
            FROM title_groups tg
            WHERE
                -- No filters if both empty
                (p_title_group_name IS NULL AND p_external_link IS NULL)
                OR
                -- If link provided → exact match on external_links
                (p_external_link IS NOT NULL AND p_external_link = ANY (tg.external_links))
                OR
                -- Else (name provided) → FTS on name + aliases
                (p_external_link IS NULL AND p_title_group_name IS NOT NULL AND
                to_tsvector('simple', tg.name || ' ' || coalesce(array_to_string(tg.name_aliases, ' '), ''))
                    @@ plainto_tsquery('simple', p_title_group_name))
        ),
        affiliated_artists_data AS (
            SELECT
                aa.title_group_id,
                jsonb_agg(
                    jsonb_build_object(
                        'id', ar.id,
                        'name', ar.name
                    ) ORDER BY ar.name
                ) AS affiliated_artists
            FROM affiliated_artists aa
            JOIN artists ar ON aa.artist_id = ar.id
            GROUP BY aa.title_group_id
        )
        SELECT
            tgr.id AS title_group_id,
            jsonb_strip_nulls(jsonb_build_object(
                'id', tgr.id,
                'name', tgr.name,
                'covers', tgr.covers,
                'category', tgr.category,
                'content_type', tgr.content_type,
                'tags', tgr.tags,
                'original_release_date', tgr.original_release_date
            ) || jsonb_build_object(
                'edition_groups', COALESCE(jsonb_agg(egwt.eg_data ORDER BY egwt.eg_id) FILTER (WHERE egwt.eg_data IS NOT NULL), '[]'::jsonb),
                'affiliated_artists', COALESCE(aad.affiliated_artists, '[]'::jsonb)
            )) AS title_group_data
        FROM title_groups_with_relevance tgr
        LEFT JOIN edition_groups_with_torrents egwt ON tgr.id = egwt.title_group_id
        LEFT JOIN affiliated_artists_data aad ON tgr.id = aad.title_group_id
        WHERE (p_include_empty_groups = TRUE OR (egwt.eg_data IS NOT NULL AND (egwt.eg_data -> 'torrents')::jsonb <> '[]'::jsonb))
        GROUP BY
            tgr.id, tgr.name, tgr.covers, tgr.category, tgr.content_type, tgr.tags, tgr.original_release_date, tgr.relevance_score, aad.affiliated_artists
        ORDER BY
            CASE
                WHEN p_sort_by = 'relevance' AND p_order = 'asc' THEN tgr.relevance_score
                ELSE NULL
            END ASC NULLS LAST,
            CASE
                WHEN p_sort_by = 'relevance' AND p_order = 'desc' THEN tgr.relevance_score
                ELSE NULL
            END DESC NULLS LAST,
            CASE
                WHEN p_sort_by = 'torrent_created_at' AND p_order = 'asc' THEN MIN(egwt.min_torrent_created_at)
                ELSE NULL
            END ASC NULLS LAST,
            CASE
                WHEN p_sort_by = 'torrent_created_at' AND p_order = 'desc' THEN MAX(egwt.max_torrent_created_at)
                ELSE NULL
            END DESC NULLS LAST,
            CASE
                WHEN p_sort_by = 'torrent_size' AND p_order = 'asc' THEN MIN(egwt.min_torrent_size)
                ELSE NULL
            END ASC NULLS LAST,
            CASE
                WHEN p_sort_by = 'torrent_size' AND p_order = 'desc' THEN MAX(egwt.max_torrent_size)
                ELSE NULL
            END DESC NULLS LAST,
            CASE
                WHEN p_sort_by = 'torrent_snatched_at' AND p_order = 'asc' THEN MAX(egwt.max_torrent_snatched_at)
                ELSE NULL
            END ASC NULLS LAST,
            CASE
                WHEN p_sort_by = 'torrent_snatched_at' AND p_order = 'desc' THEN MAX(egwt.max_torrent_snatched_at)
                ELSE NULL
            END DESC NULLS LAST,
            CASE
                WHEN p_sort_by = 'title_group_original_release_date' AND p_order = 'asc' THEN tgr.original_release_date
                ELSE NULL
            END ASC NULLS LAST,
            CASE
                WHEN p_sort_by = 'title_group_original_release_date' AND p_order = 'desc' THEN tgr.original_release_date
                ELSE NULL
            END DESC NULLS LAST,
            tgr.id ASC
        LIMIT p_limit OFFSET p_offset;
    END;
    $$;
