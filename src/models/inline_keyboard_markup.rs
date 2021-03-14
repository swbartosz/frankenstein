/*
 * Telegram Bot API
 *
 * Auto-generated OpenAPI schema
 *
 * The version of the OpenAPI document: 5.1.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// InlineKeyboardMarkup : This object represents an [inline keyboard](https://core.telegram.org/bots#inline-keyboards-and-on-the-fly-updating) that appears right next to the message it belongs to.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct InlineKeyboardMarkup {
    /// Array of button rows, each represented by an Array of [InlineKeyboardButton](https://core.telegram.org/bots/api/#inlinekeyboardbutton) objects
    #[serde(rename = "inline_keyboard")]
    pub inline_keyboard: Vec<Vec<crate::models::InlineKeyboardButton>>,
}

impl InlineKeyboardMarkup {
    /// This object represents an [inline keyboard](https://core.telegram.org/bots#inline-keyboards-and-on-the-fly-updating) that appears right next to the message it belongs to.
    pub fn new(inline_keyboard: Vec<Vec<crate::models::InlineKeyboardButton>>) -> InlineKeyboardMarkup {
        InlineKeyboardMarkup {
            inline_keyboard,
        }
    }
}

