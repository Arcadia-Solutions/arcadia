use arcadia_common::error::{Error, Result};
use bip_metainfo::{Info, InfoBuilder, InfoHash, MetainfoBuilder, PieceLength};

/// Sets the info-dictionary fields that determine a torrent's info hash: the
/// `private` flag is forced on, the original piece length is preserved and the
/// configured source tag is injected. Implemented for both builders so that the
/// hash computed on upload/rehash always matches the .torrent served on download.
pub trait NormalizedInfoFields<'a>: Sized {
    fn with_normalized_info_fields(self, info: &Info, torrent_source_tag: Option<&'a str>) -> Self;
}

macro_rules! impl_normalized_info_fields {
    ($builder:ident) => {
        impl<'a> NormalizedInfoFields<'a> for $builder<'a> {
            fn with_normalized_info_fields(
                self,
                info: &Info,
                torrent_source_tag: Option<&'a str>,
            ) -> Self {
                self.set_private_flag(Some(true))
                    .set_piece_length(PieceLength::Custom(info.piece_length() as usize))
                    .set_source(torrent_source_tag)
            }
        }
    };
}

impl_normalized_info_fields!(InfoBuilder);
impl_normalized_info_fields!(MetainfoBuilder);

/// Computes the info hash of a torrent the way it is served on download:
/// the `private` flag is forced on and the configured source tag is injected
/// into the info dictionary.
pub fn compute_torrent_info_hash(
    info: &Info,
    torrent_source_tag: Option<&str>,
) -> Result<InfoHash> {
    let info_normalized = InfoBuilder::new()
        .with_normalized_info_fields(info, torrent_source_tag)
        .build(1, info, |_| {})
        .map_err(|_| Error::TorrentFileInvalid)?;

    Ok(InfoHash::from_bytes(&info_normalized))
}
