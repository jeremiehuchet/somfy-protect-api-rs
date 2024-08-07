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
pub struct SiteInvitationRequest {
    #[serde(rename = "firstname", skip_serializing_if = "Option::is_none")]
    pub firstname: Option<String>,
    #[serde(rename = "lastname", skip_serializing_if = "Option::is_none")]
    pub lastname: Option<String>,
    /// The user display name.
    #[serde(rename = "display_name")]
    pub display_name: String,
    /// User email.
    #[serde(rename = "email")]
    pub email: String,
    /// The photo identifier.
    #[serde(rename = "photo_id", skip_serializing_if = "Option::is_none")]
    pub photo_id: Option<String>,
    /// Profiles of user for this site.
    #[serde(rename = "profiles")]
    pub profiles: Vec<models::UserSiteProfile>,
    /// User's phone number.
    #[serde(rename = "phone", skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,
}

impl SiteInvitationRequest {
    pub fn new(
        display_name: String,
        email: String,
        profiles: Vec<models::UserSiteProfile>,
    ) -> SiteInvitationRequest {
        SiteInvitationRequest {
            firstname: None,
            lastname: None,
            display_name,
            email,
            photo_id: None,
            profiles,
            phone: None,
        }
    }
}
