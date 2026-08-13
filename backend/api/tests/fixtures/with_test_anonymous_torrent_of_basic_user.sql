-- Only anonymous upload of the basic user (id 100)
INSERT INTO
  torrents (
    id, edition_group_id, created_by_id, info_hash, info_dict, languages,
    release_name, release_group, description, file_amount_per_type,
    uploaded_as_anonymous, file_list, mediainfo, trumpable, staff_checked, container, size
  )
VALUES
  (
    902, 1, 100, '\xcc11223344556677889900aabbccddeeff112233', '{}', '{}',
    'Only anonymously uploaded torrent', '', '', '{}', TRUE, '{}', '{}', FALSE, FALSE, 'zip', 104857600
  );
