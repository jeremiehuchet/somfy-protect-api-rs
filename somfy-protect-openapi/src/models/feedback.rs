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
pub struct Feedback {
    #[serde(rename = "message")]
    pub message: String,
    #[serde(rename = "os", skip_serializing_if = "Option::is_none")]
    pub os: Option<String>,
    #[serde(rename = "app", skip_serializing_if = "Option::is_none")]
    pub app: Option<String>,
    #[serde(rename = "phoneModel", skip_serializing_if = "Option::is_none")]
    pub phone_model: Option<String>,
    #[serde(rename = "from", skip_serializing_if = "Option::is_none")]
    pub from: Option<String>,
}

impl Feedback {
    pub fn new(message: String) -> Feedback {
        Feedback {
            message,
            os: None,
            app: None,
            phone_model: None,
            from: None,
        }
    }
}
