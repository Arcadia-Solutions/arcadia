use crate::{
    connection_pool::ConnectionPool,
    models::{
        friendship::{
            FriendRequest, FriendRequestStatus, FriendRequestWithUser, Friendship,
            FriendshipStatus, FriendshipWithUser, UserCreatedFriendRequest,
        },
        user::UserLite,
    },
};
use arcadia_common::error::{Error, Result};
use sqlx::PgPool;

impl ConnectionPool {
    /// Send a friend request
    pub async fn send_friend_request(
        &self,
        sender_id: i64,
        friend_request: &UserCreatedFriendRequest,
    ) -> Result<FriendRequest> {
        // Check if users are already friends
        if self
            .are_users_friends(sender_id, friend_request.receiver_id)
            .await?
        {
            return Err(Error::BadRequest("Users are already friends".to_string()));
        }

        // Check if there's already a pending request between these users
        let existing_request = sqlx::query_as!(
            FriendRequest,
            r#"
                SELECT id, sender_id, receiver_id, status as "status: FriendRequestStatus", message, created_at, updated_at
                FROM friend_requests
                WHERE (sender_id = $1 AND receiver_id = $2) OR (sender_id = $2 AND receiver_id = $1)
                AND status = 'pending'
            "#,
            sender_id,
            friend_request.receiver_id
        )
        .fetch_optional(&self.pool)
        .await?;

        if existing_request.is_some() {
            return Err(Error::BadRequest(
                "A friend request already exists between these users".to_string(),
            ));
        }

        // Create the friend request
        let created_request = sqlx::query_as!(
            FriendRequest,
            r#"
                INSERT INTO friend_requests (sender_id, receiver_id, message)
                VALUES ($1, $2, $3)
                RETURNING id, sender_id, receiver_id, status as "status: FriendRequestStatus", message, created_at, updated_at
            "#,
            sender_id,
            friend_request.receiver_id,
            friend_request.message
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(created_request)
    }

    /// Respond to a friend request (accept or reject)
    pub async fn respond_to_friend_request(
        &self,
        user_id: i64,
        friend_request_id: i64,
        accept: bool,
    ) -> Result<FriendRequest> {
        let status = if accept {
            FriendRequestStatus::Accepted
        } else {
            FriendRequestStatus::Rejected
        };

        // Update the friend request status
        let updated_request = sqlx::query_as!(
            FriendRequest,
            r#"
                UPDATE friend_requests
                SET status = $1, updated_at = NOW()
                WHERE id = $2 AND receiver_id = $3 AND status = 'pending'
                RETURNING id, sender_id, receiver_id, status as "status: FriendRequestStatus", message, created_at, updated_at
            "#,
            status as FriendRequestStatus,
            friend_request_id,
            user_id
        )
        .fetch_optional(&self.pool)
        .await?
        .ok_or_else(|| Error::NotFound("Friend request not found or already processed".to_string()))?;

        // If accepted, create a friendship
        if accept {
            let user1_id = std::cmp::min(updated_request.sender_id, updated_request.receiver_id);
            let user2_id = std::cmp::max(updated_request.sender_id, updated_request.receiver_id);

            sqlx::query!(
                r#"
                    INSERT INTO friendships (user1_id, user2_id)
                    VALUES ($1, $2)
                    ON CONFLICT (user1_id, user2_id) DO NOTHING
                "#,
                user1_id,
                user2_id
            )
            .execute(&self.pool)
            .await?;
        }

        Ok(updated_request)
    }

    /// Get pending friend requests for a user (received)
    pub async fn get_pending_friend_requests(&self, user_id: i64) -> Result<Vec<FriendRequestWithUser>> {
        let requests = sqlx::query!(
            r#"
                SELECT 
                    fr.id,
                    fr.sender_id,
                    fr.receiver_id,
                    fr.status as "status: FriendRequestStatus",
                    fr.message,
                    fr.created_at,
                    fr.updated_at,
                    sender.id as sender_user_id,
                    sender.username as sender_username,
                    sender.avatar as sender_avatar,
                    receiver.id as receiver_user_id,
                    receiver.username as receiver_username,
                    receiver.avatar as receiver_avatar
                FROM friend_requests fr
                JOIN users sender ON fr.sender_id = sender.id
                JOIN users receiver ON fr.receiver_id = receiver.id
                WHERE fr.receiver_id = $1 AND fr.status = 'pending'
                ORDER BY fr.created_at DESC
            "#,
            user_id
        )
        .fetch_all(&self.pool)
        .await?;

        let friend_requests = requests
            .into_iter()
            .map(|row| FriendRequestWithUser {
                id: row.id,
                sender: UserLite {
                    id: row.sender_user_id,
                    username: row.sender_username,
                    avatar: row.sender_avatar,
                },
                receiver: UserLite {
                    id: row.receiver_user_id,
                    username: row.receiver_username,
                    avatar: row.receiver_avatar,
                },
                status: row.status,
                message: row.message,
                created_at: row.created_at,
                updated_at: row.updated_at,
            })
            .collect();

        Ok(friend_requests)
    }

    /// Get sent friend requests for a user
    pub async fn get_sent_friend_requests(&self, user_id: i64) -> Result<Vec<FriendRequestWithUser>> {
        let requests = sqlx::query!(
            r#"
                SELECT 
                    fr.id,
                    fr.sender_id,
                    fr.receiver_id,
                    fr.status as "status: FriendRequestStatus",
                    fr.message,
                    fr.created_at,
                    fr.updated_at,
                    sender.id as sender_user_id,
                    sender.username as sender_username,
                    sender.avatar as sender_avatar,
                    receiver.id as receiver_user_id,
                    receiver.username as receiver_username,
                    receiver.avatar as receiver_avatar
                FROM friend_requests fr
                JOIN users sender ON fr.sender_id = sender.id
                JOIN users receiver ON fr.receiver_id = receiver.id
                WHERE fr.sender_id = $1
                ORDER BY fr.created_at DESC
            "#,
            user_id
        )
        .fetch_all(&self.pool)
        .await?;

        let friend_requests = requests
            .into_iter()
            .map(|row| FriendRequestWithUser {
                id: row.id,
                sender: UserLite {
                    id: row.sender_user_id,
                    username: row.sender_username,
                    avatar: row.sender_avatar,
                },
                receiver: UserLite {
                    id: row.receiver_user_id,
                    username: row.receiver_username,
                    avatar: row.receiver_avatar,
                },
                status: row.status,
                message: row.message,
                created_at: row.created_at,
                updated_at: row.updated_at,
            })
            .collect();

        Ok(friend_requests)
    }

    /// Get friends list for a user
    pub async fn get_user_friends(&self, user_id: i64) -> Result<Vec<FriendshipWithUser>> {
        let friendships = sqlx::query!(
            r#"
                SELECT 
                    f.id,
                    f.created_at,
                    CASE 
                        WHEN f.user1_id = $1 THEN f.user2_id
                        ELSE f.user1_id
                    END as friend_id,
                    u.username as friend_username,
                    u.avatar as friend_avatar
                FROM friendships f
                JOIN users u ON (
                    CASE 
                        WHEN f.user1_id = $1 THEN f.user2_id
                        ELSE f.user1_id
                    END
                ) = u.id
                WHERE f.user1_id = $1 OR f.user2_id = $1
                ORDER BY f.created_at DESC
            "#,
            user_id
        )
        .fetch_all(&self.pool)
        .await?;

        let friends = friendships
            .into_iter()
            .map(|row| FriendshipWithUser {
                id: row.id,
                friend: UserLite {
                    id: row.friend_id,
                    username: row.friend_username,
                    avatar: row.friend_avatar,
                },
                created_at: row.created_at,
            })
            .collect();

        Ok(friends)
    }

    /// Check if two users are friends
    pub async fn are_users_friends(&self, user1_id: i64, user2_id: i64) -> Result<bool> {
        let min_id = std::cmp::min(user1_id, user2_id);
        let max_id = std::cmp::max(user1_id, user2_id);

        let friendship = sqlx::query!(
            r#"
                SELECT id FROM friendships
                WHERE user1_id = $1 AND user2_id = $2
            "#,
            min_id,
            max_id
        )
        .fetch_optional(&self.pool)
        .await?;

        Ok(friendship.is_some())
    }

    /// Get friendship status between two users
    pub async fn get_friendship_status(&self, user1_id: i64, user2_id: i64) -> Result<FriendshipStatus> {
        // Check if they are friends
        let are_friends = self.are_users_friends(user1_id, user2_id).await?;

        if are_friends {
            return Ok(FriendshipStatus {
                are_friends: true,
                pending_request: None,
            });
        }

        // Check for pending requests
        let pending_request = sqlx::query!(
            r#"
                SELECT 
                    fr.id,
                    fr.sender_id,
                    fr.receiver_id,
                    fr.status as "status: FriendRequestStatus",
                    fr.message,
                    fr.created_at,
                    fr.updated_at,
                    sender.id as sender_user_id,
                    sender.username as sender_username,
                    sender.avatar as sender_avatar,
                    receiver.id as receiver_user_id,
                    receiver.username as receiver_username,
                    receiver.avatar as receiver_avatar
                FROM friend_requests fr
                JOIN users sender ON fr.sender_id = sender.id
                JOIN users receiver ON fr.receiver_id = receiver.id
                WHERE ((fr.sender_id = $1 AND fr.receiver_id = $2) OR (fr.sender_id = $2 AND fr.receiver_id = $1))
                AND fr.status = 'pending'
            "#,
            user1_id,
            user2_id
        )
        .fetch_optional(&self.pool)
        .await?;

        let pending_request_with_user = pending_request.map(|row| FriendRequestWithUser {
            id: row.id,
            sender: UserLite {
                id: row.sender_user_id,
                username: row.sender_username,
                avatar: row.sender_avatar,
            },
            receiver: UserLite {
                id: row.receiver_user_id,
                username: row.receiver_username,
                avatar: row.receiver_avatar,
            },
            status: row.status,
            message: row.message,
            created_at: row.created_at,
            updated_at: row.updated_at,
        });

        Ok(FriendshipStatus {
            are_friends: false,
            pending_request: pending_request_with_user,
        })
    }

    /// Remove a friendship
    pub async fn remove_friendship(&self, user1_id: i64, user2_id: i64) -> Result<()> {
        let min_id = std::cmp::min(user1_id, user2_id);
        let max_id = std::cmp::max(user1_id, user2_id);

        let result = sqlx::query!(
            r#"
                DELETE FROM friendships
                WHERE user1_id = $1 AND user2_id = $2
            "#,
            min_id,
            max_id
        )
        .execute(&self.pool)
        .await?;

        if result.rows_affected() == 0 {
            return Err(Error::NotFound("Friendship not found".to_string()));
        }

        Ok(())
    }

    /// Cancel a sent friend request
    pub async fn cancel_friend_request(&self, sender_id: i64, friend_request_id: i64) -> Result<()> {
        let result = sqlx::query!(
            r#"
                DELETE FROM friend_requests
                WHERE id = $1 AND sender_id = $2 AND status = 'pending'
            "#,
            friend_request_id,
            sender_id
        )
        .execute(&self.pool)
        .await?;

        if result.rows_affected() == 0 {
            return Err(Error::NotFound(
                "Friend request not found or cannot be cancelled".to_string(),
            ));
        }

        Ok(())
    }
}