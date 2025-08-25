use std::fs::File;
use std::io::{self, Cursor, Read};
use std::path::{Path, PathBuf};

use bip_util::sha::ShaHash;
use walkdir::{self, WalkDir, DirEntry};

/// Trait for types convertible as a Result into some Accessor.
pub trait IntoAccessor {
    /// Concrete Accessor type that will be converted into.
    type Accessor: Accessor;

    /// Convert the type into some Accessor as a Result.
    fn into_accessor(self) -> io::Result<Self::Accessor>;
}

/// Trait for accessing the data used to construct a torrent file.
pub trait Accessor {
    /// Access the directory that all files should be relative to.
    fn access_directory(&self) -> Option<&Path>;

    /// Access the metadata for all files including their length and path.
    fn access_metadata<C>(&self, callback: C) -> io::Result<()> where C: FnMut(u64, &Path);

    /// Access the sequential pieces that make up all of the files.
    fn access_pieces<C>(&self, callback: C) -> io::Result<()>
        where C: for<'a> FnMut(PieceAccess<'a>) -> io::Result<()>;
}

impl<'a, T> Accessor for &'a T
    where T: Accessor
{
    fn access_directory(&self) -> Option<&Path> {
        Accessor::access_directory(*self)
    }

    fn access_metadata<C>(&self, callback: C) -> io::Result<()>
        where C: FnMut(u64, &Path)
    {
        Accessor::access_metadata(*self, callback)
    }

    fn access_pieces<C>(&self, callback: C) -> io::Result<()>
        where C: for<'b> FnMut(PieceAccess<'b>) -> io::Result<()>
    {
        Accessor::access_pieces(*self, callback)
    }
}

// ----------------------------------------------------------------------------//

/// Type of access given for computing (or not) the checksums for torrent files.
///
/// Implementations should typically choose to invoke the `access_pieces` callback
/// with all `Compute` variants, or all `PreComputed` variants. It is allowable to
/// mix and match variants between calls, but any `PreComputed` hashes will be
/// considered as the next checksum to be put in the torrent file, so implementations
/// will typically want to align calls with `Compute` to a piece length boundary
/// (though not required).
pub enum PieceAccess<'a> {
    /// Hash should be computed from the bytes read.
    Compute(&'a mut Read),
    /// Hash given should be used directly as the next checksum.
    PreComputed(ShaHash)
}

// ----------------------------------------------------------------------------//

/// Accessor that pulls data in from the file system.
pub struct FileAccessor {
    absolute_path:  PathBuf,
    directory_name: Option<PathBuf>,
}

impl FileAccessor {
    /// Create a new FileAccessor from the given file/directory path.
    pub fn new<T>(path: T) -> io::Result<FileAccessor>
        where T: AsRef<Path>
    {
        let absolute_path = try!(path.as_ref().canonicalize());
        let directory_name = if absolute_path.is_dir() {
            let dir_name: &Path = absolute_path.iter().last().unwrap().as_ref();

            Some(dir_name.to_path_buf())
        } else {
            None
        };

        Ok(FileAccessor {
            absolute_path: absolute_path,
            directory_name: directory_name,
        })
    }
}

impl IntoAccessor for FileAccessor {
    type Accessor = FileAccessor;

    fn into_accessor(self) -> io::Result<FileAccessor> {
        Ok(self)
    }
}

impl<T> IntoAccessor for T
    where T: AsRef<Path>
{
    type Accessor = FileAccessor;

    fn into_accessor(self) -> io::Result<FileAccessor> {
        FileAccessor::new(self)
    }
}

impl Accessor for FileAccessor {
    fn access_directory(&self) -> Option<&Path> {
        self.directory_name.as_ref().map(|s| s.as_ref())
    }

    fn access_metadata<C>(&self, mut callback: C) -> io::Result<()>
        where C: FnMut(u64, &Path)
    {
        let num_skip_paths = if self.access_directory().is_some() {
            self.absolute_path.iter().count()
        } else {
            self.absolute_path.iter().count() - 1
        };

        for res_entry in WalkDir::new(&self.absolute_path).into_iter().filter(entry_file_filter) {
            let entry = try!(res_entry);
            let entry_metadata = try!(entry.metadata());

            let file_length = entry_metadata.len();
            // TODO: Switch to using strip_relative when it is stabilized
            let relative_path =
                entry.path().iter().skip(num_skip_paths).fold(PathBuf::new(), |mut acc, nex| {
                    acc.push(nex);
                    acc
                });

            callback(file_length, relative_path.as_path());
        }

        Ok(())
    }

    fn access_pieces<C>(&self, mut callback: C) -> io::Result<()>
        where C: for<'a> FnMut(PieceAccess<'a>) -> io::Result<()>
    {
        for res_entry in WalkDir::new(&self.absolute_path).into_iter().filter(entry_file_filter) {
            let entry = try!(res_entry);
            let mut file = try!(File::open(entry.path()));

            try!(callback(PieceAccess::Compute(&mut file)));
        }

        Ok(())
    }
}

/// Filter that yields true if the entry points to a file.
fn entry_file_filter(res_entry: &walkdir::Result<DirEntry>) -> bool {
    res_entry.as_ref().map(|f| f.file_type().is_file()).unwrap_or(true)
}

// ----------------------------------------------------------------------------//

/// Accessor that pulls data in directly from memory.
pub struct DirectAccessor<'a> {
    file_name: &'a str,
    file_contents: &'a [u8],
}

impl<'a> DirectAccessor<'a> {
    /// Create a new DirectAccessor from the given file name and contents.
    pub fn new(file_name: &'a str, file_contents: &'a [u8]) -> DirectAccessor<'a> {
        DirectAccessor {
            file_name: file_name,
            file_contents: file_contents,
        }
    }
}

impl<'a> IntoAccessor for DirectAccessor<'a> {
    type Accessor = DirectAccessor<'a>;

    fn into_accessor(self) -> io::Result<DirectAccessor<'a>> {
        Ok(self)
    }
}

impl<'a> Accessor for DirectAccessor<'a> {
    fn access_directory(&self) -> Option<&Path> {
        None
    }

    fn access_metadata<C>(&self, mut callback: C) -> io::Result<()>
        where C: FnMut(u64, &Path)
    {
        let file_path = Path::new(self.file_name);
        let file_length = self.file_contents.len() as u64;

        callback(file_length, file_path);

        Ok(())
    }

    fn access_pieces<C>(&self, mut callback: C) -> io::Result<()>
        where C: for<'b> FnMut(PieceAccess<'b>) -> io::Result<()>
    {
        let mut cursor = Cursor::new(self.file_contents);

        callback(PieceAccess::Compute(&mut cursor))
    }
}
