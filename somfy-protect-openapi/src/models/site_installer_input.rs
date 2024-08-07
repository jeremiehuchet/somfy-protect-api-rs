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
pub struct SiteInstallerInput {
    /// Additional Info.
    #[serde(rename = "additional_info", skip_serializing_if = "Option::is_none")]
    pub additional_info: Option<String>,
    /// Statement URL.
    #[serde(rename = "statement_url", skip_serializing_if = "Option::is_none")]
    pub statement_url: Option<String>,
    /// User Rights granted limit date.
    #[serde(
        rename = "user_rights_granted_until_at",
        skip_serializing_if = "Option::is_none"
    )]
    pub user_rights_granted_until_at: Option<String>,
}

impl SiteInstallerInput {
    pub fn new() -> SiteInstallerInput {
        SiteInstallerInput {
            additional_info: None,
            statement_url: None,
            user_rights_granted_until_at: None,
        }
    }
}
