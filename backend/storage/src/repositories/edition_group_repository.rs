use crate::{
    connection_pool::ConnectionPool,
    models::edition_group::{EditionGroup, UserCreatedEditionGroup},
};
use arcadia_common::error::{Error, Result};
use std::borrow::Borrow;

impl ConnectionPool {
    pub async fn create_edition_group(
        &self,
        edition_group_form: &UserCreatedEditionGroup,
        current_user_id: i64,
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

        Ok(created_edition_group)
    }
}
