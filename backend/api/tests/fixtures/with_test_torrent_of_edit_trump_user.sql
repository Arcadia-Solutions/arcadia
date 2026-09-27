-- Torrent uploaded by user 164 (user_edit_trump), who holds the edit_torrent_trumpable
-- permission and can therefore mark their own torrent as trumpable
INSERT INTO
  torrents (
    id,
    edition_group_id,
    created_by_id,
    info_hash,
    info_dict,
    languages,
    release_name,
    release_group,
    description,
    file_amount_per_type,
    uploaded_as_anonymous,
    file_list,
    mediainfo,
    trumpable,
    staff_checked,
    container,
    size
  )
VALUES
  (
    903,
    1,
    164,
    '\xdd11223344556677889900aabbccddeeff112233',
    '{}',
    '{}',
    'Torrent of a user allowed to mark their own torrents as trumpable',
    '',
    '',
    '{}',
    FALSE,
    '{}',
    '{}',
    FALSE,
    FALSE,
    'zip',
    104857600
  );
