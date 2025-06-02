use crate::{
    CONVERSATION_CACHE, ConversationMessage, ConversationSession, SessionId, config::APPCONFIG,
};
use chrono::Utc;

pub struct ConversationManager;

impl ConversationManager {
    /// 获取或创建对话会话
    pub async fn get_or_create_session(session_id: SessionId) -> ConversationSession {
        if let Some(session) = CONVERSATION_CACHE.get(&session_id).await {
            // 检查会话是否过期
            let timeout = APPCONFIG.cache.conversation_timeout_minutes.unwrap_or(10);
            if session.is_expired(timeout) {
                // 会话过期，创建新会话
                let max_history = APPCONFIG.cache.max_conversation_history.unwrap_or(20);
                let new_session = ConversationSession::new(max_history);
                CONVERSATION_CACHE
                    .insert(session_id.clone(), new_session.clone())
                    .await;
                new_session
            } else {
                session
            }
        } else {
            // 创建新会话
            let max_history = APPCONFIG.cache.max_conversation_history.unwrap_or(20);
            let new_session = ConversationSession::new(max_history);
            CONVERSATION_CACHE
                .insert(session_id, new_session.clone())
                .await;
            new_session
        }
    }

    /// 添加用户消息到会话
    pub async fn add_user_message(session_id: SessionId, content: String) {
        let mut session = Self::get_or_create_session(session_id.clone()).await;
        session.add_message("user".to_string(), content);
        CONVERSATION_CACHE.insert(session_id, session).await;
    }

    /// 添加带用户信息的用户消息到会话
    pub async fn add_user_message_with_info(
        session_id: SessionId,
        content: String,
        user_id: crate::UserId,
        username: Option<String>,
    ) {
        let mut session = Self::get_or_create_session(session_id.clone()).await;
        
        let message = ConversationMessage {
            role: "user".to_string(),
            content,
            timestamp: Utc::now(),
            user_id: Some(user_id),
            username,
        };

        session.messages.push_back(message);
        session.last_activity = Utc::now();

        // 保持历史记录数量在限制内
        while session.messages.len() > session.max_history {
            session.messages.pop_front();
        }

        CONVERSATION_CACHE.insert(session_id, session).await;
    }

    /// 添加助手回复到会话
    pub async fn add_assistant_message(session_id: SessionId, content: String) {
        let mut session = Self::get_or_create_session(session_id.clone()).await;
        session.add_message("assistant".to_string(), content);
        CONVERSATION_CACHE.insert(session_id, session).await;
    }

    /// 获取最近的对话历史
    pub async fn get_conversation_history(
        session_id: SessionId,
        limit: usize,
    ) -> Vec<ConversationMessage> {
        if let Some(session) = CONVERSATION_CACHE.get(&session_id).await {
            let timeout = APPCONFIG.cache.conversation_timeout_minutes.unwrap_or(10);
            if !session.is_expired(timeout) {
                return session.get_recent_messages(limit);
            }
        }
        Vec::new()
    }

    /// 获取特定用户在群聊中的发言历史
    pub async fn get_user_conversation_history(
        session_id: SessionId,
        user_id: crate::UserId,
        limit: usize,
    ) -> Vec<ConversationMessage> {
        if let Some(session) = CONVERSATION_CACHE.get(&session_id).await {
            let timeout = APPCONFIG.cache.conversation_timeout_minutes.unwrap_or(10);
            if !session.is_expired(timeout) {
                // 先收集所有匹配的消息
                let user_messages: Vec<ConversationMessage> = session
                    .messages
                    .iter()
                    .filter(|msg| {
                        // 过滤出特定用户的消息或助手回复
                        msg.role == "assistant" || msg.user_id == Some(user_id)
                    })
                    .cloned()
                    .collect();
                
                // 取最后limit条
                let len = user_messages.len();
                if len > limit {
                    user_messages.into_iter().skip(len - limit).collect()
                } else {
                    user_messages
                }
            } else {
                Vec::new()
            }
        } else {
            Vec::new()
        }
    }

    /// 清除用户的对话历史
    pub async fn clear_conversation(session_id: SessionId) {
        CONVERSATION_CACHE.remove(&session_id).await;
    }

    /// 获取会话的最后活动时间
    pub async fn get_last_activity(session_id: SessionId) -> Option<chrono::DateTime<Utc>> {
        CONVERSATION_CACHE
            .get(&session_id)
            .await
            .map(|session| session.last_activity)
    }

    /// 清理所有过期的会话
    pub async fn cleanup_expired_sessions() {
        // 注意: moka cache 会自动处理过期项，这里只是一个辅助方法
        // 在实际使用中，moka 的 time_to_idle 会自动清理过期项
    }

    /// 获取当前活跃会话数量
    pub async fn get_active_session_count() -> u64 {
        CONVERSATION_CACHE.entry_count()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::SessionId;

    #[tokio::test]
    async fn test_conversation_manager() {
        let session_id = SessionId::Private(123456);

        // 添加用户消息
        ConversationManager::add_user_message(session_id.clone(), "你好".to_string()).await;

        // 添加助手回复
        ConversationManager::add_assistant_message(
            session_id.clone(),
            "你好！有什么可以帮助你的吗？".to_string(),
        )
        .await;

        // 获取对话历史
        let history = ConversationManager::get_conversation_history(session_id.clone(), 10).await;
        assert_eq!(history.len(), 2);
        assert_eq!(history[0].role, "user");
        assert_eq!(history[0].content, "你好");
        assert_eq!(history[1].role, "assistant");
        assert_eq!(history[1].content, "你好！有什么可以帮助你的吗？");
    }
}
