pub mod cancel_friend_request;
pub mod get_friends;
pub mod get_friendship_status;
pub mod get_pending_friend_requests;
pub mod get_sent_friend_requests;
pub mod remove_friendship;
pub mod respond_to_friend_request;
pub mod send_friend_request;

use actix_web::web;

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/friendships")
            .route("/send", web::post().to(send_friend_request::exec))
            .route("/respond", web::post().to(respond_to_friend_request::exec))
            .route(
                "/requests/received",
                web::get().to(get_pending_friend_requests::exec),
            )
            .route(
                "/requests/sent",
                web::get().to(get_sent_friend_requests::exec),
            )
            .route("/list", web::get().to(get_friends::exec))
            .route(
                "/status/{user_id}",
                web::get().to(get_friendship_status::exec),
            )
            .route(
                "/remove/{user_id}",
                web::delete().to(remove_friendship::exec),
            )
            .route(
                "/cancel/{request_id}",
                web::delete().to(cancel_friend_request::exec),
            ),
    );
}
