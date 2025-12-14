use crate::{
    connection_pool::ConnectionPool,
    models::{
        artist::ArtistLite,
        collage::{
            Collage, CollageCategory, CollageEntry, CollageEntryHierarchy, CollageLite,
            CollageSearchResult, CollageType, SearchCollagesLiteQuery, SearchCollagesQuery,
            UserCreatedCollage, UserCreatedCollageEntry,
        },
        common::PaginatedResults,
        edition_group::EditionGroupHierarchyLite,
        entity::EntityLite,
        master_group::MasterGroupLite,
        title_group::TitleGroupHierarchyLite,
        torrent::TorrentHierarchyLite,
    },
};
use arcadia_common::error::{Error, Result};
use sqlx::{query_as, query_as_unchecked, query_scalar, types::Json};
use std::{borrow::Borrow, collections::HashMap};

impl ConnectionPool {
    pub async fn create_collage(
        &self,
        collage: &UserCreatedCollage,
        user_id: i32,
    ) -> Result<Collage> {
        let collage = sqlx::query_as!(
            Collage,
            r#"
            INSERT INTO collage (name, cover, description, tags, category, collage_type, created_by_id)
            VALUES ($1, $2, $3, $4, $5, $6, $7)
            RETURNING id, created_at, created_by_id, name, cover, description, tags, category AS "category: CollageCategory", collage_type AS "collage_type: CollageType"
            "#,
            collage.name,
            collage.cover,
            collage.description,
            &collage.tags,
            collage.category as CollageCategory,
            collage.collage_type as CollageType,
            user_id
        )
        .fetch_one(self.borrow())
        .await?;

        Ok(collage)
    }

    pub async fn create_collage_entries(
        &self,
        entries: &[UserCreatedCollageEntry],
        user_id: i32,
    ) -> Result<Vec<CollageEntry>> {
        let mut created_entries = Vec::new();

        for entry in entries {
            let created_entry = sqlx::query_as!(
                CollageEntry,
                r#"
                INSERT INTO collage_entry (artist_id, entity_id, title_group_id, master_group_id, collage_id, note, created_by_id)
                VALUES ($1, $2, $3, $4, $5, $6, $7)
                RETURNING id, created_at, created_by_id, artist_id, entity_id, title_group_id, master_group_id, collage_id, note
                "#,
                entry.artist_id,
                entry.entity_id,
                entry.title_group_id,
                entry.master_group_id,
                entry.collage_id,
                entry.note,
                user_id
            )
            .fetch_one(self.borrow())
            .await
            .map_err(|e| Error::CouldNotCreateCollageEntry(e.to_string()))?;

            created_entries.push(created_entry);
        }

        Ok(created_entries)
    }

    pub async fn find_collage(&self, collage_id: i64) -> Result<Collage> {
        let collage = sqlx::query_as!(
            Collage,
            r#"
                SELECT
                    id,
                    created_at,
                    created_by_id,
                    name,
                    cover,
                    description,
                    tags,
                    category AS "category: CollageCategory",
                    collage_type AS "collage_type: CollageType"
                FROM collage
                WHERE id = $1
            "#,
            collage_id
        )
        .fetch_one(self.borrow())
        .await
        .map_err(|_| Error::CollageNotFound)?;

        Ok(collage)
    }

    pub async fn find_collage_entries(
        &self,
        collage_id: i64,
        page: u32,
        page_size: u32,
    ) -> Result<PaginatedResults<CollageEntryHierarchy>> {
        let offset = (page - 1) * page_size;

        let total_items: i64 = query_scalar!(
            "SELECT COUNT(*) FROM collage_entry WHERE collage_id = $1",
            collage_id
        )
        .fetch_one(self.borrow())
        .await?
        .unwrap_or(0);

        // Step 1: Query basic collage entries
        let entries = sqlx::query_as!(
            CollageEntry,
            r#"
            SELECT
                id, created_at, created_by_id, artist_id, entity_id,
                title_group_id, master_group_id, collage_id, note
            FROM collage_entry
            WHERE collage_id = $1
            ORDER BY created_at DESC
            OFFSET $2
            LIMIT $3
            "#,
            collage_id,
            offset as i64,
            page_size as i64
        )
        .fetch_all(self.borrow())
        .await?;

        // Collect IDs for batch queries
        let artist_ids: Vec<i64> = entries.iter().filter_map(|e| e.artist_id).collect();
        let entity_ids: Vec<i64> = entries.iter().filter_map(|e| e.entity_id).collect();
        let title_group_ids: Vec<i32> = entries.iter().filter_map(|e| e.title_group_id).collect();
        let master_group_ids: Vec<i32> = entries.iter().filter_map(|e| e.master_group_id).collect();

        // Step 2: Query related data in batch
        let artists: HashMap<i64, ArtistLite> = if !artist_ids.is_empty() {
            sqlx::query_as!(
                ArtistLite,
                r#"SELECT id, name, pictures FROM artists WHERE id = ANY($1)"#,
                &artist_ids
            )
            .fetch_all(self.borrow())
            .await?
            .into_iter()
            .map(|a| (a.id, a))
            .collect()
        } else {
            HashMap::new()
        };

        let entities: HashMap<i64, EntityLite> = if !entity_ids.is_empty() {
            sqlx::query_as!(
                EntityLite,
                r#"SELECT id, name, pictures FROM entities WHERE id = ANY($1)"#,
                &entity_ids
            )
            .fetch_all(self.borrow())
            .await?
            .into_iter()
            .map(|e| (e.id, e))
            .collect()
        } else {
            HashMap::new()
        };

        let master_groups: HashMap<i32, MasterGroupLite> = if !master_group_ids.is_empty() {
            sqlx::query_as!(
                MasterGroupLite,
                r#"SELECT id, name FROM master_groups WHERE id = ANY($1)"#,
                &master_group_ids
            )
            .fetch_all(self.borrow())
            .await?
            .into_iter()
            .map(|m| (m.id, m))
            .collect()
        } else {
            HashMap::new()
        };

        // Step 3: Query title groups with their hierarchy (following torrent_repository pattern)
        let mut title_groups: HashMap<i32, TitleGroupHierarchyLite> = if !title_group_ids.is_empty()
        {
            let tgs = sqlx::query_as!(
                TitleGroupHierarchyLite,
                r#"
                SELECT
                    tg.id,
                    tg.name,
                    tg.covers,
                    tg.category AS "category: _",
                    tg.content_type AS "content_type: _",
                    tg.original_release_date,
                    tg.platform AS "platform: _",
                    COALESCE(
                        (SELECT array_agg(tgt.name) FROM title_group_tags tgt
                         JOIN title_group_applied_tags tgat ON tgt.id = tgat.tag_id
                         WHERE tgat.title_group_id = tg.id),
                        '{}'
                    ) AS "tags!",
                    '[]'::jsonb AS "edition_groups!: _",
                    '[]'::jsonb AS "affiliated_artists!: _"
                FROM title_groups tg
                WHERE tg.id = ANY($1)
                "#,
                &title_group_ids
            )
            .fetch_all(self.borrow())
            .await?;

            tgs.into_iter().map(|tg| (tg.id, tg)).collect()
        } else {
            HashMap::new()
        };

        // Step 4: Query edition groups for title groups
        if !title_group_ids.is_empty() {
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

            let edition_group_ids: Vec<i32> = edition_groups.iter().map(|eg| eg.id).collect();

            // Step 5: Query torrents for edition groups
            let torrents: Vec<TorrentHierarchyLite> = if !edition_group_ids.is_empty() {
                sqlx::query_as!(
                    TorrentHierarchyLite,
                    r#"
                    SELECT
                        id,
                        edition_group_id,
                        created_at,
                        release_name,
                        release_group,
                        container,
                        size,
                        audio_codec AS "audio_codec: _",
                        audio_bitrate,
                        audio_bitrate_sampling AS "audio_bitrate_sampling: _",
                        audio_channels AS "audio_channels: _",
                        video_codec AS "video_codec: _",
                        features AS "features!: _",
                        video_resolution AS "video_resolution: _",
                        video_resolution_other_x,
                        video_resolution_other_y,
                        duration,
                        languages AS "languages!: _",
                        subtitle_languages AS "subtitle_languages!: _",
                        extras AS "extras!: _",
                        seeders,
                        leechers,
                        snatched,
                        times_completed,
                        staff_checked,
                        trumpable,
                        upload_factor,
                        download_factor,
                        '[]'::jsonb AS "reports!: _"
                    FROM torrents
                    WHERE edition_group_id = ANY($1)
                    "#,
                    &edition_group_ids
                )
                .fetch_all(self.borrow())
                .await?
            } else {
                Vec::new()
            };

            // Group torrents by edition_group_id
            let mut torrents_by_eg: HashMap<i32, Vec<TorrentHierarchyLite>> = HashMap::new();
            for torrent in torrents {
                torrents_by_eg
                    .entry(torrent.edition_group_id)
                    .or_default()
                    .push(torrent);
            }

            // Group edition groups by title_group_id with their torrents
            let mut egs_by_tg: HashMap<i32, Vec<EditionGroupHierarchyLite>> = HashMap::new();
            for mut eg in edition_groups {
                eg.torrents = Json(torrents_by_eg.remove(&eg.id).unwrap_or_default());
                egs_by_tg.entry(eg.title_group_id).or_default().push(eg);
            }

            // Assign edition groups to title groups
            for (tg_id, tg) in title_groups.iter_mut() {
                tg.edition_groups = Json(egs_by_tg.remove(tg_id).unwrap_or_default());
            }
        }

        // Step 6: Query affiliated artists for title groups
        if !title_group_ids.is_empty() {
            #[derive(sqlx::FromRow)]
            struct AffiliatedArtistRow {
                title_group_id: i32,
                artist_id: i64,
                artist_name: String,
            }

            let affiliated = sqlx::query_as!(
                AffiliatedArtistRow,
                r#"
                SELECT aa.title_group_id, aa.artist_id, a.name AS artist_name
                FROM affiliated_artists aa
                JOIN artists a ON aa.artist_id = a.id
                WHERE aa.title_group_id = ANY($1)
                "#,
                &title_group_ids
            )
            .fetch_all(self.borrow())
            .await?;

            let mut artists_by_tg: HashMap<i32, Vec<crate::models::artist::AffiliatedArtistLite>> =
                HashMap::new();
            for row in affiliated {
                artists_by_tg.entry(row.title_group_id).or_default().push(
                    crate::models::artist::AffiliatedArtistLite {
                        artist_id: row.artist_id,
                        name: row.artist_name,
                    },
                );
            }

            for (tg_id, tg) in title_groups.iter_mut() {
                tg.affiliated_artists = Json(artists_by_tg.remove(tg_id).unwrap_or_default());
            }
        }

        // Step 7: Assemble CollageEntryHierarchy
        let results: Vec<CollageEntryHierarchy> = entries
            .into_iter()
            .map(|entry| CollageEntryHierarchy {
                id: entry.id,
                created_at: entry.created_at,
                created_by_id: entry.created_by_id,
                artist_id: entry.artist_id,
                artist: entry.artist_id.and_then(|id| artists.get(&id).cloned()),
                entity_id: entry.entity_id,
                entity: entry.entity_id.and_then(|id| entities.get(&id).cloned()),
                title_group_id: entry.title_group_id,
                title_group: entry.title_group_id.and_then(|id| title_groups.remove(&id)),
                master_group_id: entry.master_group_id,
                master_group: entry
                    .master_group_id
                    .and_then(|id| master_groups.get(&id).cloned()),
                collage_id: entry.collage_id,
                note: entry.note,
            })
            .collect();

        Ok(PaginatedResults {
            results,
            total_items,
            page,
            page_size,
        })
    }

    pub async fn search_collages(
        &self,
        form: &SearchCollagesQuery,
    ) -> Result<PaginatedResults<CollageSearchResult>> {
        let offset = (form.page - 1) * form.page_size;

        let total_items: i64 = query_scalar!(
            "
            SELECT COUNT(*)
            FROM collage c
            WHERE (c.name ILIKE '%' || $1 || '%')
            ",
            form.name.clone().unwrap_or_default(),
        )
        .fetch_one(self.borrow())
        .await?
        .unwrap_or(0);

        let results = query_as_unchecked!(
            CollageSearchResult,
            r#"
            SELECT
                c.id,
                c.created_at,
                c.created_by_id,
                jsonb_build_object('id', u.id, 'username', u.username, 'avatar', u.avatar) AS "created_by: _",
                c.name,
                c.cover,
                c.description,
                c.tags,
                c.category AS "category: CollageCategory",
                c.collage_type AS "collage_type: CollageType",
                COUNT(ce.id) AS "entries_amount!",
                MAX(ce.created_at) AS "last_entry_at"
            FROM collage c
            LEFT JOIN collage_entry ce ON c.id = ce.collage_id
            JOIN users u ON c.created_by_id = u.id
            WHERE (c.name ILIKE '%' || $1 || '%')
            GROUP BY c.id, u.id
            ORDER BY c.created_at DESC
            OFFSET $2 LIMIT $3
            "#,
            form.name.clone().unwrap_or_default(),
            offset as i64,
            form.page_size as i64,
        )
        .fetch_all(self.borrow())
        .await?;

        Ok(PaginatedResults {
            results,
            total_items,
            page: form.page,
            page_size: form.page_size,
        })
    }

    pub async fn search_collages_lite(
        &self,
        form: &SearchCollagesLiteQuery,
    ) -> Result<Vec<CollageLite>> {
        let results = query_as!(
            CollageLite,
            r#"
            SELECT
                c.id,
                c.name,
                c.cover,
                c.collage_type AS "collage_type: CollageType"
            FROM collage c
            WHERE c.name ILIKE '%' || $1 || '%'
            ORDER BY c.created_at DESC
            LIMIT $2
            "#,
            form.name,
            form.results_amount as i64,
        )
        .fetch_all(self.borrow())
        .await?;

        Ok(results)
    }
}
