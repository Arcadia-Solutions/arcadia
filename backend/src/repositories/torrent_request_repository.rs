use crate::{
    Error, Result,
    models::{
        torrent_request::{TorrentRequest, UserCreatedTorrentRequest},
        user::User,
    },
    repositories::torrent_request_vote_repository::create_torrent_request_vote,
};
use serde_json::Value;
use sqlx::{PgPool, query_as, query_scalar};

pub async fn create_torrent_request(
    pool: &PgPool,
    torrent_request: &mut UserCreatedTorrentRequest,
    current_user: &User,
) -> Result<TorrentRequest> {
    //TODO: make those requests transactional
    let create_torrent_request_query = r#"
        INSERT INTO torrent_requests
        (edition_group_id, created_by_id, edition_name,
        release_group, description, languages, container, audio_codec,
        audio_channels, video_codec, features, subtitle_languages, video_resolution,
        audio_bitrate_sampling, source)
        VALUES ($1, $2, $3, $4, $5, $6::language_enum[], $7, $8::audio_codec_enum[], $9::audio_channels_enum[],
        $10::video_codec_enum[], $11::features_enum[], $12::language_enum[], $13::video_resolution_enum[],
        $14::audio_bitrate_sampling_enum[], $15::source_enum[])
        RETURNING *;
    "#;

    let created_torrent_request = sqlx::query_as::<_, TorrentRequest>(create_torrent_request_query)
        .bind(torrent_request.edition_group_id)
        .bind(current_user.id)
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
        .fetch_one(pool)
        .await
        .map_err(Error::CouldNotCreateTorrentRequest)?;

    torrent_request.initial_vote.torrent_request_id = created_torrent_request.id;

    let _ = create_torrent_request_vote(pool, &torrent_request.initial_vote, current_user).await?;

    Ok(created_torrent_request)
}

pub async fn fill_torrent_request(
    pool: &PgPool,
    torrent_id: i64,
    torrent_request_id: i64,
    current_user_id: i64,
) -> Result<()> {
    let is_torrent_in_requested_edition_group = sqlx::query_scalar!(
        r#"
        SELECT EXISTS (
            SELECT 1
            FROM torrents t
            JOIN torrent_requests r ON t.edition_group_id = r.edition_group_id
            WHERE t.id = $1
              AND r.id = $2
        )
        "#,
        torrent_id,
        torrent_request_id
    )
    .fetch_one(pool)
    .await?;

    if !is_torrent_in_requested_edition_group.unwrap() {
        return Err(Error::TorrentEditionGroupNotMatchingRequestedOne);
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
    .fetch_one(pool)
    .await?;

    if is_torrent_request_filled.unwrap() {
        return Err(Error::TorrentRequestAlreadyFilled);
    }

    #[derive(Debug)]
    struct BountySummary {
        total_upload: i64,
        total_bonus: i64,
    }

    let bounties_summary = query_as!(
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
    .fetch_one(pool)
    .await?;

    // Calculate the share for each user (50% each).
    // Ensure floating-point division for accurate half-shares, then cast back to i64 for database.
    let upload_share = (bounties_summary.total_upload as f32 / 2.0).round() as i32;
    let bonus_share = (bounties_summary.total_bonus as f32 / 2.0).round() as i32;

    let torrent_uploader_id: i64 = query_scalar!(
        r#"
                SELECT created_by_id FROM torrents WHERE id = $1
            "#,
        torrent_id
    )
    .fetch_one(pool)
    .await?;

    let mut tx = pool.begin().await?;

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

pub async fn search_torrent_requests(
    pool: &PgPool,
    title_group_name: Option<&str>,
    tags: Option<&[String]>,
    page: i64,
    page_size: i64,
) -> Result<Value> {
    let offset = (page - 1).max(0) * page_size;
    let rows: Option<Value> = sqlx::query_scalar!(
        r#"
        SELECT json_agg(data) as data FROM (
            SELECT json_build_object(
                'torrent_request', tr,
                'edition_group', json_build_object(
                    'id', eg.id,
                    'name', eg.name,
                    'release_date', eg.release_date,
                    'distributor', eg.distributor,
                    'source', eg.source,
                    'additional_information', eg.additional_information
                ),
                'title_group', json_build_object(
                    'id', tg.id,
                    'name', tg.name,
                    'content_type', tg.content_type,
                    'original_release_date', tg.original_release_date,
                    'covers', tg.covers,
                    'edition_groups', '[]',
                    'platform', tg.platform
                )
            ) as data
            FROM torrent_requests tr
            JOIN edition_groups eg ON tr.edition_group_id = eg.id
            JOIN title_groups tg ON eg.title_group_id = tg.id
            WHERE ($1::TEXT IS NULL OR tg.name ILIKE $1 OR $1 = ANY(tg.name_aliases))
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
    .fetch_one(pool)
    .await
    .map_err(Error::CouldNotSearchForTorrentRequests)?;
    Ok(rows.unwrap())
}
