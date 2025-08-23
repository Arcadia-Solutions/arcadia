use actix_web::{web, HttpResponse};
use arcadia_storage::models::title_group::{EditedTitleGroup, TitleGroup};

use crate::{handlers::User, Arcadia};
use arcadia_common::error::{Error, Result};

#[utoipa::path(
    put,
    path = "/api/title-group",
    responses(
        (status = 200, description = "Successfully edited the title group", body=TitleGroup),
    )
)]
pub async fn exec(
    form: web::Json<EditedTitleGroup>,
    arc: web::Data<Arcadia>,
    current_user: User,
) -> Result<HttpResponse> {
    let title_group = arc.pool.find_title_group(form.id).await?;

    if title_group.created_by_id == current_user.id || current_user.class == "staff" {
        let updated_title_group = arc.pool.update_title_group(&form, title_group.id).await?;
        Ok(HttpResponse::Ok().json(updated_title_group))
    } else {
        Err(Error::InsufficientPrivileges)
    }
}
