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
pub struct InlineResponse20019 {
    #[serde(rename = "ok")]
    pub ok: bool,
    #[serde(rename = "result")]
    pub result: Vec<crate::models::GameHighScore>,
}

impl InlineResponse20019 {
    pub fn new(ok: bool, result: Vec<crate::models::GameHighScore>) -> InlineResponse20019 {
        InlineResponse20019 {
            ok,
            result,
        }
    }
}

