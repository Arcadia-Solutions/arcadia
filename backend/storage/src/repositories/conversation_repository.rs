use crate::{
    connection_pool::ConnectionPool,
    models::{
        common::PaginatedResults,
        conversation::{
            Conversation, ConversationMessage, ConversationSearchQuery, ConversationSearchResult,
            UserCreatedConversation, UserCreatedConversationMessage,
        },
        notification::NotificationEvent,
    },
};
use arcadia_common::error::{Error, Result};
use serde_json::Value;
use std::borrow::Borrow;
use tokio::sync::broadcast;

impl ConnectionPool {
    pub async fn create_conversation(
        &self,
        conversation: &mut UserCreatedConversation,
        current_user_id: i32,
        notification_sender: &broadcast::Sender<NotificationEvent>,
    ) -> Result<Conversation> {
        //TODO: make transactional
        let created_conversation = sqlx::query_as!(
            Conversation,
            r#"
                INSERT INTO conversations (subject, sender_id, receiver_id)
                VALUES ($1, $2, $3)
                RETURNING id, created_at, subject, sender_id, receiver_id, sender_last_seen_at, receiver_last_seen_at, locked
            "#,
            conversation.subject,
            current_user_id,
            conversation.receiver_id,
        )
        .fetch_one(self.borrow())
        .await
        .map_err(Error::CouldNotCreateConversation)?;

        conversation.first_message.conversation_id = created_conversation.id;
        self.create_conversation_message(
            &conversation.first_message,
            current_user_id,
            notification_sender,
        )
        .await?;

        Ok(created_conversation)
    }

    pub async fn create_conversation_message(
        &self,
        message: &UserCreatedConversationMessage,
        current_user_id: i32,
        notification_sender: &broadcast::Sender<NotificationEvent>,
    ) -> Result<ConversationMessage> {
        let result = sqlx::query_as!(
            ConversationMessage,
            r#"
                INSERT INTO conversation_messages (conversation_id, created_by_id, content)
                SELECT $1, $2, $3
                FROM conversations
                WHERE id = $1 AND NOT locked
                RETURNING id, conversation_id, created_at, created_by_id, content
            "#,
            message.conversation_id,
            current_user_id,
            message.content,
        )
        .fetch_optional(self.borrow())
        .await
        .map_err(Error::CouldNotCreateConversation)?;

        match result {
            Some(msg) => {
                let notification_info = sqlx::query!(
                    r#"
                    SELECT
                        CASE WHEN c.sender_id = $2 THEN c.receiver_id ELSE c.sender_id END as "other_user_id!",
                        CASE
                            WHEN prev_msg.created_by_id IS NULL THEN false
                            WHEN prev_msg.created_by_id != $2 THEN false
                            WHEN c.sender_id = $2 THEN
                                c.receiver_last_seen_at IS NULL OR c.receiver_last_seen_at < prev_msg.created_at
                            ELSE
                                c.sender_last_seen_at < prev_msg.created_at
                        END as "was_already_unread!"
                    FROM conversations c
                    LEFT JOIN LATERAL (
                        SELECT cm.created_at, cm.created_by_id
                        FROM conversation_messages cm
                        WHERE cm.conversation_id = c.id AND cm.id != $3
                        ORDER BY cm.created_at DESC
                        LIMIT 1
                    ) AS prev_msg ON TRUE
                    WHERE c.id = $1
                    "#,
                    message.conversation_id,
                    current_user_id,
                    msg.id
                )
                .fetch_one(self.borrow())
                .await
                .map_err(Error::CouldNotFindConversation)?;

                if !notification_info.was_already_unread {
                    let _ = notification_sender.send(NotificationEvent::Conversation {
                        user_ids: vec![notification_info.other_user_id],
                    });
                }

                Ok(msg)
            }
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
                    Some(false) => Err(Error::CouldNotCreateConversationMessage(
                        sqlx::Error::RowNotFound,
                    )),
                    None => Err(Error::CouldNotFindConversation(sqlx::Error::RowNotFound)),
                }
            }
        }
    }

    pub async fn search_conversations(
        &self,
        user_id: i32,
        query: &ConversationSearchQuery,
    ) -> Result<PaginatedResults<ConversationSearchResult>> {
        let limit = query.page_size as i64;
        let offset = (query.page - 1) as i64 * query.page_size as i64;
        let search_term = query.search_term.as_deref().filter(|s| !s.is_empty());

        let results = sqlx::query_as!(
            ConversationSearchResult,
            r#"
            SELECT
                c.id AS conversation_id,
                c.created_at AS conversation_created_at,
                c.subject,
                c.sender_id,
                c.receiver_id,
                c.sender_last_seen_at,
                c.receiver_last_seen_at,
                c.locked,
                co.id AS correspondant_id,
                co.username AS correspondant_username,
                co.warned AS correspondant_warned,
                co.banned AS correspondant_banned,
                lm.created_at AS last_message_created_at,
                lm_user.id AS last_message_created_by_id,
                lm_user.username AS last_message_created_by_username
            FROM conversations AS c
            JOIN LATERAL (
                SELECT cm.created_at, cm.created_by_id
                FROM conversation_messages AS cm
                WHERE cm.conversation_id = c.id
                ORDER BY cm.created_at DESC
                LIMIT 1
            ) AS lm ON TRUE
            JOIN users AS lm_user ON lm.created_by_id = lm_user.id
            JOIN users AS co ON (CASE WHEN c.sender_id = $1 THEN c.receiver_id ELSE c.sender_id END) = co.id
            WHERE
                (c.sender_id = $1 OR c.receiver_id = $1)
                AND (
                    $4::TEXT IS NULL
                    OR c.subject ILIKE '%' || $4 || '%'
                    OR co.username ILIKE '%' || $4 || '%'
                    OR (NOT $5 AND EXISTS (
                        SELECT 1 FROM conversation_messages cm
                        WHERE cm.conversation_id = c.id
                        AND cm.content ILIKE '%' || $4 || '%'
                    ))
                )
            ORDER BY lm.created_at DESC
            LIMIT $2 OFFSET $3
            "#,
            user_id,
            limit,
            offset,
            search_term,
            query.search_titles_only
        )
        .fetch_all(self.borrow())
        .await
        .map_err(Error::CouldNotSearchConversations)?;

        let total_results = sqlx::query_scalar!(
            r#"
            SELECT COUNT(*)
            FROM conversations AS c
            JOIN users AS co ON (CASE WHEN c.sender_id = $1 THEN c.receiver_id ELSE c.sender_id END) = co.id
            WHERE
                (c.sender_id = $1 OR c.receiver_id = $1)
                AND (
                    $2::TEXT IS NULL
                    OR c.subject ILIKE '%' || $2 || '%'
                    OR co.username ILIKE '%' || $2 || '%'
                    OR (NOT $3 AND EXISTS (
                        SELECT 1 FROM conversation_messages cm
                        WHERE cm.conversation_id = c.id
                        AND cm.content ILIKE '%' || $2 || '%'
                    ))
                )
            "#,
            user_id,
            search_term,
            query.search_titles_only
        )
        .fetch_one(self.borrow())
        .await
        .map_err(Error::CouldNotSearchConversations)?
        .unwrap_or(0);

        Ok(PaginatedResults {
            results,
            total_items: total_results,
            page: query.page,
            page_size: query.page_size,
        })
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
                        'class_name', s.class_name,
                        'custom_title', s.custom_title,
                        'banned', s.banned,
                        'avatar', s.avatar,
                        'warned', s.warned
                    ),
                    'receiver', json_build_object(
                        'id', r.id,
                        'username', r.username,
                        'class_name', r.class_name,
                        'custom_title', r.custom_title,
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
                            'class_name', u_msg.class_name,
                            'custom_title', u_msg.custom_title,
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
                s.id, s.username, s.class_name, s.custom_title, s.banned, s.avatar, s.warned,
                r.id, r.username, r.class_name, r.custom_title, r.banned, r.avatar, r.warned;
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
