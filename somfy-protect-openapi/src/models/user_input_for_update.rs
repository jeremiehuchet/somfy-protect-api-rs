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
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct UserInputForUpdate {
    /// The photo identifier.
    #[serde(rename = "photo_id", skip_serializing_if = "Option::is_none")]
    pub photo_id: Option<String>,
    /// The user display name.
    #[serde(rename = "display_name", skip_serializing_if = "Option::is_none")]
    pub display_name: Option<serde_json::Value>,
    /// User password given for control.
    #[serde(rename = "control_pwd", skip_serializing_if = "Option::is_none")]
    pub control_pwd: Option<serde_json::Value>,
    /// User firstname.
    #[serde(rename = "firstname", skip_serializing_if = "Option::is_none")]
    pub firstname: Option<String>,
    /// User lastname.
    #[serde(rename = "lastname", skip_serializing_if = "Option::is_none")]
    pub lastname: Option<String>,
    /// User email.
    #[serde(rename = "username", skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
    /// User gender.
    #[serde(rename = "gender", skip_serializing_if = "Option::is_none")]
    pub gender: Option<Gender>,
    /// User locale.
    #[serde(rename = "locale", skip_serializing_if = "Option::is_none")]
    pub locale: Option<String>,
    /// User phone number.
    #[serde(rename = "phone", skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,
}

impl UserInputForUpdate {
    pub fn new() -> UserInputForUpdate {
        UserInputForUpdate {
            photo_id: None,
            display_name: None,
            control_pwd: None,
            firstname: None,
            lastname: None,
            username: None,
            gender: None,
            locale: None,
            phone: None,
        }
    }
}
/// User gender.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Gender {
    #[serde(rename = "F")]
    F,
    #[serde(rename = "M")]
    M,
}

impl Default for Gender {
    fn default() -> Gender {
        Self::F
    }
}
