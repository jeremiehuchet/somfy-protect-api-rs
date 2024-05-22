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
pub struct SiteUserLinkOutput {
    #[serde(rename = "geofence", skip_serializing_if = "Option::is_none")]
    pub geofence: Option<Box<models::GeoFenceOutput>>,
    /// Profiles of user for this site.
    #[serde(rename = "profiles", skip_serializing_if = "Option::is_none")]
    pub profiles: Option<Vec<models::UserSiteProfile>>,
    /// Invitation status.
    #[serde(rename = "invitation_status", skip_serializing_if = "Option::is_none")]
    pub invitation_status: Option<InvitationStatus>,
    /// Site name for this user.
    #[serde(rename = "site_display_name", skip_serializing_if = "Option::is_none")]
    pub site_display_name: Option<String>,
    /// Status of presence's user.
    #[serde(
        rename = "display_my_presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub display_my_presence: Option<bool>,
    /// Access Code.
    #[serde(rename = "access_code", skip_serializing_if = "Option::is_none")]
    pub access_code: Option<String>,
    /// User id.
    #[serde(rename = "user_id", skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    /// User last access date to site's dashboard.
    #[serde(
        rename = "user_rights_granted_until_at",
        skip_serializing_if = "Option::is_none"
    )]
    pub user_rights_granted_until_at: Option<String>,
}

impl SiteUserLinkOutput {
    pub fn new() -> SiteUserLinkOutput {
        SiteUserLinkOutput {
            geofence: None,
            profiles: None,
            invitation_status: None,
            site_display_name: None,
            display_my_presence: None,
            access_code: None,
            user_id: None,
            user_rights_granted_until_at: None,
        }
    }
}
/// Invitation status.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum InvitationStatus {
    #[serde(rename = "pending")]
    Pending,
    #[serde(rename = "accepted")]
    Accepted,
    #[serde(rename = "declined")]
    Declined,
    #[serde(rename = "unrequired")]
    Unrequired,
    #[serde(rename = "disabled")]
    Disabled,
}

impl Default for InvitationStatus {
    fn default() -> InvitationStatus {
        Self::Pending
    }
}
