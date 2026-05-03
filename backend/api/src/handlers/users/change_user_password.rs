use crate::{
    middlewares::auth_middleware::Authdata,
    services::auth_service::{validate_password, validate_password_verification},
    Arcadia,
};
use actix_web::{
    web::{Data, Json, Path},
    HttpRequest, HttpResponse,
};
use arcadia_common::error::{Error, Result};
use arcadia_storage::{
    models::user::{UserChangedPassword, UserPermission},
    redis::RedisPoolInterface,
};
use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};

#[utoipa::path(
    put,
    operation_id = "Change user password",
    tag = "User",
    path = "/api/users/{id}/password",
    security(("http" = ["Bearer"])),
    params(
        ("id" = i32, Path, description = "User ID")
    ),
    responses(
        (status = 200, description = "Successfully changed the user password"),
        (status = 400, description = "Invalid password or wrong current password"),
        (status = 403, description = "Insufficient privileges"),
        (status = 404, description = "User not found"),
    )
)]
pub async fn exec<R: RedisPoolInterface + 'static>(
    user_id: Path<i32>,
    form: Json<UserChangedPassword>,
    current_user: Authdata,
    arc: Data<Arcadia<R>>,
    req: HttpRequest,
) -> Result<HttpResponse> {
    let target_user_id = *user_id;
    let is_self_change = target_user_id == current_user.sub;

    if !is_self_change {
        arc.pool
            .require_permission(
                current_user.sub,
                &UserPermission::ChangeUserPassword,
                req.path(),
            )
            .await?;
    }

    let target_user = arc.pool.find_user_with_id(target_user_id).await?;

    if is_self_change {
        let current_password = form
            .current_password
            .as_ref()
            .ok_or(Error::WrongUsernameOrPassword)?;
        let parsed_hash = PasswordHash::new(&target_user.password_hash)
            .map_err(|_| Error::WrongUsernameOrPassword)?;
        Argon2::default()
            .verify_password(current_password.as_bytes(), &parsed_hash)
            .map_err(|_| Error::WrongUsernameOrPassword)?;
    }

    validate_password(&form.new_password)?;
    validate_password_verification(&form.new_password, &form.new_password_verify)?;

    let salt = SaltString::generate(&mut OsRng);
    let password_hash = Argon2::default()
        .hash_password(form.new_password.as_bytes(), &salt)
        .unwrap()
        .to_string();

    arc.pool
        .update_user_password_hash(target_user_id, &password_hash)
        .await?;

    Ok(HttpResponse::Ok().finish())
}
