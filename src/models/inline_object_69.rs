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
pub struct InlineObject69 {
    /// Unique identifier for the target chat
    #[serde(rename = "chat_id")]
    pub chat_id: i32,
    /// Short name of the game, serves as the unique identifier for the game. Set up your games via [Botfather](https://t.me/botfather).
    #[serde(rename = "game_short_name")]
    pub game_short_name: String,
    /// Sends the message [silently](https://telegram.org/blog/channels-2-0#silent-messages). Users will receive a notification with no sound.
    #[serde(rename = "disable_notification", skip_serializing_if = "Option::is_none")]
    pub disable_notification: Option<bool>,
    /// If the message is a reply, ID of the original message
    #[serde(rename = "reply_to_message_id", skip_serializing_if = "Option::is_none")]
    pub reply_to_message_id: Option<i32>,
    /// Pass *True*, if the message should be sent even if the specified replied-to message is not found
    #[serde(rename = "allow_sending_without_reply", skip_serializing_if = "Option::is_none")]
    pub allow_sending_without_reply: Option<bool>,
    #[serde(rename = "reply_markup", skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<crate::models::InlineKeyboardMarkup>,
}

impl InlineObject69 {
    pub fn new(chat_id: i32, game_short_name: String) -> InlineObject69 {
        InlineObject69 {
            chat_id,
            game_short_name,
            disable_notification: None,
            reply_to_message_id: None,
            allow_sending_without_reply: None,
            reply_markup: None,
        }
    }
}

