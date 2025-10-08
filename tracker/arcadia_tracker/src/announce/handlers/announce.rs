use actix_web::{
    dev,
    web::{Data, Path},
    FromRequest, HttpRequest, HttpResponse, ResponseError,
};

use crate::{
    announce::error::{AnnounceError, Result},
    Tracker,
};

#[utoipa::path(
    post,
    operation_id = "Announce",
    tag = "Announce",
    path = "/{passkey}/announce",
    responses(
        (status = 200, description = "Announce"),
    )
)]
pub async fn exec(arc: Data<Tracker>, passkey: Path<String>) /*-> Result<HttpResponse>*/
{
    let passkey = u128::from_str_radix(&passkey, 16).map_err(|_| AnnounceError::InvalidPassKey);
    //?;

    // Ok(HttpResponse::Ok())
}
