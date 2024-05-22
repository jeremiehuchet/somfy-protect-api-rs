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
pub struct ValidateEmail200Response {
    /// Is email valid?
    #[serde(rename = "email_is_valid", skip_serializing_if = "Option::is_none")]
    pub email_is_valid: Option<bool>,
}

impl ValidateEmail200Response {
    pub fn new() -> ValidateEmail200Response {
        ValidateEmail200Response {
            email_is_valid: None,
        }
    }
}
