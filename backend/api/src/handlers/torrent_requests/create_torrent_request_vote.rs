use crate::{handlers::User, Arcadia};
use actix_web::{
    web::{self, Data, Json},
    HttpResponse,
};
use arcadia_common::error::Result;
use arcadia_storage::{
    models::torrent_request_vote::{TorrentRequestVote, UserCreatedTorrentRequestVote},
    redis::RedisPoolInterface,
};

#[utoipa::path(
    post,
    operation_id = "Create torrent request vote",
    tag = "Torrent Request",
    path = "/api/torrent-requests/vote",
    security(
      ("http" = ["Bearer"])
    ),
    responses(
        (status = 200, description = "Successfully voted on the torrent_request", body=TorrentRequestVote),
    )
)]
pub async fn exec<R: RedisPoolInterface + 'static>(
    torrent_request_vote: Json<UserCreatedTorrentRequestVote>,
    arc: Data<Arcadia<R>>,
    current_user: User,
) -> Result<HttpResponse> {
    let vote = arc
        .pool
        .create_torrent_request_vote(&torrent_request_vote, &current_user)
        .await?;

    Ok(HttpResponse::Created().json(vote))
}
