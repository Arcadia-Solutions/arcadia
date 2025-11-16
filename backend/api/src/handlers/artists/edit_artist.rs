use crate::{middlewares::auth_middleware::Authdata, Arcadia};
use actix_web::{web::Data, web::Json, HttpResponse};
use arcadia_common::error::{Error, Result};
use arcadia_storage::models::artist::Artist;
use arcadia_storage::models::user::UserClass;
use arcadia_storage::{models::artist::UserEditedArtist, redis::RedisPoolInterface};

const GRACE_PERIOD_IN_DAYS: i64 = 7;

#[utoipa::path(
    put,
    operation_id = "Edit artist",
    tag = "Artist",
    path = "/api/artists",
    security(
        ("http" = ["Bearer"])
    ),
    responses(
        (status = 200, description = "Successfully edited the artist", body=Artist),
    )
)]
pub async fn exec<R: RedisPoolInterface + 'static>(
    form: Json<UserEditedArtist>,
    arc: Data<Arcadia<R>>,
    user: Authdata,
) -> Result<HttpResponse> {
    let mut artist = arc.pool.find_artist_by_id(form.id).await?;

    // Admins. can edit any artist, but other users can edit their own artist for a grace period of
    // 7 days after creation, to prevent e.g. hostile account takeovers.
    if user.class != UserClass::Staff {
        let grace_period = chrono::Utc::now() - chrono::Duration::days(GRACE_PERIOD_IN_DAYS);
        if artist.created_by_id != user.sub || artist.created_at < grace_period {
            return Err(Error::InsufficientPrivileges);
        }
    }

    artist = arc.pool.update_artist_data(&form).await?;

    Ok(HttpResponse::Ok().json(artist))
}
