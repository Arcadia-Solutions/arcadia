use crate::{
    Arcadia, Error, Result,
    models::{
        invitation::{Invitation, SentInvitation},
        user::User,
    },
    repositories::invitation_repository::create_invitation,
    services::email_service::EmailService,
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

    let created_invitation = create_invitation(&arc.pool, &invitation, current_user.id).await?;

    // Send invitation email
    if let Ok(email_service) = EmailService::new(&arc) {
        if let Err(e) = email_service
            .send_invitation_email(
                &invitation.receiver_email,
                &current_user.username,
                &created_invitation.invitation_key,
                &invitation.message,
            )
            .await
        {
            // Log the error but don't fail the invitation creation
            log::warn!(
                "Failed to send invitation email to {}: {}",
                invitation.receiver_email,
                e
            );
        }
    } else {
        log::warn!("Email service not configured, skipping invitation email");
    }

    Ok(HttpResponse::Created().json(created_invitation))
}
