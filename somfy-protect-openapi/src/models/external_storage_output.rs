/*
 * Somfy Protect API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 3
 *
 * Generated by: https://openapi-generator.tech
 */

use crate::models;

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ExternalStorageOutput {
    /// External video provider name.
    #[serde(rename = "provider")]
    pub provider: String,
    /// Status.
    #[serde(rename = "status")]
    pub status: String,
    /// Error message.
    #[serde(rename = "errorMessage", skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
}

impl ExternalStorageOutput {
    pub fn new(provider: String, status: String) -> ExternalStorageOutput {
        ExternalStorageOutput {
            provider,
            status,
            error_message: None,
        }
    }
}
