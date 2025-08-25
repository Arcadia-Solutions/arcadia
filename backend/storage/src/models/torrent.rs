use std::str::FromStr;

use actix_multipart::form::{bytes::Bytes, text::Text, MultipartForm};
use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sqlx::{prelude::FromRow, types::Json};
use strum::Display;
use utoipa::ToSchema;

use super::{title_group::TitleGroupHierarchyLite, torrent_report::TorrentReport, user::UserLite};

#[derive(Debug, Deserialize, Serialize, sqlx::Type, ToSchema)]
#[sqlx(type_name = "audio_codec_enum")]
pub enum AudioCodec {
    #[sqlx(rename = "mp2")]
    #[serde(rename = "mp2")]
    Mp2,
    #[sqlx(rename = "mp3")]
    #[serde(rename = "mp3")]
    Mp3,
    #[sqlx(rename = "aac")]
    #[serde(rename = "aac")]
    Aac,
    #[sqlx(rename = "ac3")]
    #[serde(rename = "ac3")]
    Ac3,
    #[sqlx(rename = "dts")]
    #[serde(rename = "dts")]
    Dts,
    #[sqlx(rename = "flac")]
    #[serde(rename = "flac")]
    Flac,
    #[sqlx(rename = "pcm")]
    #[serde(rename = "pcm")]
    Pcm,
    #[sqlx(rename = "true-hd")]
    #[serde(rename = "true-hd")]
    TrueHd,
    #[sqlx(rename = "opus")]
    #[serde(rename = "opus")]
    Opus,
    #[sqlx(rename = "dsd")]
    #[serde(rename = "dsd")]
    Dsd,
}

#[derive(Debug, Deserialize, Serialize, sqlx::Type, ToSchema)]
#[sqlx(type_name = "audio_channels_enum")]
pub enum AudioChannels {
    #[sqlx(rename = "1.0")]
    #[serde(rename = "1.0")]
    OneDotZero,
    #[sqlx(rename = "2.0")]
    #[serde(rename = "2.0")]
    TwoDotZero,
    #[sqlx(rename = "2.1")]
    #[serde(rename = "2.1")]
    TwoDotOne,
    #[sqlx(rename = "5.0")]
    #[serde(rename = "5.0")]
    FiveDotZero,
    #[sqlx(rename = "5.1")]
    #[serde(rename = "5.1")]
    FiveDotOne,
    #[sqlx(rename = "7.1")]
    #[serde(rename = "7.1")]
    SevenDotOne,
}

#[derive(Debug, Deserialize, Serialize, sqlx::Type, ToSchema)]
#[sqlx(type_name = "audio_bitrate_sampling_enum")]
pub enum AudioBitrateSampling {
    #[sqlx(rename = "64")]
    #[serde(rename = "64")]
    Bitrate64,
    #[sqlx(rename = "128")]
    #[serde(rename = "128")]
    Bitrate128,
    #[sqlx(rename = "192")]
    #[serde(rename = "192")]
    Bitrate192,
    #[sqlx(rename = "256")]
    #[serde(rename = "256")]
    Bitrate256,
    #[sqlx(rename = "320")]
    #[serde(rename = "320")]
    Bitrate320,
    #[sqlx(rename = "APS (VBR)")]
    #[serde(rename = "APS (VBR)")]
    ApsVbr,
    #[sqlx(rename = "V2 (VBR)")]
    #[serde(rename = "V2 (VBR)")]
    V2Vbr,
    #[sqlx(rename = "V1 (VBR)")]
    #[serde(rename = "V1 (VBR)")]
    V1Vbr,
    #[sqlx(rename = "APX (VBR)")]
    #[serde(rename = "APX (VBR)")]
    ApxVbr,
    #[sqlx(rename = "V0 (VBR)")]
    #[serde(rename = "V0 (VBR)")]
    V0Vbr,
    Lossless,
    #[sqlx(rename = "24bit Lossless")]
    #[serde(rename = "24bit Lossless")]
    Lossless24Bit,
    #[sqlx(rename = "DSD64")]
    #[serde(rename = "DSD64")]
    Dsd64,
    #[sqlx(rename = "DSD128")]
    #[serde(rename = "DSD128")]
    Dsd128,
    #[sqlx(rename = "DSD256")]
    #[serde(rename = "DSD256")]
    Dsd256,
    #[sqlx(rename = "DSD512")]
    #[serde(rename = "DSD512")]
    Dsd512,
    Other,
}

#[derive(Debug, Deserialize, Serialize, sqlx::Type, ToSchema)]
#[sqlx(type_name = "video_codec_enum")]
pub enum VideoCodec {
    #[sqlx(rename = "mpeg1")]
    #[serde(rename = "mpeg1")]
    Mpeg1,
    #[sqlx(rename = "mpeg2")]
    #[serde(rename = "mpeg2")]
    Mpeg2,
    Xvid,
    #[sqlx(rename = "divX")]
    #[serde(rename = "divX")]
    DivX,
    #[sqlx(rename = "h264")]
    #[serde(rename = "h264")]
    H264,
    #[sqlx(rename = "h265")]
    #[serde(rename = "h265")]
    H265,
    #[sqlx(rename = "vc-1")]
    #[serde(rename = "vc-1")]
    Vc1,
    #[sqlx(rename = "vp9")]
    #[serde(rename = "vp9")]
    Vp9,
    BD50,
    UHD100,
}

#[derive(Debug, Deserialize, Serialize, sqlx::Type, ToSchema)]
#[sqlx(type_name = "language_enum")]
pub enum Language {
    English,
    French,
    German,
    Italian,
    Spanish,
    Swedish,
}

#[derive(Debug, Deserialize, Serialize, sqlx::Type, ToSchema)]
#[sqlx(type_name = "video_resolution_enum")]
pub enum VideoResolution {
    Other,
    #[sqlx(rename = "480p")]
    #[serde(rename = "480p")]
    P480,
    #[sqlx(rename = "480i")]
    #[serde(rename = "480i")]
    I480,
    #[sqlx(rename = "576i")]
    #[serde(rename = "576i")]
    I576,
    #[sqlx(rename = "576p")]
    #[serde(rename = "576p")]
    P576,
    #[sqlx(rename = "720p")]
    #[serde(rename = "720p")]
    P720,
    #[sqlx(rename = "1080p")]
    #[serde(rename = "1080p")]
    P1080,
    #[sqlx(rename = "1080i")]
    #[serde(rename = "1080i")]
    I1080,
    #[sqlx(rename = "1440p")]
    #[serde(rename = "1440p")]
    P1440,
    #[sqlx(rename = "2160p")]
    #[serde(rename = "2160p")]
    P2160,
    #[sqlx(rename = "4320p")]
    #[serde(rename = "4320p")]
    P4320,
}

#[derive(Debug, Deserialize, Serialize, sqlx::Type, ToSchema)]
#[sqlx(type_name = "features_enum")]
pub enum Features {
    #[sqlx(rename = "HDR")]
    #[serde(rename = "HDR")]
    Hdr,
    #[sqlx(rename = "HDR 10")]
    #[serde(rename = "HDR 10")]
    HdrTen,
    #[sqlx(rename = "HDR 10+")]
    #[serde(rename = "HDR 10+")]
    HdrTenPlus,
    #[sqlx(rename = "DV")]
    #[serde(rename = "DV")]
    Dv,
    Commentary,
    Remux,
    #[sqlx(rename = "3D")]
    #[serde(rename = "3D")]
    ThreeD,
    #[sqlx(rename = "OCR")]
    #[serde(rename = "OCR")]
    Ocr,
    Cue,
}

#[derive(Debug, Deserialize, Serialize, sqlx::Type, ToSchema)]
#[sqlx(type_name = "extras_enum", rename_all = "lowercase")]
#[serde(rename_all = "lowercase")]
pub enum Extras {
    Booklet,
    Manual,
    #[sqlx(rename = "behind_the_scenes")]
    #[serde(rename = "behind_the_scenes")]
    BehindTheScenes,
    #[sqlx(rename = "deleted_scenes")]
    #[serde(rename = "deleted_scenes")]
    DeletedScenes,
    Featurette,
    Trailer,
    Other,
}

// TODO: this should not be necessary with annontations, but there is somehow an error
impl FromStr for Features {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "HDR" => Ok(Features::Hdr),
            "HDR 10" => Ok(Features::HdrTen),
            "HDR 10+" => Ok(Features::HdrTenPlus),
            "DV" => Ok(Features::Dv),
            "Commentary" => Ok(Features::Commentary),
            "Remux" => Ok(Features::Remux),
            "3D" => Ok(Features::ThreeD),
            "Cue" => Ok(Features::Cue),
            _ => Err(()),
        }
    }
}

#[derive(Debug, Serialize, FromRow, ToSchema)]
pub struct Torrent {
    pub id: i64,
    pub upload_factor: f64,
    pub download_factor: f64,
    pub seeders: i64,
    pub leechers: i64,
    pub completed: i64,
    pub snatched: i64,
    pub edition_group_id: i64,
    #[schema(value_type = String, format = DateTime)]
    pub created_at: DateTime<Local>,
    #[schema(value_type = String, format = DateTime)]
    pub updated_at: DateTime<Local>,
    pub created_by_id: i64,
    pub extras: Vec<Extras>,
    pub release_name: Option<String>,
    pub release_group: Option<String>,
    pub description: Option<String>, // specific to the torrent
    #[schema(value_type = HashMap<String, String>)]
    pub file_amount_per_type: Json<Value>, // (5 mp3, 1 log, 5 jpg, etc.)
    pub uploaded_as_anonymous: bool,
    #[schema(value_type = HashMap<String, String>)]
    pub file_list: Json<Value>,
    pub mediainfo: Option<String>,
    pub trumpable: Option<String>, // description of why it is trumpable
    pub staff_checked: bool,
    pub languages: Vec<Language>, // (fallback to original language) (english, french, etc.)
    pub container: String, // container of the main file (ex: if mkv movie and srt subs, mkv is the main)
    pub size: i64,         // in bytes
    // ---- audio
    pub duration: Option<i32>, // in seconds
    pub audio_codec: Option<AudioCodec>,
    pub audio_bitrate: Option<i32>, // in kb/s
    pub audio_bitrate_sampling: Option<AudioBitrateSampling>,
    pub audio_channels: Option<AudioChannels>,
    // ---- audio
    // ---- video
    pub video_codec: Option<VideoCodec>,
    pub features: Option<Vec<Features>>,
    pub subtitle_languages: Vec<Language>,
    pub video_resolution: Option<VideoResolution>, // ---- video
    pub video_resolution_other_x: Option<i32>,
    pub video_resolution_other_y: Option<i32>,
}

#[derive(Debug, MultipartForm, FromRow, ToSchema)]
pub struct UploadedTorrent {
    #[schema(value_type = String)]
    pub extras: Text<String>,
    #[schema(value_type = String)]
    pub release_name: Text<String>,
    #[schema(value_type = String)]
    pub release_group: Option<Text<String>>,
    #[schema(value_type = String)]
    pub description: Option<Text<String>>,
    #[schema(value_type = bool)]
    pub uploaded_as_anonymous: Text<bool>,
    #[schema(value_type = String)]
    pub mediainfo: Option<Text<String>>,
    #[schema(value_type = String, format = Binary, content_media_type = "application/octet-stream")]
    pub torrent_file: Bytes,
    #[schema(value_type = String)]
    pub languages: Text<String>,
    #[schema(value_type = String)]
    pub container: Text<String>,
    #[schema(value_type = i64)]
    pub edition_group_id: Text<i64>,
    #[schema(value_type = i32)]
    pub duration: Option<Text<i32>>,
    #[schema(value_type = AudioCodec)]
    pub audio_codec: Option<Text<AudioCodec>>,
    #[schema(value_type = i32)]
    pub audio_bitrate: Option<Text<i32>>,
    #[schema(value_type = String)]
    pub audio_channels: Option<Text<AudioChannels>>,
    #[schema(value_type = AudioBitrateSampling)]
    pub audio_bitrate_sampling: Option<Text<AudioBitrateSampling>>,
    #[schema(value_type = VideoCodec)]
    pub video_codec: Option<Text<VideoCodec>>,
    #[schema(value_type = String)]
    pub features: Text<String>,
    #[schema(value_type = String)]
    pub subtitle_languages: Text<String>,
    #[schema(value_type = VideoResolution)]
    pub video_resolution: Option<Text<VideoResolution>>,
    #[schema(value_type = i32)]
    pub video_resolution_other_x: Option<Text<i32>>,
    #[schema(value_type = i32)]
    pub video_resolution_other_y: Option<Text<i32>>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct EditedTorrent {
    pub id: i64,
    pub edition_group_id: i64,
    pub extras: Vec<Extras>,
    pub release_name: Option<String>,
    pub release_group: Option<String>,
    pub description: Option<String>,
    pub uploaded_as_anonymous: bool,
    pub mediainfo: Option<String>,
    pub container: String,
    pub languages: Vec<Language>,
    pub duration: Option<i32>,
    pub audio_codec: Option<AudioCodec>,
    pub audio_bitrate: Option<i32>,
    pub audio_bitrate_sampling: Option<AudioBitrateSampling>,
    pub audio_channels: Option<AudioChannels>,
    pub video_codec: Option<VideoCodec>,
    pub features: Option<Vec<Features>>,
    pub subtitle_languages: Vec<Language>,
    pub video_resolution: Option<VideoResolution>,
    pub video_resolution_other_x: Option<i32>,
    pub video_resolution_other_y: Option<i32>,
}

#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct TorrentSearchTitleGroup {
    pub name: String,
    pub include_empty_groups: bool,
}

#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct TorrentSearchTorrent {
    pub reported: Option<bool>,
    pub staff_checked: Option<bool>,
    pub created_by_id: Option<i64>,
    pub snatched_by_id: Option<i64>,
}

#[derive(Debug, Deserialize, Serialize, ToSchema, Display)]
pub enum TorrentSearchSortField {
    #[serde(rename = "torrent_created_at")]
    #[strum(serialize = "torrent_created_at")]
    TorrentCreatedAt,
    #[serde(rename = "torrent_size")]
    #[strum(serialize = "torrent_size")]
    TorrentSize,
    #[serde(rename = "torrent_snatched_at")]
    #[strum(serialize = "torrent_snatched_at")]
    TorrentSnatchedAt,
    #[serde(rename = "title_group_original_release_date")]
    #[strum(serialize = "title_group_original_release_date")]
    TitleGroupOriginalReleaseDate,
}

#[derive(Debug, Deserialize, Serialize, ToSchema, Display)]
pub enum TorrentSearchOrder {
    #[serde(rename = "asc")]
    #[strum(serialize = "asc")]
    Asc,
    #[serde(rename = "desc")]
    #[strum(serialize = "desc")]
    Desc,
}

#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct TorrentSearch {
    pub title_group: TorrentSearchTitleGroup,
    pub torrent: TorrentSearchTorrent,
    pub page: i64,
    pub page_size: i64,
    pub sort_by: TorrentSearchSortField,
    pub order: TorrentSearchOrder,
}

#[derive(Debug, Serialize, Deserialize, FromRow, ToSchema)]
pub struct TorrentHierarchyLite {
    pub id: i64,
    pub upload_factor: f64,
    pub download_factor: f64,
    pub seeders: i64,
    pub leechers: i64,
    pub completed: i64,
    pub snatched: i64,
    pub edition_group_id: i64,
    #[schema(value_type = String, format = DateTime)]
    pub created_at: DateTime<Local>,
    pub extras: Vec<Extras>,
    pub release_name: Option<String>,
    pub release_group: Option<String>,
    #[schema(value_type = HashMap<String, String>)]
    pub file_amount_per_type: Json<Value>,
    pub trumpable: Option<String>,
    pub staff_checked: bool,
    pub languages: Vec<Language>,
    pub container: String,
    pub size: i64,
    pub duration: Option<i32>,
    pub audio_codec: Option<AudioCodec>,
    pub audio_bitrate: Option<i32>,
    pub audio_bitrate_sampling: Option<AudioBitrateSampling>,
    pub audio_channels: Option<String>,
    pub video_codec: Option<VideoCodec>,
    pub features: Option<Vec<Features>>,
    pub subtitle_languages: Vec<Language>,
    pub video_resolution: Option<VideoResolution>,
    pub video_resolution_other_x: Option<i32>,
    pub video_resolution_other_y: Option<i32>,
    pub reports: Vec<TorrentReport>,
    // pub peer_status: Option<TorrentStatus>,
}

// #[derive(Debug, Deserialize, Serialize, ToSchema, Display)]
// pub enum TorrentStatus {
//     #[serde(rename = "seeding")]
//     Seeding,
//     #[serde(rename = "leeching")]
//     Leeching,
//     #[serde(rename = "snatched")]
//     Snatched,
// }

#[derive(Debug, Serialize, Deserialize, FromRow, ToSchema)]
pub struct TorrentHierarchy {
    pub id: i64,
    pub upload_factor: f64,
    pub download_factor: f64,
    pub seeders: i64,
    pub leechers: i64,
    pub completed: i64,
    pub snatched: i64,
    pub edition_group_id: i64,
    #[schema(value_type = String, format = DateTime)]
    pub created_at: DateTime<Local>,
    #[schema(value_type = String, format = DateTime)]
    pub updated_at: DateTime<Local>,
    pub created_by_id: Option<i64>,
    pub created_by: Option<UserLite>,
    pub extras: Vec<Extras>,
    pub release_name: Option<String>,
    pub release_group: Option<String>,
    pub description: Option<String>,
    #[schema(value_type = HashMap<String, String>)]
    pub file_amount_per_type: Json<Value>,
    pub uploaded_as_anonymous: bool,
    #[schema(value_type = HashMap<String, String>)]
    pub file_list: Json<Value>,
    pub mediainfo: Option<String>,
    pub trumpable: Option<String>,
    pub staff_checked: bool,
    pub languages: Vec<Language>,
    pub container: String,
    pub size: i64,
    pub duration: Option<i32>,
    pub audio_codec: Option<AudioCodec>,
    pub audio_bitrate: Option<i32>,
    pub audio_bitrate_sampling: Option<AudioBitrateSampling>,
    pub audio_channels: Option<AudioChannels>,
    pub video_codec: Option<VideoCodec>,
    pub features: Option<Vec<Features>>,
    pub subtitle_languages: Vec<Language>,
    pub video_resolution: Option<VideoResolution>,
    pub video_resolution_other_x: Option<i32>,
    pub video_resolution_other_y: Option<i32>,
    pub uploader: UserLite,
    pub reports: Vec<TorrentReport>,
    // pub peer_status: Option<TorrentStatus>,
}

#[derive(Debug, Serialize, Deserialize, FromRow, ToSchema)]
pub struct TorrentSearchResults {
    pub title_groups: Vec<TitleGroupHierarchyLite>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct TorrentToDelete {
    pub id: i64,
    pub reason: String,
    pub displayed_reason: Option<String>,
}

#[derive(Debug, Serialize, FromRow, ToSchema)]
pub struct TorrentMinimal {
    pub id: i64,
    #[schema(value_type = String, format = DateTime)]
    pub created_at: DateTime<Local>,
    // TODO: remove Option<>, this should never be null, but without it, the deserialization with sqlx fails somehow
    pub info_hash: Option<String>,
}
