use crate::{
    connection_pool::ConnectionPool,
    models::artist::{
        AffiliatedArtist, AffiliatedArtistHierarchy, Artist, ArtistLite,
        UserCreatedAffiliatedArtist, UserCreatedArtist,
    },
};
use arcadia_common::error::{Error, Result};
use serde_json::Value;
use sqlx::PgPool;
use std::borrow::Borrow;

impl ConnectionPool {
    pub async fn create_artists(
        &self,
        artists: &Vec<UserCreatedArtist>,
        current_user_id: i64,
    ) -> Result<Vec<Artist>> {
        let mut tx = <ConnectionPool as Borrow<PgPool>>::borrow(self)
            .begin()
            .await?;

        let mut created_artists = Vec::new();

        for artist in artists {
            let artist = sqlx::query_as!(
                Artist,
                r#"
                INSERT INTO artists (name, description, pictures, created_by_id)
                VALUES ($1, $2, $3, $4)
                ON CONFLICT (name) DO UPDATE SET
                    -- This is a no-op update that still triggers RETURNING
                    name = EXCLUDED.name
                RETURNING *
                "#,
                artist.name,
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
        current_user_id: i64,
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
            "INSERT INTO affiliated_artists (title_group_id, artist_id, roles, nickname, created_by_id) VALUES {} RETURNING *",
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

        let created_affiliations = q_insert
            .fetch_all(self.borrow())
            .await
            .map_err(Error::CouldNotCreateArtistAffiliation)?;

        let artist_ids: Vec<i64> = created_affiliations
            .iter()
            .map(|aff| aff.artist_id)
            .collect();

        let fetched_artists: Vec<Artist> = sqlx::query_as!(
            Artist,
            r#"
        SELECT * FROM artists WHERE id = ANY($1)
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

    pub async fn find_artist_publications(&self, artist_id: &i64) -> Result<Value> {
        let artist_publications = sqlx::query!(
            r#"
            WITH artist_group_data AS (
                SELECT
                    aa.artist_id,
                    COALESCE(jsonb_agg(tgd.title_group_data), '[]'::jsonb) AS title_groups
                FROM affiliated_artists aa
                JOIN get_title_groups_and_edition_group_and_torrents_lite() AS tgd ON aa.title_group_id = tgd.title_group_id
                WHERE aa.artist_id = $1
                GROUP BY aa.artist_id
            ),
            artist_torrent_requests AS (
                SELECT
                    aa.artist_id,
                    COALESCE(jsonb_agg(to_jsonb(tr)), '[]'::jsonb) AS torrent_requests
                FROM affiliated_artists aa
                JOIN torrent_requests tr ON aa.title_group_id = tr.title_group_id
                WHERE aa.artist_id = $1
                GROUP BY aa.artist_id
            )
            SELECT jsonb_build_object(
                'artist', to_jsonb(a),
                'title_groups', COALESCE(agd.title_groups, '[]'::jsonb),
                'torrent_requests', COALESCE(atr.torrent_requests, '[]'::jsonb)
            ) AS artist_data
            FROM artists a
            LEFT JOIN artist_group_data agd ON agd.artist_id = a.id
            LEFT JOIN artist_torrent_requests atr ON atr.artist_id = a.id
            WHERE a.id = $1;
            "#,
            artist_id,
        )
        .fetch_one(self.borrow())
        .await?;

        Ok(artist_publications.artist_data.unwrap())
    }

    pub async fn find_artists_lite(&self, name: &String) -> Result<Vec<ArtistLite>> {
        let found_artists = sqlx::query_as!(
            ArtistLite,
            r#"
            SELECT name, id, pictures
            FROM artists
            WHERE LOWER(name) LIKE LOWER('%' || $1 || '%')
        "#,
            name
        )
        .fetch_all(self.borrow())
        .await
        .map_err(Error::CouldNotSearchForArtists)?;

        Ok(found_artists)
    }

    pub async fn delete_artists_affiliation(&self, affiliation_ids: &Vec<i64>) -> Result<()> {
        sqlx::query!(
            r#"
            DELETE FROM affiliated_artists
            WHERE id = ANY($1)
            "#,
            &affiliation_ids
        )
        .execute(self.borrow())
        .await?;

        Ok(())
    }
}
