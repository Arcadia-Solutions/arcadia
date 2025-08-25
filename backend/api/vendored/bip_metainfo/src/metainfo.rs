//! Accessing the fields of a Metainfo file.
use std::path::{Path, PathBuf};
use std::io;

use bip_bencode::{BencodeRef, BDictAccess, BDecodeOpt, BRefAccess};
use bip_util::bt::InfoHash;
use bip_util::sha::{self, ShaHash};

use accessor::{Accessor, PieceAccess, IntoAccessor};
use builder::{MetainfoBuilder, InfoBuilder, PieceLength};
use parse;
use error::{ParseError, ParseErrorKind, ParseResult};
use iter::{Files, Pieces};

/// Contains optional metadata for a torrent file.
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Metainfo {
    comment: Option<String>,
    announce: Option<String>,
    announce_list: Option<Vec<Vec<String>>>,
    encoding: Option<String>,
    created_by: Option<String>,
    creation_date: Option<i64>,
    info: Info,
}

impl Metainfo {
    /// Read a `Metainfo` from metainfo file bytes.
    pub fn from_bytes<B>(bytes: B) -> ParseResult<Metainfo>
        where B: AsRef<[u8]>
    {
        let bytes_slice = bytes.as_ref();

        parse_meta_bytes(bytes_slice)
    }

    /// Announce url for the main tracker of the metainfo file.
    pub fn main_tracker(&self) -> Option<&str> {
        self.announce.as_ref().map(|a| &a[..])
    }

    /// List of announce urls.
    pub fn trackers(&self) -> Option<&Vec<Vec<String>>> {
        self.announce_list.as_ref()
    }

    /// Comment included within the metainfo file.
    pub fn comment(&self) -> Option<&str> {
        self.comment.as_ref().map(|c| &c[..])
    }

    /// Person or group that created the metainfo file.
    pub fn created_by(&self) -> Option<&str> {
        self.created_by.as_ref().map(|c| &c[..])
    }

    /// String encoding format of the peices portion of the info dictionary.
    pub fn encoding(&self) -> Option<&str> {
        self.encoding.as_ref().map(|e| &e[..])
    }

    /// Creation date in UNIX epoch format for the metainfo file.
    pub fn creation_date(&self) -> Option<i64> {
        self.creation_date
    }

    /// Info dictionary for the metainfo file.
    pub fn info(&self) -> &Info {
        &self.info
    }

    /// Retrieve the bencoded bytes for the `Metainfo` file.
    pub fn to_bytes(&self) -> Vec<u8> {
        // Since there are no file system accesses here, should be fine to unwrap
        MetainfoBuilder::new()
            .set_main_tracker(self.main_tracker())
            .set_creation_date(self.creation_date())
            .set_comment(self.comment())
            .set_created_by(self.created_by())
            .set_private_flag(self.info().is_private())
            // TODO: Revisit this cast...
            .set_piece_length(PieceLength::Custom(self.info().piece_length() as usize))
            .build(1, &self.info, |_| ())
            .unwrap()
    }
}

impl From<Info> for Metainfo {
    fn from(info: Info) -> Metainfo {
        Metainfo{
            comment: None,
            announce: None,
            announce_list: None,
            encoding: None,
            created_by: None,
            creation_date: None,
            info: info
        }
    }
}

/// Parses the given metainfo bytes and builds a Metainfo from them.
fn parse_meta_bytes(bytes: &[u8]) -> ParseResult<Metainfo> {
    let root_bencode = try!(BencodeRef::decode(bytes, BDecodeOpt::default()));
    let root_dict = try!(parse::parse_root_dict(&root_bencode));

    let announce = parse::parse_announce_url(root_dict).map(|e| e.to_owned());

    let opt_announce_list = {
        parse::parse_announce_list(root_dict)
            .and_then(|list| Some(parse::convert_announce_list(list)))
            .or(None)
    };

    let opt_comment = parse::parse_comment(root_dict).map(|e| e.to_owned());
    let opt_encoding = parse::parse_encoding(root_dict).map(|e| e.to_owned());
    let opt_created_by = parse::parse_created_by(root_dict).map(|e| e.to_owned());
    let opt_creation_date = parse::parse_creation_date(root_dict);

    let info_bencode = try!(parse::parse_info_bencode(root_dict));
    let info = try!(parse_info_dictionary(info_bencode));

    Ok(Metainfo {
        comment: opt_comment,
        announce: announce,
        announce_list: opt_announce_list,
        encoding: opt_encoding,
        created_by: opt_created_by,
        creation_date: opt_creation_date,
        info: info
    })
}

// ----------------------------------------------------------------------------//

/// Contains directory and checksum data for a torrent file.
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Info {
    info_hash:      InfoHash,
    files:          Vec<File>,
    pieces:         Vec<[u8; sha::SHA_HASH_LEN]>,
    piece_len:      u64,
    is_private:     Option<bool>,
    // Present only for multi file torrents.
    file_directory: Option<PathBuf>,
}

impl Info {
    /// Read an `Info` from info dictionary bytes.
    pub fn from_bytes<B>(bytes: B) -> ParseResult<Info>
        where B: AsRef<[u8]>
    {
        let bytes_slice = bytes.as_ref();

        parse_info_bytes(bytes_slice)
    }

    /// Hash to uniquely identify this torrent.
    pub fn info_hash(&self) -> InfoHash {
        self.info_hash
    }

    /// Some file directory if this is a multi-file torrent, otherwise None.
    ///
    /// If you want to check to see if this is a multi-file torrent, you should
    /// check whether or not this returns Some. Checking the number of files
    /// present is NOT the correct method.
    pub fn directory(&self) -> Option<&Path> {
        self.file_directory.as_ref().map(|d| d.as_ref())
    }

    /// Length in bytes of each piece.
    pub fn piece_length(&self) -> u64 {
        self.piece_len
    }

    /// Whether or not the torrent is private.
    pub fn is_private(&self) -> Option<bool> {
        self.is_private
    }

    /// Iterator over each of the pieces SHA-1 hash.
    ///
    /// Ordering of pieces yielded in the iterator is guaranteed to be the order in
    /// which they are found in the torrent file as this is necessary to refer to
    /// pieces by their index to other peers.
    pub fn pieces<'a>(&'a self) -> Pieces<'a> {
        Pieces::new(&self.pieces)
    }

    /// Iterator over each file within the torrent file.
    ///
    /// Ordering of files yielded in the iterator is guaranteed to be the order in
    /// which they are found in the torrent file as this is necessary to reconstruct
    /// pieces received from peers.
    pub fn files<'a>(&'a self) -> Files<'a> {
        Files::new(&self.files)
    }

    /// Retrieve the bencoded bytes for the `Info` dictionary.
    pub fn to_bytes(&self) -> Vec<u8> {
        // Since there are no file system accesses here, should be fine to unwrap
        InfoBuilder::new()
            .set_private_flag(self.is_private())
            // TODO: Revisit this cast...
            .set_piece_length(PieceLength::Custom(self.piece_length() as usize))
            .build(1, self, |_| ())
            .unwrap()
    }
}

impl IntoAccessor for Info {
    type Accessor = Info;

    fn into_accessor(self) -> io::Result<Info> {
        Ok(self)
    }
}

impl<'a> IntoAccessor for &'a Info {
    type Accessor = &'a Info;

    fn into_accessor(self) -> io::Result<&'a Info> {
        Ok(self)
    }
}

impl Accessor for Info {
    fn access_directory(&self) -> Option<&Path> {
        self.directory()
    }

    fn access_metadata<C>(&self, mut callback: C) -> io::Result<()>
        where C: FnMut(u64, &Path) {
        for file in self.files() {
            callback(file.length(), file.path());
        }

        Ok(())
    }

    fn access_pieces<C>(&self, mut callback: C) -> io::Result<()>
        where C: for<'a> FnMut(PieceAccess<'a>) -> io::Result<()> {
        for piece in self.pieces() {
            try!(callback(PieceAccess::PreComputed(ShaHash::from_hash(piece).unwrap())));
        }
        
        Ok(())
    }
}

/// Parses the given info dictionary bytes and builds a Metainfo from them.
fn parse_info_bytes(bytes: &[u8]) -> ParseResult<Info> {
    let info_bencode = try!(BencodeRef::decode(bytes, BDecodeOpt::default()));

    parse_info_dictionary(&info_bencode)
}

/// Parses the given info dictionary and builds an Info from it.
fn parse_info_dictionary<'a>(info_bencode: &BencodeRef<'a>) -> ParseResult<Info> {
    let info_hash = InfoHash::from_bytes(info_bencode.buffer());

    let info_dict = try!(parse::parse_root_dict(info_bencode));
    let piece_len = try!(parse::parse_piece_length(info_dict));
    let is_private = parse::parse_private(info_dict);

    let pieces = try!(parse::parse_pieces(info_dict));
    let piece_buffers = try!(allocate_pieces(pieces));

    if is_multi_file_torrent(info_dict) {
        let file_directory = try!(parse::parse_name(info_dict));
        let mut file_directory_path = PathBuf::new();
        file_directory_path.push(file_directory);

        let files_bencode = try!(parse::parse_files_list(info_dict));

        let mut files_list = Vec::with_capacity(files_bencode.len());
        for file_bencode in files_bencode {
            let file_dict = try!(parse::parse_file_dict(file_bencode));
            let file = try!(File::as_multi_file(file_dict));

            files_list.push(file);
        }

        Ok(Info {
            info_hash: info_hash,
            files: files_list,
            pieces: piece_buffers,
            piece_len: piece_len,
            is_private: is_private,
            file_directory: Some(file_directory_path),
        })
    } else {
        let file = try!(File::as_single_file(info_dict));

        Ok(Info {
            info_hash: info_hash,
            files: vec![file],
            pieces: piece_buffers,
            piece_len: piece_len,
            is_private: is_private,
            file_directory: None,
        })
    }
}

/// Returns whether or not this is a multi file torrent.
fn is_multi_file_torrent<B>(info_dict: &BDictAccess<B::BKey, B>) -> bool
    where B: BRefAccess {
    parse::parse_length(info_dict).is_err()
}

/// Validates and allocates the hash pieces on the heap.
fn allocate_pieces(pieces: &[u8]) -> ParseResult<Vec<[u8; sha::SHA_HASH_LEN]>> {
    if pieces.len() % sha::SHA_HASH_LEN != 0 {
        let error_msg = format!("Piece Hash Length Of {} Is Invalid", pieces.len());
        Err(ParseError::from_kind(ParseErrorKind::MissingData { details: error_msg }))
    } else {
        let mut hash_buffers = Vec::with_capacity(pieces.len() / sha::SHA_HASH_LEN);
        let mut hash_bytes = [0u8; sha::SHA_HASH_LEN];

        for chunk in pieces.chunks(sha::SHA_HASH_LEN) {
            for (src, dst) in chunk.iter().zip(hash_bytes.iter_mut()) {
                *dst = *src;
            }

            hash_buffers.push(hash_bytes);
        }

        Ok(hash_buffers)
    }
}

// ----------------------------------------------------------------------------//

/// Contains information for a single file.
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct File {
    len:    u64,
    path:   PathBuf,
    md5sum: Option<Vec<u8>>,
}

impl File {
    /// Parse the info dictionary and generate a single file File.
    fn as_single_file<B>(info_dict: &BDictAccess<B::BKey, B>) -> ParseResult<File>
        where B: BRefAccess {
        let length = try!(parse::parse_length(info_dict));
        let md5sum = parse::parse_md5sum(info_dict).map(|m| m.to_owned());
        let name = try!(parse::parse_name(info_dict));

        Ok(File {
            len: length,
            path: name.to_owned().into(),
            md5sum: md5sum,
        })
    }

    /// Parse the file dictionary and generate a multi file File.
    fn as_multi_file<B>(file_dict: &BDictAccess<B::BKey, B>) -> ParseResult<File>
        where B: BRefAccess<BType=B> {
        let length = try!(parse::parse_length(file_dict));
        let md5sum = parse::parse_md5sum(file_dict).map(|m| m.to_owned());

        let path_list_bencode = try!(parse::parse_path_list(file_dict));

        let mut path_buf = PathBuf::new();
        for path_bencode in path_list_bencode {
            let path = try!(parse::parse_path_str(path_bencode));

            path_buf.push(path);
        }

        Ok(File {
            len: length,
            path: path_buf,
            md5sum: md5sum,
        })
    }

    /// Length of the file in bytes.
    pub fn length(&self) -> u64 {
        self.len
    }

    /// Optional md5sum of the file.
    ///
    /// Not used by bittorrent.
    pub fn md5sum(&self) -> Option<&[u8]> {
        self.md5sum.as_ref().map(|m| &m[..])
    }

    /// Path of the file.
    pub fn path(&self) -> &Path {
        &self.path
    }
}

#[cfg(test)]
mod tests {
    use std::path::{Path, PathBuf};

    use bip_bencode::{BencodeMut, BMutAccess};
    use bip_util::sha;
    use bip_util::bt::InfoHash;

    use metainfo::Metainfo;
    use parse;

    /// Helper function for manually constructing a metainfo file based on the parameters given.
    ///
    /// If the metainfo file builds successfully, assertions will be made about the contents of it based
    /// on the parameters given.
    fn validate_parse_from_params(tracker: Option<&str>,
                                  create_date: Option<i64>,
                                  comment: Option<&str>,
                                  create_by: Option<&str>,
                                  encoding: Option<&str>,
                                  piece_length: Option<i64>,
                                  pieces: Option<&[u8]>,
                                  private: Option<i64>,
                                  directory: Option<&str>,
                                  files: Option<Vec<(Option<i64>,
                                                     Option<&[u8]>,
                                                     Option<Vec<String>>)>>) {
        let mut root_dict = BencodeMut::new_dict();
        let info_hash = {
            let root_dict_access = root_dict.dict_mut().unwrap();
            
            tracker.map(|t| root_dict_access.insert(parse::ANNOUNCE_URL_KEY.into(), ben_bytes!(t)));
            create_date.as_ref().map(|&c| root_dict_access.insert(parse::CREATION_DATE_KEY.into(), ben_int!(c)));
            comment.map(|c| root_dict_access.insert(parse::COMMENT_KEY.into(), ben_bytes!(c)));
            create_by.map(|c| root_dict_access.insert(parse::CREATED_BY_KEY.into(), ben_bytes!(c)));
            encoding.map(|e| root_dict_access.insert(parse::ENCODING_KEY.into(), ben_bytes!(e)));

            let mut info_dict = BencodeMut::new_dict();
            {
                let info_dict_access = info_dict.dict_mut().unwrap();

                piece_length.as_ref().map(|&p| info_dict_access.insert(parse::PIECE_LENGTH_KEY.into(), ben_int!(p)));
                pieces.map(|p| info_dict_access.insert(parse::PIECES_KEY.into(), ben_bytes!(p)));
                private.as_ref().map(|&p| info_dict_access.insert(parse::PRIVATE_KEY.into(), ben_int!(p)));

                directory
                    .and_then(|d| {
                        // We intended to build a multi file torrent since we provided a directory
                        info_dict_access.insert(parse::NAME_KEY.into(), ben_bytes!(d));

                        files.as_ref().map(|files| {
                            let mut bencode_files = BencodeMut::new_list();

                            {
                                let bencode_files_access = bencode_files.list_mut().unwrap();

                                for &(ref opt_len, ref opt_md5, ref opt_paths) in files.iter() {
                                    let opt_bencode_paths = opt_paths.as_ref().map(|paths| {
                                        let mut bencode_paths = BencodeMut::new_list();

                                        {
                                            let bencode_paths_access = bencode_paths.list_mut().unwrap();
                                            for path in paths.iter() {
                                                bencode_paths_access.push(ben_bytes!(&path[..]));
                                            }
                                        }

                                        bencode_paths
                                    });

                                    let mut file_dict = BencodeMut::new_dict();
                                    {
                                        let file_dict_access = file_dict.dict_mut().unwrap();

                                        opt_bencode_paths.map(|p| file_dict_access.insert(parse::PATH_KEY.into(), p));
                                        opt_len.map(|l| file_dict_access.insert(parse::LENGTH_KEY.into(), ben_int!(l)));
                                        opt_md5.map(|m| file_dict_access.insert(parse::MD5SUM_KEY.into(), ben_bytes!(m)));
                                    }

                                    bencode_files_access.push(file_dict)
                                }
                            }

                            info_dict_access.insert(parse::FILES_KEY.into(), bencode_files);
                        });

                        Some(d)
                    })
                    .or_else(|| {
                        // We intended to build a single file torrent if a directory was not specified
                        files.as_ref().map(|files| {
                            let (ref opt_len, ref opt_md5, ref opt_path) = files[0];

                            opt_path.as_ref().map(|p| info_dict_access.insert(parse::NAME_KEY.into(), ben_bytes!(&p[0][..])));
                            opt_len.map(|l| info_dict_access.insert(parse::LENGTH_KEY.into(), ben_int!(l)));
                            opt_md5.map(|m| info_dict_access.insert(parse::MD5SUM_KEY.into(), ben_bytes!(m)));
                        });

                        None
                    });
            }
            let info_hash = InfoHash::from_bytes(&info_dict.encode());

            root_dict_access.insert(parse::INFO_KEY.into(), info_dict);

            info_hash
        };

        let metainfo_file = Metainfo::from_bytes(root_dict.encode()).unwrap();

        assert_eq!(metainfo_file.info().info_hash(), info_hash);
        assert_eq!(metainfo_file.comment(), comment);
        assert_eq!(metainfo_file.created_by(), create_by);
        assert_eq!(metainfo_file.encoding(), encoding);
        assert_eq!(metainfo_file.creation_date, create_date);

        assert_eq!(metainfo_file.info().directory(), directory.map(|d| d.as_ref()));
        assert_eq!(metainfo_file.info().piece_length(), piece_length.unwrap() as u64);
        assert_eq!(metainfo_file.info().is_private(), private.map(|private| private == 1));

        let pieces = pieces.unwrap();
        assert_eq!(pieces.chunks(sha::SHA_HASH_LEN).count(),
                   metainfo_file.info().pieces().count());
        for (piece_chunk, piece_elem) in pieces.chunks(sha::SHA_HASH_LEN)
            .zip(metainfo_file.info().pieces()) {
            assert_eq!(piece_chunk, piece_elem);
        }

        let num_files = files.as_ref().map(|f| f.len()).unwrap_or(0);
        assert_eq!(metainfo_file.info().files().count(), num_files);

        let mut supp_files = files.as_ref().unwrap().iter();
        let mut meta_files = metainfo_file.info().files();
        for _ in 0..num_files {
            let meta_file = meta_files.next().unwrap();
            let supp_file = supp_files.next().unwrap();

            assert_eq!(meta_file.length(), supp_file.0.unwrap() as u64);
            assert_eq!(meta_file.md5sum(), supp_file.1);

            let meta_paths: &Path = meta_file.path();
            let supp_paths: PathBuf = supp_file.2.as_ref().unwrap().iter().fold(PathBuf::new(), |mut buf, item| {
                let item: &str = item;
                buf.push(item);
                buf
            });
            assert_eq!(meta_paths, supp_paths);
        }
    }

    #[test]
    fn positive_parse_from_single_file() {
        let tracker = "udp://dummy_domain.com:8989";
        let piece_len = 1024;
        let pieces = [0u8; sha::SHA_HASH_LEN];

        let file_len = 0;
        let file_paths = vec!["dummy_file_name".to_owned()];

        validate_parse_from_params(Some(tracker),
                                   None,
                                   None,
                                   None,
                                   None,
                                   Some(piece_len),
                                   Some(&pieces),
                                   None,
                                   None,
                                   Some(vec![(Some(file_len), None, Some(file_paths))]));
    }

    #[test]
    fn positive_parse_from_multi_file() {
        let tracker = "udp://dummy_domain.com:8989";
        let piece_len = 1024;
        let pieces = [0u8; sha::SHA_HASH_LEN];

        let directory = "dummy_file_directory";
        let files = vec![(Some(0),
                          None,
                          Some(vec!["dummy_sub_directory".to_owned(),
                                    "dummy_file_name".to_owned()]))];

        validate_parse_from_params(Some(tracker),
                                   None,
                                   None,
                                   None,
                                   None,
                                   Some(piece_len),
                                   Some(&pieces),
                                   None,
                                   Some(directory),
                                   Some(files));
    }

    #[test]
    fn positive_parse_from_multi_files() {
        let tracker = "udp://dummy_domain.com:8989";
        let piece_len = 1024;
        let pieces = [0u8; sha::SHA_HASH_LEN];

        let directory = "dummy_file_directory";
        let files = vec![(Some(0),
                          None,
                          Some(vec!["dummy_sub_directory".to_owned(),
                                    "dummy_file_name".to_owned()])),
                         (Some(5), None, Some(vec!["other_dummy_file_name".to_owned()]))];

        validate_parse_from_params(Some(tracker),
                                   None,
                                   None,
                                   None,
                                   None,
                                   Some(piece_len),
                                   Some(&pieces),
                                   None,
                                   Some(directory),
                                   Some(files));
    }

    #[test]
    fn positive_parse_from_empty_pieces() {
        let tracker = "udp://dummy_domain.com:8989";
        let piece_len = 1024;
        let pieces = [0u8; 0];

        let file_len = 0;
        let file_paths = vec!["dummy_file_name".to_owned()];

        validate_parse_from_params(Some(tracker),
                                   None,
                                   None,
                                   None,
                                   None,
                                   Some(piece_len),
                                   Some(&pieces),
                                   None,
                                   None,
                                   Some(vec![(Some(file_len), None, Some(file_paths))]));
    }

    #[test]
    fn positive_parse_with_creation_date() {
        let tracker = "udp://dummy_domain.com:8989";
        let piece_len = 1024;
        let pieces = [0u8; sha::SHA_HASH_LEN];

        let file_len = 0;
        let file_paths = vec!["dummy_file_name".to_owned()];

        let creation_date = 5050505050;

        validate_parse_from_params(Some(tracker),
                                   Some(creation_date),
                                   None,
                                   None,
                                   None,
                                   Some(piece_len),
                                   Some(&pieces),
                                   None,
                                   None,
                                   Some(vec![(Some(file_len), None, Some(file_paths))]));
    }

    #[test]
    fn positive_parse_with_comment() {
        let tracker = "udp://dummy_domain.com:8989";
        let piece_len = 1024;
        let pieces = [0u8; sha::SHA_HASH_LEN];

        let file_len = 0;
        let file_paths = vec!["dummy_file_name".to_owned()];

        let comment = "This is my boring test comment...";

        validate_parse_from_params(Some(tracker),
                                   None,
                                   Some(comment),
                                   None,
                                   None,
                                   Some(piece_len),
                                   Some(&pieces),
                                   None,
                                   None,
                                   Some(vec![(Some(file_len), None, Some(file_paths))]));
    }

    #[test]
    fn positive_parse_with_created_by() {
        let tracker = "udp://dummy_domain.com:8989";
        let piece_len = 1024;
        let pieces = [0u8; sha::SHA_HASH_LEN];

        let file_len = 0;
        let file_paths = vec!["dummy_file_name".to_owned()];

        let created_by = "Me";

        validate_parse_from_params(Some(tracker),
                                   None,
                                   None,
                                   Some(created_by),
                                   None,
                                   Some(piece_len),
                                   Some(&pieces),
                                   None,
                                   None,
                                   Some(vec![(Some(file_len), None, Some(file_paths))]));
    }

    #[test]
    fn positive_parse_with_encoding() {
        let tracker = "udp://dummy_domain.com:8989";
        let piece_len = 1024;
        let pieces = [0u8; sha::SHA_HASH_LEN];

        let file_len = 0;
        let file_paths = vec!["dummy_file_name".to_owned()];

        let encoding = "UTF-8";

        validate_parse_from_params(Some(tracker),
                                   None,
                                   None,
                                   None,
                                   Some(encoding),
                                   Some(piece_len),
                                   Some(&pieces),
                                   None,
                                   None,
                                   Some(vec![(Some(file_len), None, Some(file_paths))]));
    }

    #[test]
    fn positive_parse_with_private_zero() {
        let tracker = "udp://dummy_domain.com:8989";
        let piece_len = 1024;
        let pieces = [0u8; sha::SHA_HASH_LEN];

        let file_len = 0;
        let file_paths = vec!["dummy_file_name".to_owned()];

        let private = 0;

        validate_parse_from_params(Some(tracker),
                                   None,
                                   None,
                                   None,
                                   None,
                                   Some(piece_len),
                                   Some(&pieces),
                                   Some(private),
                                   None,
                                   Some(vec![(Some(file_len), None, Some(file_paths))]));
    }

    #[test]
    fn positive_parse_with_private_one() {
        let tracker = "udp://dummy_domain.com:8989";
        let piece_len = 1024;
        let pieces = [0u8; sha::SHA_HASH_LEN];

        let file_len = 0;
        let file_paths = vec!["dummy_file_name".to_owned()];

        let private = 1;

        validate_parse_from_params(Some(tracker),
                                   None,
                                   None,
                                   None,
                                   None,
                                   Some(piece_len),
                                   Some(&pieces),
                                   Some(private),
                                   None,
                                   Some(vec![(Some(file_len), None, Some(file_paths))]));
    }

    #[test]
    fn positive_parse_with_private_non_zero() {
        let tracker = "udp://dummy_domain.com:8989";
        let piece_len = 1024;
        let pieces = [0u8; sha::SHA_HASH_LEN];

        let file_len = 0;
        let file_paths = vec!["dummy_file_name".to_owned()];

        let private = -1;

        validate_parse_from_params(Some(tracker),
                                   None,
                                   None,
                                   None,
                                   None,
                                   Some(piece_len),
                                   Some(&pieces),
                                   Some(private),
                                   None,
                                   Some(vec![(Some(file_len), None, Some(file_paths))]));
    }

    #[test]
    fn positive_parse_with_no_main_tracker() {
        let piece_len = 1024;
        let pieces = [0u8; sha::SHA_HASH_LEN];

        let file_len = 0;
        let file_paths = vec!["dummy_file_name".to_owned()];

        validate_parse_from_params(None,
                                   None,
                                   None,
                                   None,
                                   None,
                                   Some(piece_len),
                                   Some(&pieces),
                                   None,
                                   None,
                                   Some(vec![(Some(file_len), None, Some(file_paths))]));
    }

    #[test]
    #[should_panic]
    fn negative_parse_from_empty_bytes() {
        Metainfo::from_bytes(b"").unwrap();
    }

    #[test]
    #[should_panic]
    fn negative_parse_with_no_piece_length() {
        let tracker = "udp://dummy_domain.com:8989";
        let pieces = [0u8; sha::SHA_HASH_LEN];

        let file_len = 0;
        let file_paths = vec!["dummy_file_name".to_owned()];

        let private = -1;

        validate_parse_from_params(Some(tracker),
                                   None,
                                   None,
                                   None,
                                   None,
                                   None,
                                   Some(&pieces),
                                   Some(private),
                                   None,
                                   Some(vec![(Some(file_len), None, Some(file_paths))]));
    }

    #[test]
    #[should_panic]
    fn negative_parse_with_no_pieces() {
        let tracker = "udp://dummy_domain.com:8989";
        let piece_len = 1024;

        let file_len = 0;
        let file_paths = vec!["dummy_file_name".to_owned()];

        validate_parse_from_params(Some(tracker),
                                   None,
                                   None,
                                   None,
                                   None,
                                   Some(piece_len),
                                   None,
                                   None,
                                   None,
                                   Some(vec![(Some(file_len), None, Some(file_paths))]));
    }

    #[test]
    #[should_panic]
    fn negative_parse_from_single_file_with_no_file_length() {
        let tracker = "udp://dummy_domain.com:8989";
        let piece_len = 1024;
        let pieces = [0u8; sha::SHA_HASH_LEN];

        let file_paths = vec!["dummy_file_name".to_owned()];

        validate_parse_from_params(Some(tracker),
                                   None,
                                   None,
                                   None,
                                   None,
                                   Some(piece_len),
                                   Some(&pieces),
                                   None,
                                   None,
                                   Some(vec![(None, None, Some(file_paths))]));
    }

    #[test]
    #[should_panic]
    fn negative_parse_from_single_file_with_no_file_name() {
        let tracker = "udp://dummy_domain.com:8989";
        let piece_len = 1024;
        let pieces = [0u8; sha::SHA_HASH_LEN];

        let file_len = 0;

        validate_parse_from_params(Some(tracker),
                                   None,
                                   None,
                                   None,
                                   None,
                                   Some(piece_len),
                                   Some(&pieces),
                                   None,
                                   None,
                                   Some(vec![(Some(file_len), None, None)]));
    }
}
