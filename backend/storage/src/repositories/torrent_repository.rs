use crate::{
    connection_pool::ConnectionPool,
    models::{
        common::PaginatedResults,
        edition_group::EditionGroupHierarchyLite,
        notification::NotificationReason,
        title_group::TitleGroupHierarchyLite,
        torrent::{
            EditedTorrent, Features, Torrent, TorrentHierarchyLite, TorrentMinimal, TorrentSearch,
            TorrentToDelete, UploadedTorrent,
        },
    },
    repositories::notification_repository::NotificationItemsIds,
};
use arcadia_common::{
    error::{Error, Result},
    services::torrent_service::{get_announce_url, looks_like_url},
};
use arcadia_shared::tracker::models::torrent::InfoHash;
use bip_metainfo::{Info, InfoBuilder, Metainfo, MetainfoBuilder, PieceLength};
use serde_json::{json, Value};
use sqlx::{types::Json, PgPool};
use std::{borrow::Borrow, collections::HashMap, str::FromStr};

#[derive(sqlx::FromRow)]
struct TitleGroupInfoLite {
    id: i32,
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
        user_id: i32,
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

        let info_hash = bip_metainfo::InfoHash::from_bytes(&info_normalized);

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
            .bind(user_id)
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

    pub async fn find_torrent(&self, torrent_id: i32) -> Result<Torrent> {
        let torrent = sqlx::query_as!(
            Torrent,
            r#"
            SELECT
                id, info_hash as "info_hash: InfoHash", upload_factor, download_factor, seeders, leechers,
                times_completed, snatched, edition_group_id, created_at, updated_at,
                created_by_id,
                deleted_at AS "deleted_at!: _",
                deleted_by_id AS "deleted_by_id!: _",
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
            WHERE id = $1 AND deleted_at is NULL
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
        torrent_id: i32,
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
            WHERE id = $1 AND deleted_at IS NULL
            RETURNING
                id, info_hash as "info_hash: InfoHash", upload_factor, download_factor, seeders, leechers,
                times_completed, snatched, edition_group_id, created_at, updated_at,
                created_by_id,
                deleted_at AS "deleted_at!: _",
                deleted_by_id AS "deleted_by_id!: _",
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
        user_id: i32,
        torrent_id: i32,
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
            WHERE id = $1 AND deleted_at IS NULL
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

        let user = self.find_user_with_id(user_id).await?;
        let announce_url = get_announce_url(user.passkey, tracker_url);

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
        form: &TorrentSearch,
        requesting_user_id: Option<i32>,
    ) -> Result<PaginatedResults<TitleGroupHierarchyLite>> {
        // TODO: the torrent activities table is not populated by the tracker yet
        // once this is done, we can join on this table to get the snatched torrents
        // for a given user
        if form.torrent_snatched_by_id.is_some() {
            return Ok(PaginatedResults {
                results: Vec::new(),
                total_items: 0,
                page: 0,
                page_size: 0,
            });
        }

        let (name_filter, external_link_filter) = match &form.title_group_name {
            Some(s) => {
                let input = s.trim();
                if input.is_empty() {
                    (None, None)
                } else if looks_like_url(input) {
                    (None, Some(input.to_string()))
                } else {
                    (Some(input.to_string()), None)
                }
            }
            None => (None, None),
        };

        let limit = form.page * form.page_size;
        let offset = (form.page - 1) * form.page_size;

        // we apply filters on 3 tables: title_groups, edition_groups, and torrents

        // first: get title groups that have editions and torrents (and the title groups themselves)
        // matching the filters on the 3 tables right away, thanks to the materialized view

        let title_groups = sqlx::query_as!(
            TitleGroupHierarchyLite,
            r#"
             SELECT title_group_id AS "id!", title_group_name AS "name!", title_group_covers AS "covers!",
             title_group_category AS "category!: _", title_group_content_type AS "content_type!: _", title_group_tags AS "tags!",
             title_group_original_release_date AS "original_release_date!", title_group_platform AS "platform!: _",
             '[]'::jsonb AS "edition_groups!: _",
             '[]'::jsonb AS "affiliated_artists!: _"

             FROM title_group_hierarchy_lite tgh

             WHERE ($4::BOOLEAN IS NULL OR tgh.torrent_staff_checked = $4)
             AND ($5::BOOLEAN IS NULL OR tgh.torrent_reported = $5)
             AND (
                $7::INT IS NULL OR
                -- don't return torrents created as anonymous
                -- unless the requesting user is the uploader
                (tgh.torrent_created_by_id = $7 AND (
                   tgh.torrent_created_by_id = $8 OR
                   NOT tgh.torrent_uploaded_as_anonymous)
                )
            )
            AND (
                $9::BIGINT IS NULL OR
                EXISTS (SELECT 1 FROM affiliated_artists aa WHERE aa.title_group_id = tgh.title_group_id AND aa.artist_id = $9)
            )
            -- name filter (partial match) or external link match or series name match
            AND (
                $10::TEXT IS NULL OR
                    tgh.title_group_name ILIKE '%' || $10 || '%' OR
                    tgh.title_group_series_name ILIKE '%' || $10 || '%'
            )
            AND ($11::TEXT IS NULL OR $11 = ANY(tgh.title_group_external_links))

            GROUP BY title_group_id, title_group_name, title_group_covers, title_group_category,
            title_group_content_type, title_group_tags, title_group_original_release_date, title_group_platform

            ORDER BY
                CASE WHEN $1 = 'title_group_original_release_date' AND $6 = 'asc' THEN title_group_original_release_date END ASC,
                CASE WHEN $1 = 'title_group_original_release_date' AND $6 = 'desc' THEN title_group_original_release_date END DESC,
                CASE WHEN $1 = 'torrent_size' AND $6 = 'asc' THEN MAX(torrent_size) END ASC,
                CASE WHEN $1 = 'torrent_size' AND $6 = 'desc' THEN MAX(torrent_size) END DESC,
                CASE WHEN $1 = 'torrent_created_at' AND $6 = 'asc' THEN MAX(torrent_created_at) END ASC,
                CASE WHEN $1 = 'torrent_created_at' AND $6 = 'desc' THEN MAX(torrent_created_at) END DESC,
                title_group_original_release_date ASC
                
            LIMIT $2 OFFSET $3
            "#,
            form.order_by_column.to_string(),
            limit,
            offset,
            form.torrent_staff_checked,
            form.torrent_reported,
            form.order_by_direction.to_string(),
            form.torrent_created_by_id,
            requesting_user_id,
            form.artist_id,
            name_filter,
            external_link_filter,
        )
        .fetch_all(self.borrow())
        .await
        .map_err(|error| Error::ErrorSearchingForTorrents(error.to_string()))?;

        // amount of results for pagination
        let total_title_groups_count = sqlx::query_scalar!(
            r#"
            SELECT COUNT(DISTINCT title_group_id)
            FROM title_group_hierarchy_lite tgh
            WHERE ($1::BOOLEAN IS NULL OR tgh.torrent_staff_checked = $1)
              AND ($2::BOOLEAN IS NULL OR tgh.torrent_reported = $2)
              AND (
                 $3::INT IS NULL OR
                 -- don't return torrents created as anonymous
                 -- unless the requesting user is the uploader
                 (tgh.torrent_created_by_id = $3 AND (
                    tgh.torrent_created_by_id = $4 OR
                    NOT tgh.torrent_uploaded_as_anonymous)
                 )
             )

            AND (
                $5::TEXT IS NULL OR
                    tgh.title_group_name ILIKE '%' || $5 || '%' OR
                    tgh.title_group_series_name ILIKE '%' || $5 || '%'
            )
            AND ($6::TEXT IS NULL OR $6 = ANY(tgh.title_group_external_links))

            GROUP BY title_group_id, title_group_name, title_group_covers, title_group_category,
            title_group_content_type, title_group_tags, title_group_original_release_date, title_group_platform            
            "#,
            form.torrent_staff_checked,
            form.torrent_reported,
            form.torrent_created_by_id,
            requesting_user_id,
            name_filter,
            external_link_filter
        )
        .fetch_optional(self.borrow())
        .await
        .map_err(|error| {
            Error::ErrorSearchingForTorrents(error.to_string())
        })?.unwrap_or(Some(0));

        // second: get the edition groups that match the edition group filters, that are within the title groups
        // from the previous step

        let title_group_ids: Vec<i32> = title_groups.iter().map(|t| t.id).collect();

        let edition_groups = sqlx::query_as!(
            EditionGroupHierarchyLite,
            r#"
            SELECT
                id,
                title_group_id,
                name,
                release_date,
                distributor,
                covers,
                source AS "source: _",
                additional_information AS "additional_information: _",
                '[]'::jsonb AS "torrents!: _"
            FROM edition_groups
            WHERE title_group_id = ANY($1)
            "#,
            &title_group_ids
        )
        .fetch_all(self.borrow())
        .await?;

        let mut grouped_editions: HashMap<i32, Vec<EditionGroupHierarchyLite>> = HashMap::new();

        for eg in edition_groups {
            grouped_editions.entry(eg.title_group_id).or_default().push(
                EditionGroupHierarchyLite {
                    torrents: Json(Vec::new()),
                    ..eg
                },
            );
        }

        let title_groups: Vec<TitleGroupHierarchyLite> = title_groups
            .into_iter()
            .map(|mut tg| {
                tg.edition_groups = Json(grouped_editions.remove(&tg.id).unwrap_or_default());
                tg
            })
            .collect();

        // third: get the torrents that match the torrent filters, and are in the edition groups
        // from the previous step

        let edition_group_ids: Vec<i32> = title_groups
            .iter()
            .flat_map(|tg| tg.edition_groups.0.iter().map(|eg| eg.id))
            .collect();

        let torrents = sqlx::query_as!(
            TorrentHierarchyLite,
            r#"
            SELECT
                id AS "id!",
                upload_factor AS "upload_factor!",
                download_factor AS "download_factor!",
                seeders AS "seeders!",
                leechers AS "leechers!",
                times_completed AS "times_completed!",
                snatched AS "snatched!",
                edition_group_id AS "edition_group_id!",
                created_at AS "created_at!: _",
                release_name,
                release_group,
                trumpable AS "trumpable!",
                staff_checked AS "staff_checked!",
                COALESCE(languages, '{}') AS "languages!: _",
                container AS "container!",
                size AS "size!",
                duration,
                audio_codec AS "audio_codec: _",
                audio_bitrate,
                audio_bitrate_sampling AS "audio_bitrate_sampling: _",
                audio_channels AS "audio_channels: _",
                video_codec AS "video_codec: _",
                features AS "features!: _",
                COALESCE(subtitle_languages, '{}') AS "subtitle_languages!: _",
                video_resolution AS "video_resolution: _",
                video_resolution_other_x,
                video_resolution_other_y,
                reports AS "reports!: _",
                COALESCE(extras, '{}') AS "extras!: _"
            FROM torrents_and_reports tar
            WHERE edition_group_id = ANY($1)

            AND ($3::BOOLEAN IS NULL OR tar.staff_checked = $3)
            AND ($4::BOOLEAN IS NULL OR tar.reported = $4)
            AND (
               $2::INT IS NULL OR
               -- don't return torrents created as anonymous
               -- unless the requesting user is the uploader
               (tar.created_by_id = $2 AND (
                  tar.created_by_id = $5 OR
                  NOT tar.uploaded_as_anonymous)
               )
            )

            ORDER BY size DESC
            "#,
            &edition_group_ids,
            form.torrent_created_by_id,
            form.torrent_staff_checked,
            form.torrent_reported,
            requesting_user_id,
        )
        .fetch_all(self.borrow())
        .await?;

        let mut grouped_torrents: HashMap<i32, Vec<TorrentHierarchyLite>> = HashMap::new();

        for t in torrents {
            grouped_torrents
                .entry(t.edition_group_id)
                .or_default()
                .push(t);
        }

        let title_groups = title_groups
            .into_iter()
            .map(|mut tg| {
                let edition_groups_with_torrents: Vec<EditionGroupHierarchyLite> = tg
                    .edition_groups
                    .0
                    .into_iter()
                    .map(|mut eg| {
                        eg.torrents = Json(grouped_torrents.remove(&eg.id).unwrap_or_default());
                        eg
                    })
                    .collect();

                tg.edition_groups = Json(edition_groups_with_torrents);
                tg
            })
            .collect();

        Ok(PaginatedResults {
            results: title_groups,
            page: form.page as u32,
            page_size: form.page_size as u32,
            total_items: total_title_groups_count.unwrap_or(0),
        })
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
                END AND t.deleted_at is NULL
                GROUP BY tg.id, tg.name
                ORDER BY tg.id, COUNT(st.torrent_id) DESC
                LIMIT $2
                ----------
            ),
            title_group_data AS (
                SELECT
                    tgl.title_group_data AS lite_title_group -- 'affiliated_artists' is already inside tgl.title_group_data
                FROM get_title_groups_and_edition_group_and_torrents_lite tgl
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
        current_user_id: i32,
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
            UPDATE torrents SET deleted_at = NOW(), deleted_by_id = $1 WHERE id = $2;
            "#,
            current_user_id,
            torrent_to_delete.id
        )
        .execute(&mut *tx)
        .await
        .map_err(|error| Error::ErrorDeletingTorrent(error.to_string()))?;

        tx.commit().await?;

        Ok(())
    }

    // pub async fn update_torrent_seeders_leechers(&self) -> Result<()> {
    //     let _ = sqlx::query!(
    //         r#"
    //         WITH peer_counts AS (
    //             SELECT
    //                 torrent_id,
    //                 COUNT(CASE WHEN status = 'seeding' THEN 1 END) AS current_seeders,
    //                 COUNT(CASE WHEN status = 'leeching' THEN 1 END) AS current_leechers
    //             FROM
    //                 peers
    //             GROUP BY
    //                 torrent_id
    //         )
    //         UPDATE torrents AS t
    //         SET
    //             seeders = COALESCE(pc.current_seeders, 0),
    //             leechers = COALESCE(pc.current_leechers, 0)
    //         FROM
    //             torrents AS t_alias -- Use an alias for the table in the FROM clause to avoid ambiguity
    //         LEFT JOIN
    //             peer_counts AS pc ON t_alias.id = pc.torrent_id
    //         WHERE
    //             t.id = t_alias.id AND
    //             t.deleted_at IS NULL;
    //         "#
    //     )
    //     .execute(self.borrow())
    //     .await?;

    //     Ok(())
    // }

    pub async fn increment_torrent_times_completed(&self, torrent_id: i32) -> Result<()> {
        let _ = sqlx::query!(
            r#"
            UPDATE torrents
            SET
                times_completed = times_completed + 1
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
            SELECT id, created_at, ENCODE(info_hash, 'hex') as info_hash FROM torrents WHERE deleted_at IS NULL;
            "#
        )
        .fetch_all(self.borrow())
        .await?;

        Ok(torrents)
    }
}
