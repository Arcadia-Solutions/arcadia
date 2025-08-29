use actix_web::{
    web::{Data, Json},
    HttpResponse,
};
use arcadia_storage::{
    models::title_group::{PublicRating, TitleGroup, UserCreatedTitleGroup},
    redis::RedisPoolInterface,
};
use futures::future::join_all;

use crate::{
    handlers::external_db::get_tmdb_data::get_tmdb_rating, middlewares::jwt_middleware::Authdata,
    Arcadia,
};
use arcadia_common::error::Result;

#[utoipa::path(
    post,
    operation_id = "Create title group",
    tag = "Title Group",
    path = "/api/title-groups",
    security(
      ("http" = ["Bearer"])
    ),
    responses(
        (status = 200, description = "Successfully created the title_group", body=TitleGroup),
    )
)]
pub async fn exec<R: RedisPoolInterface + 'static>(
    mut form: Json<UserCreatedTitleGroup>,
    arc: Data<Arcadia<R>>,
    user: Authdata,
) -> Result<HttpResponse> {
    let rating_futures: Vec<_> = form
        .external_links
        .iter()
        .filter(|link| link.contains("https://www.themoviedb.org/"))
        .map(|link| get_tmdb_rating(link, arc.tmdb_api_key.clone().unwrap()))
        .collect();
    let ratings: Vec<PublicRating> = join_all(rating_futures)
        .await
        .into_iter()
        .filter_map(Result::ok)
        .collect();

    let created_title_group = arc
        .pool
        .create_title_group(&form, &ratings, user.sub)
        .await?;

    if !form.affiliated_artists.is_empty() {
        for artist in &mut form.affiliated_artists {
            artist.title_group_id = created_title_group.id
        }

        let _ = arc
            .pool
            .create_artists_affiliation(&form.affiliated_artists, user.sub)
            .await?;
    }

    Ok(HttpResponse::Created().json(created_title_group))
}
