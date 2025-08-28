pub mod send_friend_request;
pub mod respond_to_friend_request;
pub mod get_friend_requests;
pub mod get_friends;
pub mod get_friendship_status;
pub mod remove_friendship;
pub mod cancel_friend_request;

use actix_web::web;

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/friendships")
            .route("/send", web::post().to(send_friend_request::send_friend_request))
            .route("/respond", web::post().to(respond_to_friend_request::respond_to_friend_request))
            .route("/requests/received", web::get().to(get_friend_requests::get_pending_friend_requests))
            .route("/requests/sent", web::get().to(get_friend_requests::get_sent_friend_requests))
            .route("/list", web::get().to(get_friends::get_user_friends))
            .route("/status/{user_id}", web::get().to(get_friendship_status::get_friendship_status))
            .route("/remove/{user_id}", web::delete().to(remove_friendship::remove_friendship))
            .route("/cancel/{request_id}", web::delete().to(cancel_friend_request::cancel_friend_request)),
    );
}