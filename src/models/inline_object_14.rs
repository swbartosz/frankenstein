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
pub struct InlineObject14 {
    /// Unique identifier for the target chat or username of the target channel (in the format `@channelusername`)
    #[serde(rename = "chat_id")]
    pub chat_id: crate::models::AnyOfintegerstring,
    /// Latitude of the location
    #[serde(rename = "latitude")]
    pub latitude: f32,
    /// Longitude of the location
    #[serde(rename = "longitude")]
    pub longitude: f32,
    /// The radius of uncertainty for the location, measured in meters; 0-1500
    #[serde(rename = "horizontal_accuracy", skip_serializing_if = "Option::is_none")]
    pub horizontal_accuracy: Option<f32>,
    /// Period in seconds for which the location will be updated (see [Live Locations](https://telegram.org/blog/live-locations), should be between 60 and 86400.
    #[serde(rename = "live_period", skip_serializing_if = "Option::is_none")]
    pub live_period: Option<i32>,
    /// For live locations, a direction in which the user is moving, in degrees. Must be between 1 and 360 if specified.
    #[serde(rename = "heading", skip_serializing_if = "Option::is_none")]
    pub heading: Option<i32>,
    /// For live locations, a maximum distance for proximity alerts about approaching another chat member, in meters. Must be between 1 and 100000 if specified.
    #[serde(rename = "proximity_alert_radius", skip_serializing_if = "Option::is_none")]
    pub proximity_alert_radius: Option<i32>,
    /// Sends the message [silently](https://telegram.org/blog/channels-2-0#silent-messages). Users will receive a notification with no sound.
    #[serde(rename = "disable_notification", skip_serializing_if = "Option::is_none")]
    pub disable_notification: Option<bool>,
    /// If the message is a reply, ID of the original message
    #[serde(rename = "reply_to_message_id", skip_serializing_if = "Option::is_none")]
    pub reply_to_message_id: Option<i32>,
    /// Pass *True*, if the message should be sent even if the specified replied-to message is not found
    #[serde(rename = "allow_sending_without_reply", skip_serializing_if = "Option::is_none")]
    pub allow_sending_without_reply: Option<bool>,
    /// Additional interface options. A JSON-serialized object for an [inline keyboard](https://core.telegram.org/bots#inline-keyboards-and-on-the-fly-updating), [custom reply keyboard](https://core.telegram.org/bots#keyboards), instructions to remove reply keyboard or to force a reply from the user.
    #[serde(rename = "reply_markup", skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<crate::models::AnyOfInlineKeyboardMarkupReplyKeyboardMarkupReplyKeyboardRemoveForceReply>,
}

impl InlineObject14 {
    pub fn new(chat_id: crate::models::AnyOfintegerstring, latitude: f32, longitude: f32) -> InlineObject14 {
        InlineObject14 {
            chat_id,
            latitude,
            longitude,
            horizontal_accuracy: None,
            live_period: None,
            heading: None,
            proximity_alert_radius: None,
            disable_notification: None,
            reply_to_message_id: None,
            allow_sending_without_reply: None,
            reply_markup: None,
        }
    }
}

