use crate::{
    connection_pool::ConnectionPool,
    models::{
        notification::NotificationReason,
        torrent::{
            EditedTorrent, Features, Torrent, TorrentMinimal, TorrentSearch, TorrentToDelete,
            UploadedTorrent,
        },
        user::User,
    },
    repositories::notification_repository::NotificationItemsIds,
};
use arcadia_common::{
    error::{Error, Result},
    services::torrent_service::{get_announce_url, looks_like_url},
};
use bip_metainfo::{Info, InfoBuilder, InfoHash, Metainfo, MetainfoBuilder, PieceLength};
use serde_json::{json, Value};
use sqlx::PgPool;
use std::{borrow::Borrow, str::FromStr};

#[derive(sqlx::FromRow)]
struct TitleGroupInfoLite {
    id: i64,
    #[allow(dead_code)]
    name: String,
}

pub struct GetTorrentResult {
    pub title: String,
    pub file_contents: Vec<u8>,
}

impl ConnectionPool {
    pub async fn create_torrent(
        &self,
        torrent_form: &UploadedTorrent,
        current_user: &User,
    ) -> Result<Torrent> {
        let mut tx = <ConnectionPool as Borrow<PgPool>>::borrow(self)
            .begin()
            .await?;

        let create_torrent_query = r#"
            INSERT INTO torrents (
                edition_group_id, created_by_id, release_name, release_group, description,
                file_amount_per_type, uploaded_as_anonymous, file_list, mediainfo, trumpable,
                staff_checked, size, duration, audio_codec, audio_bitrate, audio_bitrate_sampling,
                audio_channels, video_codec, features, subtitle_languages, video_resolution,
                video_resolution_other_x, video_resolution_other_y, container, languages, info_hash, info_dict, extras
            ) VALUES (
                $1, $2, $3, $4, $5, $6, $7,
                $8, $9, $10, $11, $12, $13,
                $14::audio_codec_enum, $15, $16::audio_bitrate_sampling_enum,
                $17::audio_channels_enum, $18::video_codec_enum, $19::features_enum[],
                $20::language_enum[], $21::video_resolution_enum, $22, $23, $24,
                $25::language_enum[], $26::bytea, $27::bytea, $28::extras_enum[]
            )
            RETURNING *
        "#;

        let metainfo = Metainfo::from_bytes(&torrent_form.torrent_file.data)
            .map_err(|_| Error::TorrentFileInvalid)?;

        let info = metainfo.info();

        // We cannot trust that the uploader has set the private field properly,
        // so we need to recreate the info db with it forced, which requires a
        // recomputation of info hash
        let info_normalized = InfoBuilder::new()
            .set_private_flag(Some(true))
            .set_piece_length(PieceLength::Custom(info.piece_length() as usize))
            .build(1, info, |_| {})
            .map_err(|_| Error::TorrentFileInvalid)?;

        let info_hash = InfoHash::from_bytes(&info_normalized);

        // TODO: torrent metadata extraction should be done on the client side
        let parent_folder = info.directory().map(|d| d.to_str().unwrap()).unwrap_or("");
        let files = info
            .files()
            .map(|f| json!({"name": f.path().to_str().unwrap(), "size": f.length()}))
            .collect::<Vec<_>>();

        let file_list = json!({"parent_folder": parent_folder, "files": files});

        let file_amount_per_type = json!(info
            .files()
            .flat_map(|file| file.path().to_str().unwrap().split('.').next_back())
            .fold(std::collections::HashMap::new(), |mut acc, ext| {
                *acc.entry(ext.to_string()).or_insert(0) += 1;
                acc
            }));

        // TODO: check if the torrent is trumpable: via a service ?
        let trumpable = String::from("");
        let size = metainfo
            .info()
            .files()
            .map(|file| file.length())
            .sum::<u64>() as i64;

        let uploaded_torrent = sqlx::query_as::<_, Torrent>(create_torrent_query)
            .bind(torrent_form.edition_group_id.0)
            .bind(current_user.id)
            .bind(&*torrent_form.release_name.0)
            .bind(torrent_form.release_group.as_deref())
            .bind(torrent_form.description.as_deref())
            .bind(&file_amount_per_type)
            .bind(torrent_form.uploaded_as_anonymous.0)
            .bind(&file_list)
            // set mediainfo to None if empty
            .bind(torrent_form.mediainfo.as_deref().and_then(|s| {
                if s.trim().is_empty() {
                    None
                } else {
                    Some(s.trim().to_string())
                }
            }))
            .bind(&trumpable)
            .bind(false)
            .bind(size)
            .bind(torrent_form.duration.as_deref())
            .bind(torrent_form.audio_codec.as_deref())
            .bind(torrent_form.audio_bitrate.as_deref())
            .bind(torrent_form.audio_bitrate_sampling.as_deref())
            .bind(torrent_form.audio_channels.as_deref())
            .bind(torrent_form.video_codec.as_deref())
            .bind(
                torrent_form
                    .features
                    .split(',')
                    .filter(|f| !f.is_empty())
                    .map(|f| Features::from_str(f).ok().unwrap())
                    .collect::<Vec<Features>>(),
            )
            .bind(
                torrent_form
                    .subtitle_languages
                    .0
                    .split(',')
                    .filter(|f| !f.is_empty())
                    .map(|f| f.trim())
                    .collect::<Vec<&str>>(),
            )
            .bind(torrent_form.video_resolution.as_deref())
            .bind(torrent_form.video_resolution_other_x.as_deref())
            .bind(torrent_form.video_resolution_other_y.as_deref())
            .bind(&*torrent_form.container.to_lowercase())
            .bind(
                torrent_form
                    .languages
                    .0
                    .split(',')
                    .filter(|f| !f.is_empty())
                    .map(|f| f.trim())
                    .collect::<Vec<&str>>(),
            )
            .bind(info_hash.as_ref())
            .bind(info.to_bytes())
            .bind(
                torrent_form
                    .extras
                    .split(',')
                    .filter(|f| !f.is_empty())
                    .map(|f| f.trim())
                    .collect::<Vec<&str>>(),
            )
            .fetch_one(&mut *tx)
            .await
            .map_err(Error::CouldNotCreateTorrent)?;

        let title_group_info = sqlx::query_as!(
            TitleGroupInfoLite,
            r#"
                SELECT title_groups.id, title_groups.name
                FROM edition_groups
                JOIN title_groups ON edition_groups.title_group_id = title_groups.id
                WHERE edition_groups.id = $1
            "#,
            torrent_form.edition_group_id.0
        )
        .fetch_one(&mut *tx)
        .await?;

        let _ = Self::notify_users(
            &mut tx,
            &NotificationReason::TorrentUploadedInSubscribedTitleGroup,
            None,
            NotificationItemsIds {
                title_group_id: Some(title_group_info.id),
                torrent_id: Some(uploaded_torrent.id),
                artist_id: None,
                collage_id: None,
                forum_thread_id: None,
            },
        )
        .await;

        tx.commit().await?;

        Ok(uploaded_torrent)
    }

    pub async fn find_torrent(&self, torrent_id: i64) -> Result<Torrent> {
        let torrent = sqlx::query_as!(
            Torrent,
            r#"
            SELECT
                id, upload_factor, download_factor, seeders, leechers,
                completed, snatched, edition_group_id, created_at, updated_at,
                created_by_id,
                extras AS "extras!: _",
                languages AS "languages!: _",
                release_name, release_group, description, file_amount_per_type,
                uploaded_as_anonymous, file_list, mediainfo, trumpable, staff_checked,
                container, size, duration,
                audio_codec AS "audio_codec: _",
                audio_bitrate,
                audio_bitrate_sampling AS "audio_bitrate_sampling: _",
                audio_channels AS "audio_channels: _",
                video_codec AS "video_codec: _",
                features AS "features!: _",
                subtitle_languages AS "subtitle_languages!: _",
                video_resolution AS "video_resolution!: _",
                video_resolution_other_x,
                video_resolution_other_y
            FROM torrents
            WHERE id = $1
            "#,
            torrent_id
        )
        .fetch_one(self.borrow())
        .await
        .map_err(|_| Error::TorrentNotFound)?;

        Ok(torrent)
    }

    pub async fn update_torrent(
        &self,
        edited_torrent: &EditedTorrent,
        torrent_id: i64,
    ) -> Result<Torrent> {
        let updated_torrent = sqlx::query_as!(
            Torrent,
            r#"
            UPDATE torrents
            SET
                release_name = $2,
                release_group = $3,
                description = $4,
                uploaded_as_anonymous = $5,
                mediainfo = $6,
                container = $7,
                duration = $8,
                audio_codec = $9,
                audio_bitrate = $10,
                audio_bitrate_sampling = $11,
                audio_channels = $12,
                video_codec = $13,
                features = $14,
                subtitle_languages = $15,
                video_resolution = $16,
                video_resolution_other_x = $17,
                video_resolution_other_y = $18,
                languages = $19,
                extras = $20,
                updated_at = NOW()
            WHERE id = $1
            RETURNING
                id, upload_factor, download_factor, seeders, leechers,
                completed, snatched, edition_group_id, created_at, updated_at,
                created_by_id,
                extras AS "extras!: _",
                languages AS "languages!: _",
                release_name, release_group, description, file_amount_per_type,
                uploaded_as_anonymous, file_list, mediainfo, trumpable, staff_checked,
                container, size, duration,
                audio_codec AS "audio_codec: _",
                audio_bitrate,
                audio_bitrate_sampling AS "audio_bitrate_sampling: _",
                audio_channels AS "audio_channels: _",
                video_codec AS "video_codec: _",
                features AS "features!: _",
                subtitle_languages AS "subtitle_languages!: _",
                video_resolution AS "video_resolution!: _",
                video_resolution_other_x,
                video_resolution_other_y
            "#,
            torrent_id,
            edited_torrent.release_name,
            edited_torrent.release_group,
            edited_torrent.description,
            edited_torrent.uploaded_as_anonymous,
            edited_torrent.mediainfo,
            edited_torrent.container,
            edited_torrent.duration,
            edited_torrent.audio_codec as _,
            edited_torrent.audio_bitrate,
            edited_torrent.audio_bitrate_sampling as _,
            edited_torrent.audio_channels as _,
            edited_torrent.video_codec as _,
            edited_torrent.features as _,
            edited_torrent.subtitle_languages as _,
            edited_torrent.video_resolution as _,
            edited_torrent.video_resolution_other_x,
            edited_torrent.video_resolution_other_y,
            edited_torrent.languages as _,
            edited_torrent.extras as _
        )
        .fetch_one(self.borrow())
        .await
        .map_err(|e| Error::ErrorWhileUpdatingTorrent(e.to_string()))?;

        Ok(updated_torrent)
    }

    pub async fn get_torrent(
        &self,
        user: &User,
        torrent_id: i64,
        tracker_name: &str,
        frontend_url: &str,
        tracker_url: &str,
    ) -> Result<GetTorrentResult> {
        let mut tx = <ConnectionPool as Borrow<PgPool>>::borrow(self)
            .begin()
            .await?;

        let torrent = sqlx::query!(
            r#"
            UPDATE torrents
            SET snatched = snatched + 1
            WHERE id = $1
            RETURNING
                info_dict,
                EXTRACT(EPOCH FROM created_at)::BIGINT AS "created_at_secs!",
                release_name;
            "#,
            torrent_id
        )
        .fetch_one(&mut *tx)
        .await
        .map_err(|_| Error::TorrentFileInvalid)?;

        let info = Info::from_bytes(torrent.info_dict).map_err(|_| Error::TorrentFileInvalid)?;

        let announce_url = get_announce_url(user.passkey_upper, user.passkey_lower, tracker_url);

        let frontend_url = format!("{frontend_url}torrent/{torrent_id}");

        let metainfo = MetainfoBuilder::new()
            .set_main_tracker(Some(&announce_url))
            .set_creation_date(Some(torrent.created_at_secs))
            .set_comment(Some(&frontend_url))
            .set_created_by(Some(tracker_name))
            .set_piece_length(PieceLength::Custom(info.piece_length() as usize))
            .set_private_flag(Some(true))
            .build(1, &info, |_| {})
            .map_err(|_| Error::TorrentFileInvalid)?;

        let _ = sqlx::query!(
            r#"
                INSERT INTO torrent_activities(torrent_id, user_id, snatched_at)
                VALUES ($1, $2, NOW())
                ON CONFLICT (torrent_id, user_id) DO NOTHING;
                "#,
            torrent_id,
            user.id,
        )
        .execute(&mut *tx)
        .await
        .map_err(|_| Error::InvalidUserIdOrTorrentId);

        tx.commit().await?;

        Ok(GetTorrentResult {
            title: torrent.release_name,
            file_contents: metainfo,
        })
    }

    pub async fn search_torrents(
        &self,
        torrent_search: &TorrentSearch,
        requesting_user_id: Option<i64>,
    ) -> Result<Value> {
        let input = torrent_search.title_group.name.trim();

        let (name, external_link) = if looks_like_url(input) {
            (None, Some(input.to_string()))
        } else if input.trim().is_empty() {
            (None, None)
        } else {
            (Some(input.to_string()), None)
        };

        let search_results = sqlx::query!(
            r#"
            WITH title_group_data AS (
                SELECT
                    tgl.title_group_data AS lite_title_group
                FROM get_title_groups_and_edition_group_and_torrents_lite($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12) tgl
            )
            SELECT jsonb_agg(lite_title_group) AS title_groups
            FROM title_group_data;
            "#,
            name,
            torrent_search.torrent.staff_checked,
            torrent_search.torrent.reported,
            torrent_search.title_group.include_empty_groups,
            torrent_search.sort_by.to_string(),
            torrent_search.order.to_string(),
            torrent_search.page_size,
            (torrent_search.page - 1) * torrent_search.page_size,
            torrent_search.torrent.created_by_id,
            torrent_search.torrent.snatched_by_id,
            requesting_user_id,
            external_link
        )
        .fetch_one(self.borrow())
        .await
        .map_err(|error| Error::ErrorSearchingForTorrents(error.to_string()))?;

        Ok(serde_json::json!({"title_groups": search_results.title_groups}))
    }

    pub async fn find_top_torrents(&self, period: &str, amount: i64) -> Result<Value> {
        let search_results = sqlx::query!(
            r#"
            WITH title_group_search AS (
                ---------- This is the part that selects the top torrents
                SELECT DISTINCT ON (tg.id) tg.id AS title_group_id
                FROM torrents t
                JOIN torrent_activities st ON t.id = st.torrent_id
                JOIN edition_groups eg ON t.edition_group_id = eg.id
                JOIN title_groups tg ON eg.title_group_id = tg.id
                WHERE CASE
                    WHEN $1 = 'all time' THEN TRUE
                    ELSE t.created_at >= NOW() - CAST($1 AS INTERVAL)
                END
                GROUP BY tg.id, tg.name
                ORDER BY tg.id, COUNT(st.torrent_id) DESC
                LIMIT $2
                ----------
            ),
            title_group_data AS (
                SELECT
                    tgl.title_group_data AS lite_title_group -- 'affiliated_artists' is already inside tgl.title_group_data
                FROM get_title_groups_and_edition_group_and_torrents_lite('', NULL, NULL, TRUE, 'title_group_original_release_date', 'desc', NULL, NULL, NULL, NULL, NULL) tgl
                JOIN title_groups tg ON tgl.title_group_id = tg.id
                JOIN title_group_search tgs ON tg.id = tgs.title_group_id
            )
            SELECT jsonb_agg(lite_title_group) AS title_groups
            FROM title_group_data;
            "#,
            period,
            amount
        )
        .fetch_one(self.borrow())
        .await
        .map_err(|error| Error::ErrorSearchingForTorrents(error.to_string()))?;

        Ok(serde_json::json!({"title_groups": search_results.title_groups}))
    }

    pub async fn remove_torrent(
        &self,
        torrent_to_delete: &TorrentToDelete,
        current_user_id: i64,
    ) -> Result<()> {
        let mut tx = <ConnectionPool as Borrow<PgPool>>::borrow(self)
            .begin()
            .await?;

        Self::notify_users(
            &mut tx,
            &NotificationReason::SeedingTorrentDeleted,
            Some(torrent_to_delete.displayed_reason.as_ref().unwrap()),
            NotificationItemsIds {
                title_group_id: None,
                torrent_id: Some(torrent_to_delete.id),
                artist_id: None,
                collage_id: None,
                forum_thread_id: None,
            },
        )
        .await?;

        sqlx::query!(
            r#"
            INSERT INTO deleted_torrents (SELECT *, NOW() AS deleted_at, $1 AS deleted_by_id, $2 AS reason FROM torrents WHERE id = $3);
            "#,
            current_user_id,
            torrent_to_delete.reason,
            torrent_to_delete.id
        )
        .execute(&mut *tx)
        .await
        .map_err(|error| Error::ErrorDeletingTorrent(error.to_string()))?;

        sqlx::query!(
            r#"
            DELETE FROM torrents WHERE id = $1;
            "#,
            torrent_to_delete.id
        )
        .execute(&mut *tx)
        .await
        .map_err(|error| Error::ErrorDeletingTorrent(error.to_string()))?;

        tx.commit().await?;

        Ok(())
    }

    pub async fn update_torrent_seeders_leechers(&self) -> Result<()> {
        let _ = sqlx::query!(
            r#"
            WITH peer_counts AS (
                SELECT
                    torrent_id,
                    COUNT(CASE WHEN status = 'seeding' THEN 1 END) AS current_seeders,
                    COUNT(CASE WHEN status = 'leeching' THEN 1 END) AS current_leechers
                FROM
                    peers
                GROUP BY
                    torrent_id
            )
            UPDATE torrents AS t
            SET
                seeders = COALESCE(pc.current_seeders, 0),
                leechers = COALESCE(pc.current_leechers, 0)
            FROM
                torrents AS t_alias -- Use an alias for the table in the FROM clause to avoid ambiguity
            LEFT JOIN
                peer_counts AS pc ON t_alias.id = pc.torrent_id
            WHERE
                t.id = t_alias.id;
            "#
        )
        .execute(self.borrow())
        .await?;

        Ok(())
    }

    pub async fn increment_torrent_completed(&self, torrent_id: i64) -> Result<()> {
        let _ = sqlx::query!(
            r#"
            UPDATE torrents
            SET
                completed = completed + 1
            WHERE
                id = $1
            "#,
            torrent_id
        )
        .execute(self.borrow())
        .await?;

        Ok(())
    }

    pub async fn find_registered_torrents(&self) -> Result<Vec<TorrentMinimal>> {
        let torrents = sqlx::query_as!(
            TorrentMinimal,
            r#"
            SELECT id, created_at, ENCODE(info_hash, 'hex') as info_hash FROM torrents
            "#
        )
        .fetch_all(self.borrow())
        .await?;

        Ok(torrents)
    }
}
