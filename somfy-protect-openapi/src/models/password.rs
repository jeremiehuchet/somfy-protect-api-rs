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
pub struct Password {
    /// Old password.
    #[serde(rename = "old_password")]
    pub old_password: String,
    /// New password.
    #[serde(rename = "new_password")]
    pub new_password: String,
}

impl Password {
    pub fn new(old_password: String, new_password: String) -> Password {
        Password {
            old_password,
            new_password,
        }
    }
}
