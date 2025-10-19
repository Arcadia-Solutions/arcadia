use crate::{
    connection_pool::ConnectionPool,
    models::torrent_request::{EditedTorrentRequest, TorrentRequest, UserCreatedTorrentRequest},
};
use arcadia_common::error::{Error, Result};
use serde_json::Value;
use sqlx::{query_as, query_scalar, PgPool};
use std::borrow::Borrow;

impl ConnectionPool {
    pub async fn create_torrent_request(
        &self,
        torrent_request: &mut UserCreatedTorrentRequest,
        user_id: i32,
    ) -> Result<TorrentRequest> {
        //TODO: make those requests transactional
        let create_torrent_request_query = r#"
            INSERT INTO torrent_requests
            (title_group_id, created_by_id, edition_name,
            release_group, description, languages, container, audio_codec,
            audio_channels, video_codec, features, subtitle_languages, video_resolution,
            audio_bitrate_sampling, source)
            VALUES ($1, $2, $3, $4, $5, $6::language_enum[], $7, $8::audio_codec_enum[], $9::audio_channels_enum[],
            $10::video_codec_enum[], $11::features_enum[], $12::language_enum[], $13::video_resolution_enum[],
            $14::audio_bitrate_sampling_enum[], $15::source_enum[])
            RETURNING *;
        "#;

        let created_torrent_request =
            sqlx::query_as::<_, TorrentRequest>(create_torrent_request_query)
                .bind(torrent_request.title_group_id)
                .bind(user_id)
                .bind(&torrent_request.edition_name)
                .bind(&torrent_request.release_group)
                .bind(&torrent_request.description)
                .bind(&torrent_request.languages)
                .bind(&torrent_request.container)
                .bind(&torrent_request.audio_codec)
                .bind(&torrent_request.audio_channels)
                .bind(&torrent_request.video_codec)
                .bind(&torrent_request.features)
                .bind(&torrent_request.subtitle_languages)
                .bind(&torrent_request.video_resolution)
                .bind(&torrent_request.audio_bitrate_sampling)
                .bind(&torrent_request.source)
                .fetch_one(self.borrow())
                .await
                .map_err(Error::CouldNotCreateTorrentRequest)?;

        torrent_request.initial_vote.torrent_request_id = created_torrent_request.id;

        let _ = self
            .create_torrent_request_vote(&torrent_request.initial_vote, user_id)
            .await?;

        Ok(created_torrent_request)
    }

    pub async fn fill_torrent_request(
        &self,
        torrent_id: i32,
        torrent_request_id: i64,
        current_user_id: i32,
    ) -> Result<()> {
        let is_torrent_in_requested_title_group = sqlx::query_scalar!(
            r#"
            SELECT EXISTS (
                SELECT 1
                FROM torrents t
                JOIN edition_groups eg ON t.edition_group_id = eg.id
                JOIN torrent_requests r ON eg.title_group_id = r.title_group_id
                WHERE t.id = $1
                  AND r.id = $2
            )
            "#,
            torrent_id,
            torrent_request_id
        )
        .fetch_one(self.borrow())
        .await?;

        if !is_torrent_in_requested_title_group.unwrap() {
            return Err(Error::TorrentTitleGroupNotMatchingRequestedOne);
        }

        let is_torrent_request_filled = sqlx::query_scalar!(
            r#"
            SELECT EXISTS (
                SELECT 1
                FROM torrent_requests tr
                WHERE tr.id = $1 AND tr.filled_at IS NOT NULL
            )
            "#,
            torrent_request_id
        )
        .fetch_one(self.borrow())
        .await?;

        if is_torrent_request_filled.unwrap() {
            return Err(Error::TorrentRequestAlreadyFilled);
        }

        #[derive(Debug)]
        struct BountySummary {
            total_upload: i64,
            total_bonus: i64,
        }

        let bounty_summary = query_as!(
            BountySummary,
            r#"
            SELECT
                COALESCE(SUM(bounty_upload)::BIGINT, 0::BIGINT) AS "total_upload!",
                COALESCE(SUM(bounty_bonus_points)::BIGINT, 0::BIGINT) AS "total_bonus!"
            FROM torrent_request_votes
            WHERE torrent_request_id = $1
            "#,
            torrent_request_id
        )
        .fetch_one(self.borrow())
        .await?;

        // Calculate the share for each user (50% each).
        // Ensure floating-point division for accurate half-shares, then cast back to i64 for database.
        let upload_share = (bounty_summary.total_upload as f32 / 2.0).round() as i32;
        let bonus_share = (bounty_summary.total_bonus as f32 / 2.0).round() as i32;

        let torrent_uploader_id: i32 = query_scalar!(
            r#"
                    SELECT created_by_id FROM torrents WHERE id = $1
                "#,
            torrent_id
        )
        .fetch_one(self.borrow())
        .await?;

        let mut tx = <ConnectionPool as Borrow<PgPool>>::borrow(self)
            .begin()
            .await?;

        sqlx::query!(
            r#"
            UPDATE users
            SET
                uploaded = users.uploaded +
                    CASE
                        WHEN users.id = $1 THEN $3
                        WHEN users.id = $2 THEN $3
                        ELSE 0
                    END,
                bonus_points = users.bonus_points +
                    CASE
                        WHEN users.id = $1 THEN $4
                        WHEN users.id = $2 THEN $4
                        ELSE 0
                    END
            WHERE
                users.id IN ($1, $2)
                "#,
            torrent_uploader_id,
            current_user_id,
            upload_share,
            bonus_share
        )
        .execute(&mut *tx)
        .await?;

        sqlx::query!(
            r#"
            UPDATE torrent_requests tr
            SET
                filled_by_torrent_id = $1,
                filled_by_user_id = $2,
                filled_at = NOW()
            WHERE
                tr.id = $3
                "#,
            torrent_id,
            current_user_id,
            torrent_request_id
        )
        .execute(&mut *tx)
        .await?;

        tx.commit().await?;

        Ok(())
    }

    pub async fn find_torrent_request(&self, torrent_request_id: i64) -> Result<TorrentRequest> {
        let torrent_request = sqlx::query_as!(
            TorrentRequest,
            r#"
            SELECT
                id, title_group_id, edition_name, release_group,
                created_at, updated_at, created_by_id, description,
                languages AS "languages!: _", container, audio_codec AS "audio_codec: _",
                audio_channels AS "audio_channels: _", video_codec AS "video_codec: _", features AS "features!: _", subtitle_languages AS "subtitle_languages!: _", video_resolution AS "video_resolution!: _",
                video_resolution_other_x, video_resolution_other_y,
                audio_bitrate_sampling AS "audio_bitrate_sampling!: _", source AS "source!: _",
                filled_by_user_id, filled_by_torrent_id, filled_at
            FROM torrent_requests
            WHERE id = $1
            "#,
            torrent_request_id
        )
        .fetch_one(self.borrow())
        .await
        .map_err(|_| Error::TorrentRequestNotFound)?;

        Ok(torrent_request)
    }
    pub async fn search_torrent_requests(
        &self,
        title_group_name: Option<&str>,
        tags: Option<&[String]>,
        page: i64,
        page_size: i64,
    ) -> Result<Value> {
        let offset = (page - 1).max(0) * page_size;
        let rows: Option<Value> = sqlx::query_scalar!(
            r#"
            SELECT COALESCE(json_agg(data), '[]'::json) as data FROM (
                SELECT json_build_object(
                    'torrent_request', tr,
                    'title_group', json_build_object(
                        'id', tg.id,
                        'name', tg.name,
                        'content_type', tg.content_type,
                        'original_release_date', tg.original_release_date,
                        'covers', tg.covers,
                        'edition_groups', '[]',
                        'platform', tg.platform
                    ),
                    'bounty', json_build_object(
                        'upload', (
                            SELECT COALESCE(SUM(trv.bounty_upload), 0)
                            FROM torrent_request_votes trv
                            WHERE trv.torrent_request_id = tr.id
                        ),
                        'bonus_points', (
                            SELECT COALESCE(SUM(trv.bounty_bonus_points), 0)
                            FROM torrent_request_votes trv
                            WHERE trv.torrent_request_id = tr.id
                        )
                    ),
                    'user_votes_amount', (
                        SELECT COALESCE(COUNT(DISTINCT trv2.created_by_id), 0)
                        FROM torrent_request_votes trv2
                        WHERE trv2.torrent_request_id = tr.id
                    ),
                    'affiliated_artists', COALESCE((
                        SELECT json_agg(
                            json_build_object(
                                'id', aa.id,
                                'title_group_id', aa.title_group_id,
                                'artist_id', aa.artist_id,
                                'roles', aa.roles,
                                'nickname', aa.nickname,
                                'created_at', aa.created_at,
                                'created_by_id', aa.created_by_id,
                                'artist', json_build_object(
                                    'id', a.id,
                                    'name', a.name,
                                    'created_at', a.created_at,
                                    'created_by_id', a.created_by_id,
                                    'description', a.description,
                                    'pictures', a.pictures,
                                    'title_groups_amount', a.title_groups_amount,
                                    'edition_groups_amount', a.edition_groups_amount,
                                    'torrents_amount', a.torrents_amount,
                                    'seeders_amount', a.seeders_amount,
                                    'leechers_amount', a.leechers_amount,
                                    'snatches_amount', a.snatches_amount
                                )
                            )
                        )
                        FROM affiliated_artists aa
                        JOIN artists a ON a.id = aa.artist_id
                        WHERE aa.title_group_id = tg.id
                    ), '[]'::json),
                    'series', COALESCE((
                        SELECT json_build_object('id', s.id, 'name', s.name)
                        FROM series s
                        WHERE s.id = tg.series_id
                    ), '{}'::json)
                ) as data
                FROM torrent_requests tr
                JOIN title_groups tg ON tr.title_group_id = tg.id
                WHERE ($1::TEXT IS NULL OR tg.name ILIKE '%' || $1 || '%' OR $1 = ANY(tg.name_aliases))
                AND ($2::VARCHAR[] IS NULL OR tg.tags && $2::VARCHAR[])
                ORDER BY tr.created_at DESC
                LIMIT $3 OFFSET $4
            ) sub
        "#,
            title_group_name,
            tags,
            page_size,
            offset
        )
        .fetch_one(self.borrow())
        .await
        .map_err(Error::CouldNotSearchForTorrentRequests)?;
        Ok(rows.unwrap())
    }

    pub async fn find_torrent_request_hierarchy(&self, torrent_request_id: i64) -> Result<Value> {
        let result = sqlx::query!(
            r#"
        SELECT json_build_object(
                    'torrent_request', tr,
                    'title_group', tg,
                    'affiliated_artists', COALESCE((
                        SELECT json_agg(
                            json_build_object(
                                'id', aa.id,
                                'title_group_id', aa.title_group_id,
                                'artist_id', aa.artist_id,
                                'roles', aa.roles,
                                'nickname', aa.nickname,
                                'created_at', aa.created_at,
                                'created_by_id', aa.created_by_id,
                                'artist', json_build_object(
                                    'id', a.id,
                                    'name', a.name,
                                    'created_at', a.created_at,
                                    'created_by_id', a.created_by_id,
                                    'description', a.description,
                                    'pictures', a.pictures,
                                    'title_groups_amount', a.title_groups_amount,
                                    'edition_groups_amount', a.edition_groups_amount,
                                    'torrents_amount', a.torrents_amount,
                                    'seeders_amount', a.seeders_amount,
                                    'leechers_amount', a.leechers_amount,
                                    'snatches_amount', a.snatches_amount
                                )
                            )
                        )
                        FROM affiliated_artists aa
                        JOIN artists a ON a.id = aa.artist_id
                        WHERE aa.title_group_id = tg.id
                    ), '[]'::json),
                    'series', COALESCE((
                        SELECT json_build_object('id', s.id, 'name', s.name)
                        FROM series s
                        WHERE s.id = tg.series_id
                    ), '{}'::json),
                    'votes', COALESCE((
                        SELECT json_agg(
                            json_build_object(
                                'id', trv3.id,
                                'torrent_request_id', trv3.torrent_request_id,
                                'created_at', trv3.created_at,
                                'created_by_id', trv3.created_by_id,
                                'created_by', json_build_object(
                                    'id', u.id,
                                    'username', u.username,
                                    'warned', u.warned,
                                    'banned', u.banned
                                ),
                                'bounty_upload', trv3.bounty_upload,
                                'bounty_bonus_points', trv3.bounty_bonus_points
                            )
                            ORDER BY trv3.created_at DESC
                        )
                        FROM torrent_request_votes trv3
                        JOIN users u ON u.id = trv3.created_by_id
                        WHERE trv3.torrent_request_id = tr.id
                    ), '[]'::json),
                    'comments', COALESCE((
                        SELECT json_agg(
                            json_build_object(
                                'id', trc.id,
                                'torrent_request_id', trc.torrent_request_id,
                                'created_by_id', trc.created_by_id,
                                'created_by', json_build_object(
                                    'id', u2.id,
                                    'username', u2.username,
                                    'warned', u2.warned,
                                    'banned', u2.banned
                                ),
                                'content', trc.content,
                                'created_at', trc.created_at,
                                'updated_at', trc.updated_at
                            )
                            ORDER BY trc.created_at DESC
                        )
                        FROM torrent_request_comments trc
                        JOIN users u2 ON u2.id = trc.created_by_id
                        WHERE trc.torrent_request_id = tr.id
                    ), '[]'::json)
                ) as data
                FROM torrent_requests tr
                JOIN title_groups tg ON tr.title_group_id = tg.id
                WHERE tr.id = $1
        "#,
            torrent_request_id
        )
        .fetch_one(self.borrow())
        .await
        .map_err(Error::CouldNotFindTheTorrentRequest)?;

        Ok(result.data.unwrap())
    }

    pub async fn update_torrent_request(
        &self,
        edited_torrent_request: &EditedTorrentRequest,
        torrent_request_id: i64,
    ) -> Result<TorrentRequest> {
        let updated_torrent_request = sqlx::query_as!(
            TorrentRequest,
            r#"
            UPDATE torrent_requests
            SET
                title_group_id = $2,
                edition_name = $3,
                release_group = $4,
                description = $5,
                languages = $6,
                container = $7,
                audio_codec = $8,
                audio_channels = $9,
                video_codec = $10,
                features = $11,
                subtitle_languages = $12,
                video_resolution = $13,
                audio_bitrate_sampling = $14,
                source = $15,
                updated_at = NOW()
            WHERE id = $1
            RETURNING
                id, title_group_id, edition_name, release_group,
                created_at, updated_at, created_by_id, description,
                languages AS "languages!: _", container, audio_codec AS "audio_codec: _",
                audio_channels AS "audio_channels: _", video_codec AS "video_codec: _", features AS "features!: _", subtitle_languages AS "subtitle_languages!: _", video_resolution AS "video_resolution!: _",
                video_resolution_other_x, video_resolution_other_y,
                audio_bitrate_sampling AS "audio_bitrate_sampling!: _", source AS "source!: _",
                filled_by_user_id, filled_by_torrent_id, filled_at
            "#,
            torrent_request_id,
            edited_torrent_request.title_group_id,
            edited_torrent_request.edition_name,
            edited_torrent_request.release_group,
            edited_torrent_request.description,
            edited_torrent_request.languages as _,
            &edited_torrent_request.container,
            edited_torrent_request.audio_codec as _,
            edited_torrent_request.audio_channels as _,
            edited_torrent_request.video_codec as _,
            edited_torrent_request.features as _,
            edited_torrent_request.subtitle_languages as _,
            edited_torrent_request.video_resolution as _,
            edited_torrent_request.audio_bitrate_sampling as _,
            edited_torrent_request.source as _,
        )
        .fetch_one(self.borrow())
        .await
        .map_err(|e| Error::ErrorWhileUpdatingTorrentRequest(e.to_string()))?;

        Ok(updated_torrent_request)
    }
}
