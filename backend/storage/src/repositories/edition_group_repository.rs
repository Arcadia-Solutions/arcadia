use crate::{
    connection_pool::ConnectionPool,
    models::edition_group::{EditedEditionGroup, EditionGroup, UserCreatedEditionGroup},
};
use arcadia_common::error::{Error, Result};
use std::borrow::Borrow;

impl ConnectionPool {
    pub async fn create_edition_group(
        &self,
        edition_group_form: &UserCreatedEditionGroup,
        current_user_id: i32,
    ) -> Result<EditionGroup> {
        const CREATE_EDITION_GROUPS_QUERY: &str = r#"
            INSERT INTO edition_groups (title_group_id, name, release_date, created_by_id, description, distributor, covers, external_links, source, additional_information)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9::source_enum, $10)
            RETURNING *;
        "#;

        let created_edition_group = sqlx::query_as::<_, EditionGroup>(CREATE_EDITION_GROUPS_QUERY)
            .bind(edition_group_form.title_group_id)
            .bind(&edition_group_form.name)
            .bind(edition_group_form.release_date)
            .bind(current_user_id)
            .bind(&edition_group_form.description)
            .bind(&edition_group_form.distributor)
            .bind(&edition_group_form.covers)
            .bind(&edition_group_form.external_links)
            .bind(&edition_group_form.source)
            .bind(&edition_group_form.additional_information)
            .fetch_one(self.borrow())
            .await
            .map_err(Error::CouldNotCreateEditionGroup)?;

        // Update edition_groups_amount for all affiliated artists of this title group
        sqlx::query!(
            r#"
            UPDATE artists
            SET edition_groups_amount = edition_groups_amount + 1
            WHERE id IN (
                SELECT DISTINCT artist_id
                FROM affiliated_artists
                WHERE title_group_id = $1
            )
            "#,
            edition_group_form.title_group_id
        )
        .execute(self.borrow())
        .await?;

        Ok(created_edition_group)
    }

    pub async fn find_edition_group(&self, edition_group_id: i32) -> Result<EditionGroup> {
        let edition_group = sqlx::query_as!(
            EditionGroup,
            r#"
            SELECT
                id, title_group_id, name, release_date,
                created_at, updated_at, created_by_id, description,
                distributor, covers AS "covers!: _", external_links AS "external_links!: _",
                source AS "source: _", additional_information
            FROM edition_groups
            WHERE id = $1
            "#,
            edition_group_id
        )
        .fetch_one(self.borrow())
        .await
        .map_err(|_| Error::EditionGroupNotFound)?;

        Ok(edition_group)
    }

    pub async fn update_edition_group(
        &self,
        edited_edition_group: &EditedEditionGroup,
    ) -> Result<EditionGroup> {
        let updated_edition_group = sqlx::query_as!(
            EditionGroup,
            r#"
            UPDATE edition_groups
            SET
                name = $2,
                release_date = $3,
                description = $4,
                distributor = $5,
                covers = $6,
                external_links = $7,
                source = $8::source_enum,
                additional_information = $9,
                updated_at = NOW()
            WHERE id = $1
            RETURNING
                id, title_group_id, name, release_date,
                created_at, updated_at, created_by_id, description,
                distributor, covers AS "covers!: _", external_links AS "external_links!: _",
                source AS "source: _", additional_information
            "#,
            edited_edition_group.id,
            edited_edition_group.name,
            edited_edition_group.release_date,
            edited_edition_group.description,
            edited_edition_group.distributor,
            edited_edition_group.covers.as_slice(),
            edited_edition_group.external_links.as_slice(),
            edited_edition_group.source as _,
            edited_edition_group.additional_information
        )
        .fetch_one(self.borrow())
        .await
        .map_err(|e| Error::ErrorWhileUpdatingEditionGroup(e.to_string()))?;

        Ok(updated_edition_group)
    }
}
