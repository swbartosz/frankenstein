/*
 * Telegram Bot API
 *
 * Auto-generated OpenAPI schema
 *
 * The version of the OpenAPI document: 5.1.0
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct InlineObject24 {
    /// Unique identifier for the target group or username of the target supergroup or channel (in the format `@channelusername`)
    #[serde(rename = "chat_id")]
    pub chat_id: crate::models::AnyOfintegerstring,
    /// Unique identifier of the target user
    #[serde(rename = "user_id")]
    pub user_id: i32,
    /// Date when the user will be unbanned, unix time. If user is banned for more than 366 days or less than 30 seconds from the current time they are considered to be banned forever. Applied for supergroups and channels only.
    #[serde(rename = "until_date", skip_serializing_if = "Option::is_none")]
    pub until_date: Option<i32>,
    /// Pass *True* to delete all messages from the chat for the user that is being removed. If *False*, the user will be able to see messages in the group that were sent before the user was removed. Always *True* for supergroups and channels.
    #[serde(rename = "revoke_messages", skip_serializing_if = "Option::is_none")]
    pub revoke_messages: Option<bool>,
}

impl InlineObject24 {
    pub fn new(chat_id: crate::models::AnyOfintegerstring, user_id: i32) -> InlineObject24 {
        InlineObject24 {
            chat_id,
            user_id,
            until_date: None,
            revoke_messages: None,
        }
    }
}

