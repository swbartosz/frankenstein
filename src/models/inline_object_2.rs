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
pub struct InlineObject2 {
    /// Pass *True* to drop all pending updates
    #[serde(rename = "drop_pending_updates", skip_serializing_if = "Option::is_none")]
    pub drop_pending_updates: Option<bool>,
}

impl InlineObject2 {
    pub fn new() -> InlineObject2 {
        InlineObject2 {
            drop_pending_updates: None,
        }
    }
}

