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
pub struct InlineObject65 {
    /// Unique identifier for the target private chat
    #[serde(rename = "chat_id")]
    pub chat_id: i32,
    /// Product name, 1-32 characters
    #[serde(rename = "title")]
    pub title: String,
    /// Product description, 1-255 characters
    #[serde(rename = "description")]
    pub description: String,
    /// Bot-defined invoice payload, 1-128 bytes. This will not be displayed to the user, use for your internal processes.
    #[serde(rename = "payload")]
    pub payload: String,
    /// Payments provider token, obtained via [Botfather](https://t.me/botfather)
    #[serde(rename = "provider_token")]
    pub provider_token: String,
    /// Unique deep-linking parameter that can be used to generate this invoice when used as a start parameter
    #[serde(rename = "start_parameter")]
    pub start_parameter: String,
    /// Three-letter ISO 4217 currency code, see [more on currencies](/bots/payments#supported-currencies)
    #[serde(rename = "currency")]
    pub currency: String,
    /// Price breakdown, a JSON-serialized list of components (e.g. product price, tax, discount, delivery cost, delivery tax, bonus, etc.)
    #[serde(rename = "prices")]
    pub prices: Vec<crate::models::LabeledPrice>,
    /// A JSON-serialized data about the invoice, which will be shared with the payment provider. A detailed description of required fields should be provided by the payment provider.
    #[serde(rename = "provider_data", skip_serializing_if = "Option::is_none")]
    pub provider_data: Option<String>,
    /// URL of the product photo for the invoice. Can be a photo of the goods or a marketing image for a service. People like it better when they see what they are paying for.
    #[serde(rename = "photo_url", skip_serializing_if = "Option::is_none")]
    pub photo_url: Option<String>,
    /// Photo size
    #[serde(rename = "photo_size", skip_serializing_if = "Option::is_none")]
    pub photo_size: Option<i32>,
    /// Photo width
    #[serde(rename = "photo_width", skip_serializing_if = "Option::is_none")]
    pub photo_width: Option<i32>,
    /// Photo height
    #[serde(rename = "photo_height", skip_serializing_if = "Option::is_none")]
    pub photo_height: Option<i32>,
    /// Pass *True*, if you require the user's full name to complete the order
    #[serde(rename = "need_name", skip_serializing_if = "Option::is_none")]
    pub need_name: Option<bool>,
    /// Pass *True*, if you require the user's phone number to complete the order
    #[serde(rename = "need_phone_number", skip_serializing_if = "Option::is_none")]
    pub need_phone_number: Option<bool>,
    /// Pass *True*, if you require the user's email address to complete the order
    #[serde(rename = "need_email", skip_serializing_if = "Option::is_none")]
    pub need_email: Option<bool>,
    /// Pass *True*, if you require the user's shipping address to complete the order
    #[serde(rename = "need_shipping_address", skip_serializing_if = "Option::is_none")]
    pub need_shipping_address: Option<bool>,
    /// Pass *True*, if user's phone number should be sent to provider
    #[serde(rename = "send_phone_number_to_provider", skip_serializing_if = "Option::is_none")]
    pub send_phone_number_to_provider: Option<bool>,
    /// Pass *True*, if user's email address should be sent to provider
    #[serde(rename = "send_email_to_provider", skip_serializing_if = "Option::is_none")]
    pub send_email_to_provider: Option<bool>,
    /// Pass *True*, if the final price depends on the shipping method
    #[serde(rename = "is_flexible", skip_serializing_if = "Option::is_none")]
    pub is_flexible: Option<bool>,
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

impl InlineObject65 {
    pub fn new(chat_id: i32, title: String, description: String, payload: String, provider_token: String, start_parameter: String, currency: String, prices: Vec<crate::models::LabeledPrice>) -> InlineObject65 {
        InlineObject65 {
            chat_id,
            title,
            description,
            payload,
            provider_token,
            start_parameter,
            currency,
            prices,
            provider_data: None,
            photo_url: None,
            photo_size: None,
            photo_width: None,
            photo_height: None,
            need_name: None,
            need_phone_number: None,
            need_email: None,
            need_shipping_address: None,
            send_phone_number_to_provider: None,
            send_email_to_provider: None,
            is_flexible: None,
            disable_notification: None,
            reply_to_message_id: None,
            allow_sending_without_reply: None,
            reply_markup: None,
        }
    }
}

