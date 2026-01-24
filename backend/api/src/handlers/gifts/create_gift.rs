use crate::{middlewares::auth_middleware::Authdata, Arcadia};
use actix_web::{
    web::{Data, Json},
    HttpResponse,
};
use arcadia_common::error::{Error, Result};
use arcadia_storage::{
    models::gift::{Gift, UserCreatedGift},
    redis::RedisPoolInterface,
};

#[utoipa::path(
    post,
    operation_id = "Create gift",
    tag = "Gift",
    path = "/api/gifts",
    security(
      ("http" = ["Bearer"])
    ),
    responses(
        (status = 200, description = "Successfully sent the gift", body=Gift),
    )
)]
pub async fn exec<R: RedisPoolInterface + 'static>(
    gift: Json<UserCreatedGift>,
    arc: Data<Arcadia<R>>,
    user: Authdata,
) -> Result<HttpResponse> {
    let current_user = arc.pool.find_user_with_id(user.sub).await?;
    if current_user.bonus_points < gift.bonus_points {
        return Err(Error::NotEnoughBonusPointsAvailable);
    }
    if current_user.freeleech_tokens < gift.freeleech_tokens {
        return Err(Error::NotEnoughFreeleechTokensAvailable);
    }

    let gift = arc.pool.create_gift(&gift, user.sub).await?;

    // Send a notification message to the receiver from user ID 1
    let sender_url = arc
        .frontend_url
        .join(&format!("/user/{}", user.sub))
        .unwrap();

    let mut gift_items = Vec::new();
    if gift.bonus_points > 0 {
        gift_items.push(format!("{} bonus points", gift.bonus_points));
    }
    if gift.freeleech_tokens > 0 {
        gift_items.push(format!("{} freeleech tokens", gift.freeleech_tokens));
    }
    let gift_description = gift_items.join(" and ");

    let mut message_content = format!(
        "[url={}]{}[/url] has sent you a gift of {}!",
        sender_url.as_str(),
        current_user.username,
        gift_description
    );

    if !gift.message.is_empty() {
        message_content.push_str(&format!(
            "\n\nThey also left a message with it:\n\n{}",
            gift.message
        ));
    }

    if let Err(error) = arc
        .pool
        .send_batch_messages(
            1,
            &[gift.receiver_id],
            "You received a gift!",
            &message_content,
            false,
        )
        .await
    {
        log::error!(
            "Failed to send gift notification to user {}: {}",
            gift.receiver_id,
            error
        );
    }

    Ok(HttpResponse::Created().json(gift))
}
