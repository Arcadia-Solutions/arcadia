use crate::models::artist::UserEditedArtist;
use crate::{
    connection_pool::ConnectionPool,
    models::{
        artist::{
            AffiliatedArtist, AffiliatedArtistHierarchy, Artist, ArtistAndTitleGroupsLite,
            ArtistLite, UserCreatedAffiliatedArtist, UserCreatedArtist,
        },
        torrent::{TorrentSearch, TorrentSearchOrderByColumn, TorrentSearchOrderByDirection},
    },
};
use arcadia_common::error::{Error, Result};
use sqlx::PgPool;
use std::borrow::Borrow;

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

    pub async fn find_artist_publications(
        &self,
        artist_id: &i64,
    ) -> Result<ArtistAndTitleGroupsLite> {
        let artist = sqlx::query_as!(
            Artist,
            r#"
            SELECT *
            FROM artists
            WHERE id = $1
            "#,
            artist_id
        )
        .fetch_one(self.borrow())
        .await?;

        let torrent_search_form = TorrentSearch {
            artist_id: Some(artist_id.to_owned()),
            title_group_include_empty_groups: true,
            title_group_name: None,
            torrent_created_by_id: None,
            torrent_reported: None,
            torrent_snatched_by_id: None,
            torrent_staff_checked: None,
            order_by_direction: TorrentSearchOrderByDirection::Desc,
            order_by_column: TorrentSearchOrderByColumn::TitleGroupOriginalReleaseDate,
            collage_id: None,
            page: 1,
            page_size: i64::MAX,
        };

        let search_results = self.search_torrents(&torrent_search_form, None).await?;

        Ok(ArtistAndTitleGroupsLite {
            artist,
            title_groups: search_results.results,
        })
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

    pub async fn find_artist_by_id(&self, artist_id: i64) -> Result<Artist> {
        sqlx::query_as!(
            Artist,
            r#"
                SELECT *
                FROM artists
                WHERE id = $1;
            "#,
            artist_id
        )
        .fetch_one(self.borrow())
        .await
        .map_err(Error::CouldNotFindArtist)
    }

    pub async fn update_artist_data(&self, updated_artist: &UserEditedArtist) -> Result<Artist> {
        sqlx::query_as!(
            Artist,
            r#"
                UPDATE artists
                SET name = $1, description = $2, pictures = $3
                WHERE id = $4
                RETURNING *
            "#,
            updated_artist.name,
            updated_artist.description,
            &updated_artist.pictures,
            updated_artist.id
        )
        .fetch_one(self.borrow())
        .await
        .map_err(Error::CouldNotUpdateArtist)
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
