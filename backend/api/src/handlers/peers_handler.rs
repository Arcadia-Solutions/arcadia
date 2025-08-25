// use actix_web::{HttpResponse, web};

// use crate::{Arcadia, Result, handlers::UserId, models::peer::Peer, repositories::peer_repository};

// #[utoipa::path(
//     get,
//     path = "/api/me/peers",
//     responses(
//         (status = 200, description = "Successfully retrieved peer list", body=Vec<Peer>),
//     )
// )]
// pub async fn get_user_peers(
//     arc: web::Data<Arcadia>,
//     current_user_id: UserId,
// ) -> Result<HttpResponse> {
//     let peers = peer_repository::get_user_peers(&arc.pool, current_user_id.0).await;
//     Ok(HttpResponse::Ok().json(peers))
// }
