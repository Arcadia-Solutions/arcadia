use crate::{
    Error, Result,
    models::{
        title_group::{TitleGroup, UserCreatedTitleGroup},
        user::User,
    },
};
use serde_json::Value;
use sqlx::PgPool;

fn sanitize_title_group_tags(tags: Vec<String>) -> Vec<String> {
    tags.into_iter()
        .map(|s| {
            s.trim()
                .to_lowercase()
                .split_whitespace()
                .collect::<Vec<&str>>()
                .join(".")
        })
        .collect()
}

pub async fn create_title_group(
    pool: &PgPool,
    title_group_form: &UserCreatedTitleGroup,
    current_user: &User,
) -> Result<TitleGroup> {
    let create_title_group_query = r#"
        INSERT INTO title_groups (master_group_id,name,name_aliases,created_by_id,description,original_language,country_from,covers,external_links,embedded_links,category,content_type,original_release_date,tags,tagline,platform,screenshots)
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11::title_group_category_enum, $12::content_type_enum, $13, $14, $15, $16, $17)
        RETURNING *;
    "#;

    let created_title_group = sqlx::query_as::<_, TitleGroup>(create_title_group_query)
        .bind(title_group_form.master_group_id)
        .bind(&title_group_form.name)
        .bind(&title_group_form.name_aliases)
        .bind(current_user.id)
        .bind(&title_group_form.description)
        .bind(&title_group_form.original_language)
        .bind(&title_group_form.country_from)
        .bind(&title_group_form.covers)
        .bind(&title_group_form.external_links)
        .bind(&title_group_form.embedded_links)
        .bind(&title_group_form.category)
        .bind(&title_group_form.content_type)
        .bind(title_group_form.original_release_date)
        .bind(&sanitize_title_group_tags(title_group_form.tags.clone()))
        .bind(&title_group_form.tagline)
        .bind(&title_group_form.platform)
        .bind(&title_group_form.screenshots)
        // .bind(&title_group_form.public_ratings)
        .fetch_one(pool)
        .await
        .map_err(Error::CouldNotCreateTitleGroup)?;

    Ok(created_title_group)
}

pub async fn find_title_group(
    pool: &PgPool,
    title_group_id: i64,
    current_user: &User,
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
            torrent_request_with_bounties AS (
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
                        (to_jsonb(trb) - 'created_by_id') ||
                        jsonb_build_object(
                            'created_by', jsonb_build_object(
                                'id', trb.created_by_id,
                                'username', trb.username,
                                'warned', trb.warned,
                                'banned', trb.banned
                            ),
                            'bounties', jsonb_build_object(
                                'upload', trb.total_upload_bounty,
                                'bonus_points', trb.total_bonus_bounty
                            ),
                            'user_votes_amount', trb.user_votes_amount
                        )
                        ORDER BY trb.id
                    ) AS torrent_requests
                FROM torrent_request_with_bounties trb
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
                        FROM title_group_subscriptions tgs
                        WHERE tgs.title_group_id = tg.id
                        AND tgs.subscriber_id = $1
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
            )
            SELECT
                to_jsonb(tg) || jsonb_build_object(
                    'series', COALESCE(sd.series, '{}'::jsonb),
                    'edition_groups', COALESCE(ed.edition_groups, '[]'::jsonb),
                    'affiliated_artists', COALESCE(ad.affiliated_artists, '[]'::jsonb),
                    'title_group_comments', COALESCE(cd.title_group_comments, '[]'::jsonb),
                    'torrent_requests', COALESCE(trd.torrent_requests, '[]'::jsonb),
                    'is_subscribed', COALESCE(sud.is_subscribed, false),
                    'in_same_master_group', COALESCE(smg.in_same_master_group, '[]'::jsonb)
                ) AS title_group_data
            FROM title_groups tg
            LEFT JOIN edition_data ed ON ed.title_group_id = tg.id
            LEFT JOIN artist_data ad ON ad.title_group_id = tg.id
            LEFT JOIN comment_data cd ON cd.title_group_id = tg.id
            LEFT JOIN series_data sd ON sd.title_group_id = tg.id
            LEFT JOIN torrent_request_data trd ON trd.title_group_id = tg.id
            LEFT JOIN subscription_data sud ON sud.id = tg.id
            LEFT JOIN same_master_group smg ON TRUE -- Only one row will be returned from same_master_group when master_group_id is set
            WHERE tg.id = $2;"#, current_user.id, title_group_id)
        .fetch_one(pool)
        .await?;

    Ok(title_group.title_group_data.unwrap())
}
pub async fn find_title_group_info_lite(
    pool: &PgPool,
    title_group_id: Option<i64>,
    title_group_name: Option<&str>,
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
            WHERE ($1::BIGINT IS NOT NULL AND tg.id = $1)
               OR ($2::TEXT IS NOT NULL AND (tg.name ILIKE '%' || $2 || '%' OR $2 = ANY(tg.name_aliases)))
            GROUP BY tg.id
            LIMIT $3
        ) AS subquery;
        "#,
        title_group_id,
        title_group_name,
        limit as i32
    )
    .fetch_one(pool)
    .await?;

    Ok(title_groups
        .jsonb_agg
        .unwrap_or_else(|| serde_json::Value::Array(vec![])))
}
