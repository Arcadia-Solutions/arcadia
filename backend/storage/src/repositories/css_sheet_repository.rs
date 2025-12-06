use crate::{
    connection_pool::ConnectionPool,
    models::css_sheet::{CssSheet, CssSheetsEnriched, EditedCssSheet, UserCreatedCssSheet},
};
use arcadia_common::error::{Error, Result};
use std::borrow::Borrow;

impl ConnectionPool {
    pub async fn create_css_sheet(
        &self,
        css_sheet: &UserCreatedCssSheet,
        current_user_id: i32,
    ) -> Result<CssSheet> {
        let css_sheet = sqlx::query_as!(
            CssSheet,
            r#"
                INSERT INTO css_sheets (name, css, created_by_id, preview_image_url)
                VALUES ($1, $2, $3, $4)
                RETURNING *
            "#,
            css_sheet.name.trim(),
            css_sheet.css,
            current_user_id,
            css_sheet.preview_image_url.trim()
        )
        .fetch_one(self.borrow())
        .await
        .map_err(Error::CouldNotCreateCssSheet)?;

        Ok(css_sheet)
    }

    pub async fn find_css_sheets(&self) -> Result<CssSheetsEnriched> {
        let sheets = sqlx::query_as!(
            CssSheet,
            r#"
                SELECT * FROM css_sheets
            "#,
        )
        .fetch_all(self.borrow())
        .await
        .map_err(Error::CouldNotFindCssSheets)?;

        let default_sheet_name = Self::find_default_css_sheet_name(self).await?;

        Ok(CssSheetsEnriched {
            css_sheets: sheets,
            default_sheet_name,
        })
    }

    pub async fn find_default_css_sheet_name(&self) -> Result<String> {
        let mut default_sheet_name = sqlx::query_scalar!(
            r#"
                    SELECT column_default
                    FROM information_schema.columns
                    WHERE table_name='users' AND column_name='css_sheet_name';
                "#,
        )
        .fetch_one(self.borrow())
        .await
        .map_err(Error::CouldNotFindCssSheets)?
        .unwrap();

        default_sheet_name = regex::Regex::new(r#"^'(.*)'::\s*character varying$"#)
            .unwrap()
            .captures(&default_sheet_name)
            .and_then(|caps| caps.get(1))
            .map(|m| m.as_str().to_string())
            .unwrap();

        Ok(default_sheet_name)
    }

    pub async fn find_css_sheet(&self, name: &str) -> Result<CssSheet> {
        let sheet = sqlx::query_as!(
            CssSheet,
            r#"
                SELECT * FROM css_sheets WHERE name = $1
            "#,
            name
        )
        .fetch_one(self.borrow())
        .await
        .map_err(Error::CssSheetNotFound)?;

        Ok(sheet)
    }

    pub async fn update_css_sheet(&self, form: &EditedCssSheet) -> Result<CssSheet> {
        let default_sheet_name = Self::find_default_css_sheet_name(self).await?;

        let css_sheet = sqlx::query_as!(
            CssSheet,
            r#"
                UPDATE css_sheets
                SET name = $2, css = $3, preview_image_url = $4
                WHERE name = $1
                RETURNING *
            "#,
            form.old_name,
            form.name,
            form.css,
            form.preview_image_url
        )
        .fetch_one(self.borrow())
        .await
        .map_err(Error::CssSheetNotFound)?;

        if form.old_name == default_sheet_name {
            Self::set_default_css_sheet(self, &form.name).await?;
        }

        Ok(css_sheet)
    }

    pub async fn set_default_css_sheet(&self, sheet_name: &str) -> Result<()> {
        let sql = format!(
            "ALTER TABLE users ALTER COLUMN css_sheet_name SET DEFAULT {}",
            sqlx::query_scalar!("SELECT quote_literal($1) AS quoted", sheet_name)
                .fetch_one(self.borrow())
                .await
                .map_err(Error::CouldNotUpdateDefaultCssSheet)?
                .unwrap()
        );

        sqlx::query(&sql)
            .execute(self.borrow())
            .await
            .map_err(Error::CouldNotUpdateDefaultCssSheet)?;

        Ok(())
    }

    pub async fn set_css_sheet_for_user(
        &self,
        user_id: i32,
        css_sheet_name: Option<String>,
    ) -> Result<()> {
        let _ = sqlx::query!(
            r#"
                UPDATE users
                SET css_sheet_name = $2
                WHERE id = $1
            "#,
            user_id,
            css_sheet_name
        )
        .execute(self.borrow())
        .await?;

        Ok(())
    }

    pub async fn get_css_sheet_content(&self, name: &str) -> Result<String> {
        let css = sqlx::query!(
            r#"
                SELECT css FROM css_sheets
                WHERE name = $1
            "#,
            name
        )
        .fetch_one(self.borrow())
        .await
        .map_err(Error::CssSheetNotFound)?;

        Ok(css.css)
    }
}
