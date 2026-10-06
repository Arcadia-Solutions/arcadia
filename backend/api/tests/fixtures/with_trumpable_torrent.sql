-- Marks torrent 2 (from the `with_test_torrent` fixture) as trumpable by giving it a reason.
-- Torrent 1 keeps its empty-string trumpable value, which means "not trumpable", so the two
-- torrents exercise both sides of the torrent search trumpable filter.
UPDATE torrents
SET
  trumpable = 'forced subtitles'
WHERE
  id = 2;