use crate::{
    connection_pool::ConnectionPool,
    models::{
        title_group::{
            ContentType, EditedTitleGroup, Platform, PublicRating, TitleGroup, TitleGroupCategory,
            UserCreatedTitleGroup,
        },
        title_group_tag::UserCreatedTitleGroupTag,
        torrent::Language,
    },
};
use arcadia_common::error::{Error, Result};
use serde_json::{json, Value};
use std::borrow::Borrow;

impl ConnectionPool {
    pub async fn create_title_group(
        &self,
        title_group_form: &UserCreatedTitleGroup,
        public_ratings: &Vec<PublicRating>,
        user_id: i32,
    ) -> Result<TitleGroup> {
        let created_title_group_id: i32 = sqlx::query_scalar!(
            r#"
            INSERT INTO title_groups (
                master_group_id,
                name,
                name_aliases,
                created_by_id,
                description,
                original_language,
                country_from,
                covers,
                external_links,
                embedded_links,
                category,
                content_type,
                original_release_date,
                tagline,
                platform,
                screenshots,
                public_ratings
            )
            VALUES (
                $1, $2, $3, $4, $5, $6::language_enum,
                $7, $8, $9, $10, $11::title_group_category_enum,
                $12::content_type_enum, $13, $14, $15, $16, $17
            )
            RETURNING id
            "#,
            title_group_form.master_group_id,
            &title_group_form.name,
            &title_group_form.name_aliases,
            user_id,
            &title_group_form.description,
            title_group_form.original_language.clone() as Option<Language>,
            title_group_form.country_from,
            &title_group_form.covers,
            &title_group_form.external_links,
            &title_group_form.embedded_links,
            title_group_form.category.clone() as Option<TitleGroupCategory>,
            title_group_form.content_type.clone() as ContentType,
            title_group_form.original_release_date,
            title_group_form.tagline,
            title_group_form.platform.clone() as Option<Platform>,
            &title_group_form.screenshots,
            json!(public_ratings)
        )
        .fetch_one(self.borrow())
        .await
        .map_err(Error::CouldNotCreateTitleGroup)?;

        // ensure tags exist
        let mut tag_ids = Vec::new();
        for tag_name in title_group_form.tags.iter() {
            let tag = Self::create_title_group_tag(
                self,
                &UserCreatedTitleGroupTag {
                    name: tag_name.clone(),
                },
                user_id,
            )
            .await?;
            tag_ids.push(tag.id);
        }

        // apply tags to title group
        for tag_id in tag_ids {
            Self::apply_tag_to_title_group(self, created_title_group_id, tag_id, user_id).await?;
        }

        let created_title_group = Self::find_title_group(self, created_title_group_id).await?;

        Ok(created_title_group)
    }

    pub async fn find_title_group_hierarchy(
        &self,
        title_group_id: i32,
        user_id: i32,
    ) -> Result<Value> {
        let title_group = sqlx::query!(r#"WITH torrent_data AS (
                    SELECT
                        t.edition_group_id,
                        jsonb_agg(
                            -- Handle anonymity: show creator info only if requesting user is the uploader or if not anonymous
                            CASE
                                WHEN t.uploaded_as_anonymous AND t.created_by_id != $1 THEN
                                    (to_jsonb(t) - 'created_by_id' - 'display_created_by_id' - 'display_created_by') ||
                                    jsonb_build_object('created_by_id', NULL, 'created_by', NULL, 'uploaded_as_anonymous', true)
                                ELSE
                                    (to_jsonb(t) - 'display_created_by_id' - 'display_created_by') ||
                                    jsonb_build_object('created_by', to_jsonb(u))
                            END
                            ORDER BY t.size DESC
                        ) AS torrents
                    FROM torrents_and_reports t
                    LEFT JOIN users u ON u.id = t.created_by_id
                    GROUP BY t.edition_group_id
                ),
                torrent_request_with_bounty AS (
                    SELECT
                        tr.*,
                        u.username,
                        u.warned,
                        u.banned,
                        COALESCE(SUM(trv.bounty_upload), 0) AS total_upload_bounty,
                        COALESCE(SUM(trv.bounty_bonus_points), 0) AS total_bonus_bounty,
                        COUNT(DISTINCT trv.created_by_id) AS user_votes_amount
                    FROM torrent_requests tr
                    LEFT JOIN torrent_request_votes trv ON tr.id = trv.torrent_request_id
                    LEFT JOIN users u ON u.id = tr.created_by_id -- Join with users table
                    GROUP BY
                        tr.id,
                        tr.title_group_id,
                        tr.created_at,
                        tr.updated_at,
                        tr.created_by_id,
                        tr.filled_by_user_id,
                        tr.filled_by_torrent_id,
                        tr.filled_at,
                        tr.edition_name,
                        tr.release_group,
                        tr.description,
                        tr.languages,
                        tr.container,
                        tr.audio_codec,
                        tr.audio_channels,
                        tr.audio_bitrate_sampling,
                        tr.video_codec,
                        tr.features,
                        tr.subtitle_languages,
                        tr.video_resolution,
                        u.username,
                        u.warned,
                        u.banned
                ),
                torrent_request_data AS (
                    SELECT
                        trb.title_group_id,
                        jsonb_agg(
                            jsonb_build_object(
                                'torrent_request', to_jsonb(trb),
                                'created_by', jsonb_build_object(
                                    'id', trb.created_by_id,
                                    'username', trb.username,
                                    'warned', trb.warned,
                                    'banned', trb.banned
                                ),
                                'bounty', jsonb_build_object(
                                    'upload', trb.total_upload_bounty,
                                    'bonus_points', trb.total_bonus_bounty
                                ),
                                'user_votes_amount', trb.user_votes_amount
                            )
                            ORDER BY trb.id
                        ) AS torrent_requests
                    FROM torrent_request_with_bounty trb
                    GROUP BY trb.title_group_id
                ),
                edition_data AS (
                    SELECT
                        eg.title_group_id,
                        jsonb_agg(
                            to_jsonb(eg) || jsonb_build_object('torrents', COALESCE(td.torrents, '[]'::jsonb))
                            ORDER BY eg.release_date
                        ) AS edition_groups
                    FROM edition_groups eg
                    LEFT JOIN torrent_data td ON td.edition_group_id = eg.id
                    GROUP BY eg.title_group_id
                ),
                artist_data AS (
                    SELECT
                        aa.title_group_id,
                        jsonb_agg(
                            to_jsonb(aa) || jsonb_build_object('artist', to_jsonb(a))
                        ) AS affiliated_artists
                    FROM affiliated_artists aa
                    JOIN artists a ON a.id = aa.artist_id
                    GROUP BY aa.title_group_id
                ),
                entity_data AS (
                    SELECT
                        ae.title_group_id,
                        jsonb_agg(
                            to_jsonb(ae) || jsonb_build_object('entity', to_jsonb(e))
                        ) AS affiliated_entities
                    FROM affiliated_entities ae
                    JOIN entities e ON e.id = ae.entity_id
                    GROUP BY ae.title_group_id
                ),
                comment_data AS (
                    SELECT
                        c.title_group_id,
                        jsonb_agg(
                            to_jsonb(c) || jsonb_build_object('created_by', jsonb_build_object('id', u.id, 'username', u.username, 'avatar', u.avatar, 'warned', u.warned, 'banned', u.banned))
                            ORDER BY c.created_at
                        ) AS title_group_comments
                    FROM title_group_comments c
                    LEFT JOIN users u ON u.id = c.created_by_id
                    GROUP BY c.title_group_id
                ),
                series_data AS (
                    SELECT
                        tg.id AS title_group_id,
                        jsonb_build_object('name', s.name, 'id', s.id) AS series
                    FROM title_groups tg
                    LEFT JOIN series s ON s.id = tg.series_id
                ),
                subscription_data AS (
                    SELECT
                        id,
                        EXISTS(
                            SELECT 1
                            FROM subscriptions_title_group_torrents tgs
                            WHERE tgs.title_group_id = tg.id
                            AND tgs.user_id = $1
                        ) AS is_subscribed
                    FROM title_groups tg
                ),
                same_master_group AS (
                    SELECT
                        jsonb_agg(jsonb_build_object('id', tg_inner.id, 'name', tg_inner.name, 'content_type', tg_inner.content_type, 'platform', tg_inner.platform)) AS in_same_master_group
                    FROM title_groups tg_main
                    JOIN title_groups tg_inner ON tg_inner.master_group_id = tg_main.master_group_id AND tg_inner.id != tg_main.id
                    WHERE tg_main.id = $2 AND tg_main.master_group_id IS NOT NULL
                    GROUP BY tg_main.master_group_id
                ),
                collage_metrics AS (
                    SELECT
                        collage_id,
                        COUNT(id) AS entries_amount,
                        MAX(created_at) AS last_entry_at
                    FROM collage_entry
                    GROUP BY collage_id
                ),
                collage_data AS (
                    SELECT
                        ce.title_group_id,
                        jsonb_agg(
                            jsonb_build_object(
                                'id', c.id,
                                'created_at', c.created_at,
                                'created_by_id', c.created_by_id,
                                'created_by', jsonb_build_object(
                                    'id', u.id,
                                    'username', u.username,
                                    'warned', u.warned,
                                    'banned', u.banned
                                ),
                                'name', c.name,
                                'cover', c.cover,
                                'description', c.description,
                                'tags', c.tags,
                                'category', c.category,
                                'collage_type', c.collage_type,
                                'entries_amount', cm.entries_amount,
                                'last_entry_at', cm.last_entry_at
                            )
                            ORDER BY c.created_at
                        ) AS collages
                    FROM collage_entry ce
                    JOIN collage c ON c.id = ce.collage_id
                    JOIN users u ON u.id = c.created_by_id
                    LEFT JOIN collage_metrics cm ON cm.collage_id = c.id
                    WHERE ce.title_group_id = $2
                    GROUP BY ce.title_group_id
                ),
                title_group_tags AS (
                    SELECT
                        tg.id AS title_group_id,
                        COALESCE(
                            ARRAY(
                                SELECT t.name
                                FROM title_group_applied_tags tat
                                JOIN title_group_tags t ON t.id = tat.tag_id
                                WHERE tat.title_group_id = tg.id
                            ),
                            ARRAY[]::text[]
                        ) AS tags
                    FROM title_groups tg
                )
                SELECT
                    jsonb_build_object(
                        'title_group', to_jsonb(tg) || jsonb_build_object('tags', COALESCE(td.tags, ARRAY[]::text[])),
                        'series', COALESCE(sd.series, '{}'::jsonb),
                        'edition_groups', COALESCE(ed.edition_groups, '[]'::jsonb),
                        'affiliated_artists', COALESCE(ad.affiliated_artists, '[]'::jsonb),
                        'affiliated_entities', COALESCE(aed.affiliated_entities, '[]'::jsonb),
                        'title_group_comments', COALESCE(cd.title_group_comments, '[]'::jsonb),
                        'torrent_requests', COALESCE(trd.torrent_requests, '[]'::jsonb),
                        'is_subscribed', COALESCE(sud.is_subscribed, false),
                        'in_same_master_group', COALESCE(smg.in_same_master_group, '[]'::jsonb),
                        'collages', COALESCE(cod.collages, '[]'::jsonb)
                    ) AS title_group_data
                FROM title_groups tg
                LEFT JOIN title_group_tags td ON td.title_group_id = tg.id
                LEFT JOIN edition_data ed ON ed.title_group_id = tg.id
                LEFT JOIN artist_data ad ON ad.title_group_id = tg.id
                LEFT JOIN entity_data aed ON aed.title_group_id = tg.id
                LEFT JOIN comment_data cd ON cd.title_group_id = tg.id
                LEFT JOIN series_data sd ON sd.title_group_id = tg.id
                LEFT JOIN torrent_request_data trd ON trd.title_group_id = tg.id
                LEFT JOIN subscription_data sud ON sud.id = tg.id
                LEFT JOIN same_master_group smg ON TRUE -- Only one row will be returned from same_master_group when master_group_id is set
                LEFT JOIN collage_data cod ON cod.title_group_id = tg.id
                WHERE tg.id = $2;"#, user_id, title_group_id)
            .fetch_one(self.borrow())
            .await?;

        Ok(title_group.title_group_data.unwrap())
    }
    pub async fn find_title_group_info_lite(
        &self,
        title_group_id: Option<i32>,
        title_group_name: Option<&str>,
        title_group_content_type: &Option<ContentType>,
        limit: u32,
    ) -> Result<Value> {
        let title_groups = sqlx::query!(
            r#"
            SELECT jsonb_agg(data)
                FROM (
                    SELECT jsonb_build_object(
                        'id', tg.id, 'content_type', tg.content_type, 'name', tg.name, 'platform', tg.platform, 'covers', tg.covers,
                        'original_release_date', tg.original_release_date,
                        'edition_groups', COALESCE(
                            jsonb_agg(
                                jsonb_build_object(
                                    'id', eg.id,
                                    'name', eg.name,
                                    'release_date', eg.release_date,
                                    'distributor', eg.distributor,
                                    'source', eg.source,
                                    'additional_information', eg.additional_information
                                )
                            ) FILTER (WHERE eg.id IS NOT NULL),
                            '[]'::jsonb
                        )
                    ) as data
                    FROM title_groups tg
                    LEFT JOIN edition_groups eg ON eg.title_group_id = tg.id
                    LEFT JOIN (
                        SELECT edition_group_id, MAX(created_at) as created_at
                        FROM torrents
                        GROUP BY edition_group_id
                    ) AS latest_torrent ON latest_torrent.edition_group_id = eg.id
                    WHERE ($1::INT IS NOT NULL AND tg.id = $1)
                        OR ($2::TEXT IS NOT NULL AND (tg.name ILIKE '%' || $2 || '%' OR $2 = ANY(tg.name_aliases)))
                        AND ($3::content_type_enum IS NULL OR tg.content_type = $3::content_type_enum)
                    GROUP BY tg.id
                    ORDER BY MAX(latest_torrent.created_at) DESC NULLS LAST
                    LIMIT $4
                ) AS subquery;
            "#,
            title_group_id,
            title_group_name,
            title_group_content_type as &Option<ContentType>,
            limit as i32
        )
        .fetch_one(self.borrow())
        .await?;

        Ok(title_groups
            .jsonb_agg
            .unwrap_or_else(|| serde_json::Value::Array(vec![])))
    }

    pub async fn find_title_group(&self, title_group_id: i32) -> Result<TitleGroup> {
        let title_group = sqlx::query_as!(
            TitleGroup,
            r#"
            SELECT
                id, master_group_id, name, name_aliases AS "name_aliases!: _",
                created_at, updated_at, created_by_id, description,
                platform AS "platform: _", original_language AS "original_language: _", original_release_date,
                tagline, country_from, covers AS "covers!: _",
                external_links AS "external_links!: _", embedded_links,
                category AS "category: _", content_type AS "content_type: _",
                public_ratings, screenshots AS "screenshots!: _", series_id,
                COALESCE(
                    ARRAY(
                        SELECT t.name
                        FROM title_group_applied_tags tat
                        JOIN title_group_tags t ON t.id = tat.tag_id
                        WHERE tat.title_group_id = title_groups.id
                    ),
                    ARRAY[]::text[]
                ) AS "tags!: _"
            FROM title_groups
            WHERE id = $1
            "#,
            title_group_id
        )
        .fetch_one(self.borrow())
        .await
        .map_err(|_| Error::TitleGroupNotFound)?;

        Ok(title_group)
    }

    pub async fn update_title_group(
        &self,
        edited_title_group: &EditedTitleGroup,
        title_group_id: i32,
    ) -> Result<TitleGroup> {
        let updated_title_group = sqlx::query_as!(
            TitleGroup,
            r#"
            UPDATE title_groups
            SET
                master_group_id = $2,
                name = $3,
                name_aliases = $4,
                description = $5,
                platform = $6,
                original_language = $7,
                original_release_date = $8,
                tagline = $9,
                country_from = $10,
                covers = $11,
                external_links = $12,
                embedded_links = $13,
                category = $14,
                content_type = $15,
                screenshots = $16,
                updated_at = NOW()
            WHERE id = $1
            RETURNING
                id, master_group_id, name, name_aliases AS "name_aliases!: _",
                created_at, updated_at, created_by_id, description,
                platform AS "platform: _", original_language AS "original_language: _", original_release_date,
                tagline, country_from, covers AS "covers!: _",
                external_links AS "external_links!: _", embedded_links,
                category AS "category: _", content_type AS "content_type: _",
                public_ratings, screenshots AS "screenshots!: _", series_id,
                COALESCE(
                    ARRAY(
                        SELECT t.name
                        FROM title_group_applied_tags tat
                        JOIN title_group_tags t ON t.id = tat.tag_id
                        WHERE tat.title_group_id = title_groups.id
                    ),
                    ARRAY[]::text[]
                ) AS "tags!: _"
            "#,
            title_group_id,
            edited_title_group.master_group_id,
            edited_title_group.name,
            edited_title_group.name_aliases as _,
            edited_title_group.description,
            edited_title_group.platform as _,
            edited_title_group.original_language as _,
            edited_title_group.original_release_date,
            edited_title_group.tagline,
            edited_title_group.country_from,
            edited_title_group.covers as _,
            edited_title_group.external_links as _,
            edited_title_group.embedded_links,
            edited_title_group.category as _,
            edited_title_group.content_type as _,
            edited_title_group.screenshots as _
        )
        .fetch_one(self.borrow())
        .await
        .map_err(|e| Error::ErrorWhileUpdatingTitleGroup(e.to_string()))?;

        Ok(updated_title_group)
    }

    pub async fn assign_title_group_to_series(
        &self,
        title_group_id: i32,
        series_id: i64,
    ) -> Result<()> {
        let _ = sqlx::query!(
            r#"
            UPDATE title_groups
            SET series_id = $2, updated_at = NOW()
            WHERE id = $1
            "#,
            title_group_id,
            series_id
        )
        .fetch_one(self.borrow())
        .await
        .map_err(|e| Error::ErrorWhileUpdatingTitleGroup(e.to_string()))?;

        Ok(())
    }

    pub async fn does_title_group_with_link_exist(
        &self,
        external_link: &str,
    ) -> Result<Option<i32>> {
        let title_group_id: Option<i32> = sqlx::query_scalar!(
            r#"
            SELECT id
            FROM title_groups
            WHERE external_links @> ARRAY[$1::TEXT];
            "#,
            external_link
        )
        .fetch_optional(self.borrow())
        .await
        .map_err(|e| Error::ErrorSearchingForTitleGroup(e.to_string()))?;

        Ok(title_group_id)
    }
}
