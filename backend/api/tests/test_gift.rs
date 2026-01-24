pub mod common;
pub mod mocks;

use actix_web::http::StatusCode;
use actix_web::test;
use arcadia_storage::connection_pool::ConnectionPool;
use arcadia_storage::models::gift::{Gift, UserCreatedGift};
use common::{
    auth_header, call_and_read_body_json_with_status, create_test_app_and_login, TestUser,
};
use mocks::mock_redis::MockRedisPool;
use sqlx::PgPool;
use std::sync::Arc;

#[sqlx::test(
    fixtures("with_test_users", "with_test_gift_balance"),
    migrations = "../storage/migrations"
)]
async fn test_gift_updates_sender_and_receiver_balances_and_sends_message(pool: PgPool) {
    let pool = Arc::new(ConnectionPool::with_pg_pool(pool));

    // Get initial balances
    let sender_before = pool.find_user_with_id(100).await.unwrap();
    let receiver_before = pool.find_user_with_id(101).await.unwrap();

    assert_eq!(sender_before.bonus_points, 1000);
    assert_eq!(sender_before.freeleech_tokens, 10);
    assert_eq!(receiver_before.bonus_points, 500);
    assert_eq!(receiver_before.freeleech_tokens, 5);

    let (service, user) = create_test_app_and_login(
        Arc::clone(&pool),
        MockRedisPool::default(),
        TestUser::Standard,
    )
    .await;

    let gift = UserCreatedGift {
        message: "Test gift".into(),
        receiver_id: 101,
        bonus_points: 100,
        freeleech_tokens: 2,
    };

    let req = test::TestRequest::post()
        .uri("/api/gifts")
        .insert_header(("X-Forwarded-For", "10.10.4.88"))
        .insert_header(auth_header(&user.token))
        .set_json(&gift)
        .to_request();

    let created_gift: Gift =
        call_and_read_body_json_with_status(&service, req, StatusCode::CREATED).await;

    assert_eq!(created_gift.bonus_points, 100);
    assert_eq!(created_gift.freeleech_tokens, 2);
    assert_eq!(created_gift.sender_id, 100);
    assert_eq!(created_gift.receiver_id, 101);

    // Verify sender's balance was decremented
    let sender_after = pool.find_user_with_id(100).await.unwrap();
    assert_eq!(sender_after.bonus_points, 900);
    assert_eq!(sender_after.freeleech_tokens, 8);

    // Verify receiver's balance was incremented
    let receiver_after = pool.find_user_with_id(101).await.unwrap();
    assert_eq!(receiver_after.bonus_points, 600);
    assert_eq!(receiver_after.freeleech_tokens, 7);

    // Verify receiver got a message (from user ID 1)
    let conversations = pool.find_user_conversations(101).await.unwrap();
    let conversations_array = conversations.as_array().unwrap();

    assert!(!conversations_array.is_empty());

    let gift_conversation = conversations_array
        .iter()
        .find(|c| c["subject"].as_str() == Some("You received a gift!"))
        .expect("Gift notification conversation not found");

    assert_eq!(gift_conversation["sender_id"].as_i64(), Some(1));
    assert_eq!(gift_conversation["receiver_id"].as_i64(), Some(101));
}
