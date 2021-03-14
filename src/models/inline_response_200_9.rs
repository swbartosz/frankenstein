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
pub struct InlineResponse2009 {
    #[serde(rename = "ok")]
    pub ok: bool,
    #[serde(rename = "result")]
    pub result: File,
}

impl InlineResponse2009 {
    pub fn new(ok: bool, result: File) -> InlineResponse2009 {
        InlineResponse2009 {
            ok,
            result,
        }
    }
}

