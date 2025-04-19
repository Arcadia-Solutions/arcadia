use crate::{
    Arcadia, Error, Result,
    models::{
        invitation::{Invitation, SentInvitation},
        user::User,
    },
    repositories::invitation_repository::create_invitation,
};
use actix_web::{HttpResponse, web};

#[utoipa::path(
    post,
    path = "/api/invitation",
    responses(
        (status = 200, description = "Successfully sent the invitation", body=Invitation),
    )
)]
pub async fn send_invitation(
    invitation: web::Json<SentInvitation>,
    arc: web::Data<Arcadia>,
    current_user: User,
) -> Result<HttpResponse> {
    if current_user.invitations == 0 {
        return Err(Error::NoInvitationsAvailable);
    }

    // TODO: send email to the user who receives the invite

    let invitation = create_invitation(&arc.pool, &invitation, current_user.id).await?;

    Ok(HttpResponse::Created().json(invitation))
}
