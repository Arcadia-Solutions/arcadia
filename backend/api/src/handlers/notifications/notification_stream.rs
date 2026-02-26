use crate::{middlewares::auth_middleware, Arcadia};
use actix_web::web::{Data, Query};
use actix_web_lab::sse;
use arcadia_storage::redis::RedisPoolInterface;
use serde::Deserialize;
use std::time::Duration;
use tokio::sync::broadcast;

#[derive(Debug, Deserialize)]
pub struct NotificationStreamQuery {
    pub token: String,
}

pub async fn exec<R: RedisPoolInterface + 'static>(
    query: Query<NotificationStreamQuery>,
    arc: Data<Arcadia<R>>,
) -> Result<
    sse::Sse<impl futures::Stream<Item = Result<sse::Event, std::convert::Infallible>>>,
    actix_web::Error,
> {
    let user_id = auth_middleware::validate_token::<R>(&query.token, &arc).await?;

    let mut receiver = arc.notification_sender.subscribe();

    let stream = async_stream::stream! {
        loop {
            match receiver.recv().await {
                Ok(event) => {
                    if event.user_ids().contains(&user_id) {
                        yield Ok(sse::Event::Data(sse::Data::new(event.event_type())));
                    }
                }
                Err(broadcast::error::RecvError::Lagged(_)) => {
                    yield Ok(sse::Event::Data(sse::Data::new("refresh")));
                }
                Err(broadcast::error::RecvError::Closed) => {
                    break;
                }
            }
        }
    };

    Ok(sse::Sse::from_stream(stream).with_keep_alive(Duration::from_secs(30)))
}
