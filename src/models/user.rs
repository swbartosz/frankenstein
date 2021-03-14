/*
 * Telegram Bot API
 *
 * Auto-generated OpenAPI schema
 *
 * The version of the OpenAPI document: 5.1.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// User : This object represents a Telegram user or bot.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct User {
    /// Unique identifier for this user or bot. This number may have more than 32 significant bits and some programming languages may have difficulty/silent defects in interpreting it. But it has at most 52 significant bits, so a 64-bit integer or double-precision float type are safe for storing this identifier.
    #[serde(rename = "id")]
    pub id: i32,
    /// True, if this user is a bot
    #[serde(rename = "is_bot")]
    pub is_bot: bool,
    /// User's or bot's first name
    #[serde(rename = "first_name")]
    pub first_name: String,
    /// *Optional*. User's or bot's last name
    #[serde(rename = "last_name", skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,
    /// *Optional*. User's or bot's username
    #[serde(rename = "username", skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
    /// *Optional*. [IETF language tag](https://en.wikipedia.org/wiki/IETF_language_tag) of the user's language
    #[serde(rename = "language_code", skip_serializing_if = "Option::is_none")]
    pub language_code: Option<String>,
    /// *Optional*. True, if the bot can be invited to groups. Returned only in [getMe](https://core.telegram.org/bots/api/#getme).
    #[serde(rename = "can_join_groups", skip_serializing_if = "Option::is_none")]
    pub can_join_groups: Option<bool>,
    /// *Optional*. True, if [privacy mode](https://core.telegram.org/bots#privacy-mode) is disabled for the bot. Returned only in [getMe](https://core.telegram.org/bots/api/#getme).
    #[serde(rename = "can_read_all_group_messages", skip_serializing_if = "Option::is_none")]
    pub can_read_all_group_messages: Option<bool>,
    /// *Optional*. True, if the bot supports inline queries. Returned only in [getMe](https://core.telegram.org/bots/api/#getme).
    #[serde(rename = "supports_inline_queries", skip_serializing_if = "Option::is_none")]
    pub supports_inline_queries: Option<bool>,
}

impl User {
    /// This object represents a Telegram user or bot.
    pub fn new(id: i32, is_bot: bool, first_name: String) -> User {
        User {
            id,
            is_bot,
            first_name,
            last_name: None,
            username: None,
            language_code: None,
            can_join_groups: None,
            can_read_all_group_messages: None,
            supports_inline_queries: None,
        }
    }
}

