use crate::{middlewares::jwt_middleware::Authdata, Arcadia};
use actix_web::{web, HttpResponse};
use arcadia_common::error::Result;
use arcadia_storage::models::edition_group::{EditionGroup, UserCreatedEditionGroup};

#[utoipa::path(
    post,
    operation_id = "Create edition group",
    tag = "Edition Group",
    path = "/api/edition-groups",
    security(
      ("http" = ["Bearer"])
    ),
    responses(
        (status = 200, description = "Successfully created the edition_group", body=EditionGroup),
    )
)]
pub async fn exec(
    form: web::Json<UserCreatedEditionGroup>,
    arc: web::Data<Arcadia>,
    user: Authdata,
) -> Result<HttpResponse> {
    let edition_group = arc.pool.create_edition_group(&form, user.sub).await?;

    Ok(HttpResponse::Created().json(edition_group))
}
