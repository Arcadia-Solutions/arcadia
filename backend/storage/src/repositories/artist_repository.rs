use crate::models::artist::EditedArtist;
use crate::{
    connection_pool::ConnectionPool,
    models::{
        artist::{
            AffiliatedArtist, AffiliatedArtistHierarchy, Artist, ArtistLite, ArtistSearchResult,
            SearchArtistsQuery, UserCreatedAffiliatedArtist, UserCreatedArtist,
        },
        common::PaginatedResults,
    },
};
use arcadia_common::error::{Error, Result};
use sqlx::PgPool;
use std::borrow::Borrow;
use std::collections::HashMap;

impl ConnectionPool {
    pub async fn create_artists(
        &self,
        artists: &Vec<UserCreatedArtist>,
        current_user_id: i32,
    ) -> Result<Vec<Artist>> {
        let mut tx = <ConnectionPool as Borrow<PgPool>>::borrow(self)
            .begin()
            .await?;

        let mut created_artists = Vec::new();

        for artist in artists {
            let artist = sqlx::query_as!(
                Artist,
                r#"
                INSERT INTO artists (name, aliases, description, pictures, created_by_id)
                VALUES ($1, $2, $3, $4, $5)
                ON CONFLICT (name) DO UPDATE SET
                    -- This is a no-op update that still triggers RETURNING
                    name = EXCLUDED.name
                RETURNING id, name, aliases, created_at, created_by_id, description, pictures, title_groups_amount, edition_groups_amount, torrents_amount, seeders_amount, leechers_amount, snatches_amount
                "#,
                artist.name,
                &artist.aliases,
                artist.description,
                &artist.pictures,
                current_user_id
            )
            .fetch_one(&mut *tx)
            .await
            .map_err(Error::CouldNotCreateArtist)?;

            created_artists.push(artist);
        }

        tx.commit().await?;

        Ok(created_artists)
    }

    pub async fn create_artists_affiliation(
        &self,
        artists: &Vec<UserCreatedAffiliatedArtist>,
        current_user_id: i32,
    ) -> Result<Vec<AffiliatedArtistHierarchy>> {
        let values: Vec<String> = (0..artists.len())
            .map(|i| {
                format!(
                    "(${}, ${}, ${}::artist_role_enum[], ${}, ${})",
                    i * 5 + 1,
                    i * 5 + 2,
                    i * 5 + 3,
                    i * 5 + 4,
                    i * 5 + 5
                )
            })
            .collect();

        let insert_query = format!(
            "INSERT INTO affiliated_artists (title_group_id, artist_id, roles, nickname, created_by_id) VALUES {} RETURNING id, title_group_id, artist_id, roles, nickname, created_at, created_by_id",
            values.join(", ")
        );

        let mut q_insert = sqlx::query_as::<_, AffiliatedArtist>(&insert_query);
        for artist in artists {
            q_insert = q_insert
                .bind(artist.title_group_id)
                .bind(artist.artist_id)
                .bind(&artist.roles)
                .bind(
                    artist
                        .nickname
                        .clone()
                        .map(|nick| if nick.is_empty() { None } else { Some(nick) }),
                )
                .bind(current_user_id);
        }

        let created_affiliations = q_insert.fetch_all(self.borrow()).await.map_err(|e| {
            if let sqlx::Error::Database(ref db_err) = e
                && db_err.code().as_deref() == Some("23505")
            {
                return Error::DuplicateArtistAffiliation;
            }
            Error::CouldNotCreateArtistAffiliation(e)
        })?;

        // Update title_groups_amount for each affected artist
        for affiliation in &created_affiliations {
            sqlx::query!(
                r#"
                UPDATE artists
                SET title_groups_amount = title_groups_amount + 1
                WHERE id = $1
                "#,
                affiliation.artist_id,
            )
            .execute(self.borrow())
            .await?;
        }

        let artist_ids: Vec<i64> = created_affiliations
            .iter()
            .map(|aff| aff.artist_id)
            .collect();

        let fetched_artists: Vec<Artist> = sqlx::query_as!(
            Artist,
            r#"
        SELECT id, name, aliases, created_at, created_by_id, description, pictures, title_groups_amount, edition_groups_amount, torrents_amount, seeders_amount, leechers_amount, snatches_amount FROM artists WHERE id = ANY($1)
        "#,
            &artist_ids
        )
        .fetch_all(self.borrow())
        .await
        .unwrap();

        let mut affiliated_artist_hierarchies: Vec<AffiliatedArtistHierarchy> = Vec::new();

        for affiliation in created_affiliations {
            if let Some(artist) = fetched_artists
                .iter()
                .find(|a| a.id == affiliation.artist_id)
            {
                affiliated_artist_hierarchies.push(AffiliatedArtistHierarchy {
                    id: affiliation.id,
                    title_group_id: affiliation.title_group_id,
                    artist_id: affiliation.artist_id,
                    roles: affiliation.roles,
                    nickname: affiliation.nickname,
                    created_at: affiliation.created_at,
                    created_by_id: affiliation.created_by_id,
                    artist: artist.clone(),
                });
            }
        }

        Ok(affiliated_artist_hierarchies)
    }

    pub async fn find_artists_lite(&self, name: &String, limit: i64) -> Result<Vec<ArtistLite>> {
        let found_artists = sqlx::query_as!(
            ArtistLite,
            r#"
            SELECT name, id, aliases, pictures
            FROM artists
            WHERE unaccent(name) ILIKE '%' || unaccent($1) || '%'
               OR EXISTS (
                   SELECT 1 FROM unnest(aliases) AS alias
                   WHERE unaccent(alias) ILIKE '%' || unaccent($1) || '%'
               )
            LIMIT $2
        "#,
            name,
            limit
        )
        .fetch_all(self.borrow())
        .await
        .map_err(Error::CouldNotSearchForArtists)?;

        Ok(found_artists)
    }

    pub async fn search_artists(
        &self,
        form: &SearchArtistsQuery,
    ) -> Result<PaginatedResults<ArtistSearchResult>> {
        let offset = (form.page - 1) * form.page_size;

        let total_items: i64 = sqlx::query_scalar!(
            r#"
            SELECT COUNT(*) FROM artists
            WHERE $1::TEXT IS NULL
               OR unaccent(name) ILIKE '%' || unaccent($1) || '%'
               OR EXISTS (
                   SELECT 1 FROM unnest(aliases) AS alias
                   WHERE unaccent(alias) ILIKE '%' || unaccent($1) || '%'
               )
            "#,
            form.name,
        )
        .fetch_one(self.borrow())
        .await
        .unwrap()
        .unwrap();

        let results = sqlx::query_as!(
            ArtistSearchResult,
            r#"
            SELECT id, name, aliases, created_at, created_by_id, pictures, title_groups_amount
            FROM artists
            WHERE $1::TEXT IS NULL
               OR unaccent(name) ILIKE '%' || unaccent($1) || '%'
               OR EXISTS (
                   SELECT 1 FROM unnest(aliases) AS alias
                   WHERE unaccent(alias) ILIKE '%' || unaccent($1) || '%'
               )
            ORDER BY
                CASE WHEN $4 = 'name' AND $5 = 'asc' THEN name END ASC,
                CASE WHEN $4 = 'name' AND $5 = 'desc' THEN name END DESC,
                CASE WHEN $4 = 'created_at' AND $5 = 'asc' THEN created_at END ASC,
                CASE WHEN $4 = 'created_at' AND $5 = 'desc' THEN created_at END DESC,
                CASE WHEN $4 = 'title_groups_amount' AND $5 = 'asc' THEN title_groups_amount END ASC,
                CASE WHEN $4 = 'title_groups_amount' AND $5 = 'desc' THEN title_groups_amount END DESC
            OFFSET $2 LIMIT $3
            "#,
            form.name,
            offset as i64,
            form.page_size as i64,
            form.order_by_column.to_string(),
            form.order_by_direction.to_string()
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

    pub async fn find_artist_by_id(&self, artist_id: i64) -> Result<Artist> {
        sqlx::query_as!(
            Artist,
            r#"
                SELECT id, name, aliases, created_at, created_by_id, description, pictures, title_groups_amount, edition_groups_amount, torrents_amount, seeders_amount, leechers_amount, snatches_amount
                FROM artists
                WHERE id = $1;
            "#,
            artist_id
        )
        .fetch_one(self.borrow())
        .await
        .map_err(Error::CouldNotFindArtist)
    }

    pub async fn find_artist_tags_by_id(&self, artist_id: i64) -> Result<HashMap<String, i64>> {
        let rows = sqlx::query!(
            r#"
                SELECT tgt.name, COUNT(*) AS "count!: i64"
                FROM affiliated_artists aa
                JOIN title_group_applied_tags tgat ON tgat.title_group_id = aa.title_group_id
                JOIN title_group_tags tgt ON tgt.id = tgat.tag_id
                WHERE aa.artist_id = $1 AND tgt.deleted_at IS NULL
                GROUP BY tgt.name
            "#,
            artist_id
        )
        .fetch_all(self.borrow())
        .await?;

        Ok(rows.into_iter().map(|r| (r.name, r.count)).collect())
    }

    pub async fn update_artist_data(&self, updated_artist: &EditedArtist) -> Result<Artist> {
        sqlx::query_as!(
            Artist,
            r#"
                UPDATE artists
                SET name = $1, aliases = $2, description = $3, pictures = $4
                WHERE id = $5
                RETURNING id, name, aliases, created_at, created_by_id, description, pictures, title_groups_amount, edition_groups_amount, torrents_amount, seeders_amount, leechers_amount, snatches_amount
            "#,
            updated_artist.name,
            &updated_artist.aliases,
            updated_artist.description,
            &updated_artist.pictures,
            updated_artist.id
        )
        .fetch_one(self.borrow())
        .await
        .map_err(Error::CouldNotUpdateArtist)
    }

    pub async fn update_artist_pictures(&self, artist_id: i64, pictures: &[String]) -> Result<()> {
        sqlx::query!(
            r#"UPDATE artists SET pictures = $1 WHERE id = $2"#,
            pictures,
            artist_id
        )
        .execute(self.borrow())
        .await
        .map_err(Error::CouldNotUpdateArtist)?;
        Ok(())
    }

    pub async fn delete_artists_affiliation(&self, affiliation_ids: &Vec<i64>) -> Result<()> {
        // Get artist_id for affiliations being deleted
        let affected_affiliations: Vec<(i64,)> = sqlx::query_as(
            r#"
            SELECT artist_id
            FROM affiliated_artists
            WHERE id = ANY($1)
            "#,
        )
        .bind(affiliation_ids)
        .fetch_all(self.borrow())
        .await?;

        sqlx::query!(
            r#"
            DELETE FROM affiliated_artists
            WHERE id = ANY($1)
            "#,
            &affiliation_ids
        )
        .execute(self.borrow())
        .await?;

        // Update title_groups_amount for each affected artist
        for (artist_id,) in affected_affiliations {
            sqlx::query!(
                r#"
                UPDATE artists
                SET title_groups_amount = title_groups_amount - 1
                WHERE id = $1
                "#,
                artist_id
            )
            .execute(self.borrow())
            .await?;
        }

        Ok(())
    }

    /// Aggregates `seeders`, `leechers` and `times_completed` from all torrents
    /// affiliated with each artist and stores the sums in the artists table.
    /// Returns the number of artist rows that were actually changed.
    pub async fn update_artist_peer_stats(&self) -> Result<u64> {
        let result = sqlx::query!(
            r#"
            WITH artist_stats AS (
                SELECT aa.artist_id,
                       COALESCE(SUM(t.seeders), 0)::INT AS seeders,
                       COALESCE(SUM(t.leechers), 0)::INT AS leechers,
                       COALESCE(SUM(t.times_completed), 0)::INT AS snatches
                FROM affiliated_artists aa
                JOIN edition_groups eg ON eg.title_group_id = aa.title_group_id
                JOIN torrents t ON t.edition_group_id = eg.id AND t.deleted_at IS NULL
                GROUP BY aa.artist_id
            )
            UPDATE artists a
            SET seeders_amount = COALESCE(s.seeders, 0),
                leechers_amount = COALESCE(s.leechers, 0),
                snatches_amount = COALESCE(s.snatches, 0)
            FROM artists a2
            LEFT JOIN artist_stats s ON s.artist_id = a2.id
            WHERE a.id = a2.id
              AND (a.seeders_amount != COALESCE(s.seeders, 0)
                OR a.leechers_amount != COALESCE(s.leechers, 0)
                OR a.snatches_amount != COALESCE(s.snatches, 0))
            "#
        )
        .execute(self.borrow())
        .await?;

        Ok(result.rows_affected())
    }

    pub async fn delete_artist(&self, artist_id: i64) -> Result<()> {
        sqlx::query!(
            r#"
            DELETE FROM artists
            WHERE id = $1
            "#,
            artist_id
        )
        .execute(self.borrow())
        .await
        .map_err(Error::CouldNotDeleteArtist)?;

        Ok(())
    }
}
