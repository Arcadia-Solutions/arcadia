use crate::{
    connection_pool::ConnectionPool,
    models::conversation::{
        Conversation, ConversationMessage, UserCreatedConversation, UserCreatedConversationMessage,
    },
};
use arcadia_common::error::{Error, Result};
use serde_json::Value;
use std::borrow::Borrow;

impl ConnectionPool {
    pub async fn create_conversation(
        &self,
        conversation: &mut UserCreatedConversation,
        current_user_id: i32,
    ) -> Result<Conversation> {
        //TODO: make transactional
        let created_conversation = sqlx::query_as!(
            Conversation,
            r#"
                INSERT INTO conversations (subject, sender_id, receiver_id)
                VALUES ($1, $2, $3)
                RETURNING *
            "#,
            conversation.subject,
            current_user_id,
            conversation.receiver_id,
        )
        .fetch_one(self.borrow())
        .await
        .map_err(Error::CouldNotCreateConversation)?;

        conversation.first_message.conversation_id = created_conversation.id;
        self.create_conversation_message(&conversation.first_message, current_user_id)
            .await?;

        Ok(created_conversation)
    }

    pub async fn create_conversation_message(
        &self,
        message: &UserCreatedConversationMessage,
        current_user_id: i32,
    ) -> Result<ConversationMessage> {
        let result = sqlx::query_as!(
            ConversationMessage,
            r#"
                INSERT INTO conversation_messages (conversation_id, created_by_id, content)
                SELECT $1, $2, $3
                FROM conversations
                WHERE id = $1 AND NOT locked
                RETURNING *
            "#,
            message.conversation_id,
            current_user_id,
            message.content,
        )
        .fetch_optional(self.borrow())
        .await
        .map_err(Error::CouldNotCreateConversation)?;

        match result {
            Some(msg) => Ok(msg),
            None => {
                // Check if conversation exists to differentiate between not found and locked
                let is_locked = sqlx::query_scalar!(
                    r#"SELECT locked FROM conversations WHERE id = $1"#,
                    message.conversation_id,
                )
                .fetch_optional(self.borrow())
                .await
                .map_err(Error::CouldNotFindConversation)?;

                match is_locked {
                    Some(true) => Err(Error::ConversationLocked),
                    Some(false) => Err(Error::BadRequest("".to_string())),
                    None => Err(Error::CouldNotFindConversation(sqlx::Error::RowNotFound)),
                }
            }
        }
    }

    pub async fn find_user_conversations(&self, user_id: i32) -> Result<Value> {
        let conversations = sqlx::query!(
            r#"
            SELECT
                COALESCE(
                    jsonb_agg(
                        jsonb_build_object(
                            'id', c.id,
                            'created_at', c.created_at,
                            'subject', c.subject,
                            'sender_id', c.sender_id,
                            'receiver_id', c.receiver_id,
                            'sender_last_seen_at', c.sender_last_seen_at,
                            'receiver_last_seen_at', c.receiver_last_seen_at,
                            'locked', c.locked,
                            'last_message', jsonb_build_object(
                                'created_at', lm.created_at,
                                'created_by', jsonb_build_object(
                                    'id', u.id,
                                    'username', u.username,
                                    'warned', u.warned,
                                    'banned', u.banned
                                )
                            ),
                            'correspondant', jsonb_build_object(
                                'id', co.id,
                                'username', co.username,
                                'warned', co.warned,
                                'banned', co.banned
                            )
                        )
                        ORDER BY lm.created_at DESC
                    ),
                    '[]'::jsonb
                ) AS conversations_json
            FROM
                conversations AS c
            JOIN LATERAL (
                SELECT
                    cm.created_at,
                    cm.created_by_id
                FROM
                    conversation_messages AS cm
                WHERE
                    cm.conversation_id = c.id
                ORDER BY
                    cm.created_at DESC
                LIMIT 1
            ) AS lm ON TRUE
            JOIN
                users AS u ON lm.created_by_id = u.id
            JOIN
                users AS co ON (CASE WHEN c.sender_id = $1 THEN c.receiver_id ELSE c.sender_id END) = co.id
            WHERE
                c.sender_id = $1 OR c.receiver_id = $1;
            "#,
            user_id,
        )
        .fetch_one(self.borrow())
        .await
        .map_err(Error::CouldNotFindConversations)?;

        Ok(conversations.conversations_json.unwrap())
    }

    pub async fn find_conversation(
        &self,
        conversation_id: i64,
        current_user_id: i32,
        update_last_seen_at: bool,
    ) -> Result<Value> {
        let conversation_with_messages = sqlx::query!(
            r#"
            SELECT
                json_build_object(
                    'id', c.id,
                    'created_at', c.created_at,
                    'subject', c.subject,
                    'sender_last_seen_at', c.sender_last_seen_at,
                    'receiver_last_seen_at', c.receiver_last_seen_at,
                    'locked', c.locked,
                    'sender', json_build_object(
                        'id', s.id,
                        'username', s.username,
                        'banned', s.banned,
                        'avatar', s.avatar,
                        'warned', s.warned
                    ),
                    'receiver', json_build_object(
                        'id', r.id,
                        'username', r.username,
                        'banned', r.banned,
                        'avatar', r.avatar,
                        'warned', r.warned
                    ),
                    'messages', json_agg(json_build_object(
                        'id', m.id,
                        'created_at', m.created_at,
                        'content', m.content,
                        'created_by', json_build_object(
                            'id', u_msg.id,
                            'username', u_msg.username,
                            'banned', u_msg.banned,
                            'avatar', u_msg.avatar,
                            'warned', u_msg.warned
                        )
                    ) ORDER BY m.created_at ASC)
                ) AS conversation_details
            FROM
                conversations c
            INNER JOIN
                users s ON c.sender_id = s.id
            INNER JOIN
                users r ON c.receiver_id = r.id
            INNER JOIN
                conversation_messages m ON c.id = m.conversation_id
            INNER JOIN
                users u_msg ON m.created_by_id = u_msg.id
            WHERE
                c.id = $1 AND (c.sender_id = $2 OR c.receiver_id = $2) -- prevent users from reading a conversation they're not part of
            GROUP BY
                c.id, c.created_at, c.subject, c.locked,
                s.id, s.username, s.banned, s.avatar, s.warned,
                r.id, r.username, r.banned, r.avatar, r.warned;
            "#,
            conversation_id,
            current_user_id
        )
        .fetch_one(self.borrow())
        .await
        .map_err(Error::CouldNotFindConversation)?;

        sqlx::query!(
            r#"
            UPDATE conversations
            SET
                sender_last_seen_at = CASE
                    WHEN sender_id = $2 THEN NOW()
                    ELSE sender_last_seen_at
                END,
                receiver_last_seen_at = CASE
                    WHEN receiver_id = $2 THEN NOW()
                    ELSE receiver_last_seen_at
                END
            WHERE
                id = $1 AND $3;
            "#,
            conversation_id,
            current_user_id,
            update_last_seen_at
        )
        .execute(self.borrow())
        .await?;

        Ok(conversation_with_messages.conversation_details.unwrap())
    }

    pub async fn find_unread_conversations_amount(&self, user_id: i32) -> Result<u32> {
        let amount = sqlx::query_scalar!(
            r#"
            SELECT
                COUNT(c.id)
            FROM
                conversations c
            JOIN LATERAL (
                SELECT
                    cm.created_at,
                    cm.created_by_id
                FROM
                    conversation_messages cm
                WHERE
                    cm.conversation_id = c.id
                ORDER BY
                    cm.created_at DESC
                LIMIT 1
            ) AS lm ON TRUE
            WHERE
                lm.created_by_id != $1
                AND
                (
                    (c.sender_id = $1 AND (c.sender_last_seen_at < lm.created_at))
                    OR
                    (c.receiver_id = $1 AND (c.receiver_last_seen_at IS NULL OR c.receiver_last_seen_at < lm.created_at))
                );
            "#,
            user_id,
        )
        .fetch_one(self.borrow())
        .await
        .expect("error looking for unread conversations");

        Ok(amount.unwrap() as u32)
    }

    /// Sends a message from a sender to multiple recipients, creating a new conversation for each.
    pub async fn send_batch_messages(
        &self,
        sender_id: i32,
        recipient_ids: &[i32],
        subject: &str,
        content: &str,
        locked: bool,
    ) -> Result<()> {
        for &recipient_id in recipient_ids {
            let conversation = sqlx::query_scalar!(
                r#"
                INSERT INTO conversations (subject, sender_id, receiver_id, locked)
                VALUES ($1, $2, $3, $4)
                RETURNING id
                "#,
                subject,
                sender_id,
                recipient_id,
                locked
            )
            .fetch_one(self.borrow())
            .await
            .map_err(Error::CouldNotCreateConversation)?;

            sqlx::query!(
                r#"
                INSERT INTO conversation_messages (conversation_id, created_by_id, content)
                VALUES ($1, $2, $3)
                "#,
                conversation,
                sender_id,
                content
            )
            .execute(self.borrow())
            .await
            .map_err(Error::CouldNotCreateConversation)?;
        }

        Ok(())
    }
}
