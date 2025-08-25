//! Iterators over torrent file information.

use bip_util::sha;

use metainfo::File;

/// Iterator over each File within the MetainfoFile.
pub struct Files<'a> {
    index: usize,
    files: &'a [File],
}

impl<'a> Files<'a> {
    pub fn new(files: &'a [File]) -> Files<'a> {
        Files {
            index: 0,
            files: files,
        }
    }
}

impl<'a> Iterator for Files<'a> {
    type Item = &'a File;

    fn next(&mut self) -> Option<&'a File> {
        if let Some(file) = self.files.get(self.index) {
            self.index += 1;
            Some(file)
        } else {
            None
        }
    }
}

// ----------------------------------------------------------------------------//

/// Iterator over each piece hash within the MetainfoFile.
pub struct Pieces<'a> {
    index: usize,
    pieces: &'a [[u8; sha::SHA_HASH_LEN]],
}

impl<'a> Pieces<'a> {
    pub fn new(pieces: &'a [[u8; sha::SHA_HASH_LEN]]) -> Pieces<'a> {
        Pieces {
            index: 0,
            pieces: pieces,
        }
    }
}

impl<'a> Iterator for Pieces<'a> {
    type Item = &'a [u8];

    fn next(&mut self) -> Option<&'a [u8]> {
        if let Some(hash) = self.pieces.get(self.index) {
            self.index += 1;
            Some(hash)
        } else {
            None
        }
    }
}