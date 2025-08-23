use crate::{handlers::User, services::email_service::EmailService, Arcadia};
use actix_web::{web, HttpResponse};
use arcadia_common::error::{Error, Result};
use arcadia_storage::models::invitation::{Invitation, SentInvitation};

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

    let created_invitation = arc
        .pool
        .create_invitation(&invitation, current_user.id)
        .await?;

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
