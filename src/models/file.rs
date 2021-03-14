/*
 * Telegram Bot API
 *
 * Auto-generated OpenAPI schema
 *
 * The version of the OpenAPI document: 5.1.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// File : This object represents a file ready to be downloaded. The file can be downloaded via the link `https://api.telegram.org/file/bot<token>/<file_path>`. It is guaranteed that the link will be valid for at least 1 hour. When the link expires, a new one can be requested by calling [getFile](https://core.telegram.org/bots/api/#getfile).  Maximum file size to download is 20 MB



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct File {
    /// Identifier for this file, which can be used to download or reuse the file
    #[serde(rename = "file_id")]
    pub file_id: String,
    /// Unique identifier for this file, which is supposed to be the same over time and for different bots. Can't be used to download or reuse the file.
    #[serde(rename = "file_unique_id")]
    pub file_unique_id: String,
    /// *Optional*. File size, if known
    #[serde(rename = "file_size", skip_serializing_if = "Option::is_none")]
    pub file_size: Option<i32>,
    /// *Optional*. File path. Use `https://api.telegram.org/file/bot<token>/<file_path>` to get the file.
    #[serde(rename = "file_path", skip_serializing_if = "Option::is_none")]
    pub file_path: Option<String>,
}

impl File {
    /// This object represents a file ready to be downloaded. The file can be downloaded via the link `https://api.telegram.org/file/bot<token>/<file_path>`. It is guaranteed that the link will be valid for at least 1 hour. When the link expires, a new one can be requested by calling [getFile](https://core.telegram.org/bots/api/#getfile).  Maximum file size to download is 20 MB
    pub fn new(file_id: String, file_unique_id: String) -> File {
        File {
            file_id,
            file_unique_id,
            file_size: None,
            file_path: None,
        }
    }
}

