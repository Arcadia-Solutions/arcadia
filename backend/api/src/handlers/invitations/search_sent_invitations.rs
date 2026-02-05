use crate::{middlewares::auth_middleware::Authdata, Arcadia};
use actix_web::{
    web::{Data, Query},
    HttpResponse,
};
use arcadia_common::error::Result;
use arcadia_storage::{
    models::{
        common::PaginatedResults,
        invitation::{InvitationHierarchy, SearchSentInvitationsQuery},
    },
    redis::RedisPoolInterface,
};

#[utoipa::path(
    get,
    operation_id = "Search sent invitations",
    tag = "Invitation",
    path = "/api/invitations",
    params(SearchSentInvitationsQuery),
    security(
      ("http" = ["Bearer"])
    ),
    responses(
        (status = 200, description = "Successfully got the sent invitations", body = PaginatedResults<InvitationHierarchy>),
    )
)]
pub async fn exec<R: RedisPoolInterface + 'static>(
    query: Query<SearchSentInvitationsQuery>,
    arc: Data<Arcadia<R>>,
    user: Authdata,
) -> Result<HttpResponse> {
    let invitations = arc.pool.search_sent_invitations(&query, user.sub).await?;

    Ok(HttpResponse::Ok().json(invitations))
}
