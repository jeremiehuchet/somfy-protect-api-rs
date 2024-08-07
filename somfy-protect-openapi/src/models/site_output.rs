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
pub struct SiteOutput {
    /// Site identifier.
    #[serde(rename = "site_id")]
    pub site_id: String,
    /// Site security level.
    #[serde(rename = "security_level")]
    pub security_level: SecurityLevel,
    #[serde(rename = "invitation")]
    pub invitation: Box<models::SiteInvitationOutput>,
    #[serde(rename = "alarm")]
    pub alarm: Box<models::SiteAlarm>,
    #[serde(rename = "diagnosis", skip_serializing_if = "Option::is_none")]
    pub diagnosis: Option<Box<models::SiteDiagnosis>>,
    /// Install completed flag.
    #[serde(rename = "install_completed", skip_serializing_if = "Option::is_none")]
    pub install_completed: Option<bool>,
    /// Brand.
    #[serde(rename = "brand", skip_serializing_if = "Option::is_none")]
    pub brand: Option<serde_json::Value>,
    /// Is a Plug configuration currently being backed up?
    #[serde(rename = "restoration_active")]
    pub restoration_active: bool,
    /// Privacy current status.
    #[serde(rename = "privacy_active")]
    pub privacy_active: bool,
    /// Read only status.
    #[serde(rename = "read_only")]
    pub read_only: bool,
    /// HCS site identifier.
    #[serde(rename = "hcs_site_id", skip_serializing_if = "Option::is_none")]
    pub hcs_site_id: Option<String>,
    /// ILO site identifier.
    #[serde(rename = "ilo_site_id", skip_serializing_if = "Option::is_none")]
    pub ilo_site_id: Option<String>,
    /// Site origin.
    #[serde(rename = "origin", skip_serializing_if = "Option::is_none")]
    pub origin: Option<String>,
    /// Axa IT phone number.
    #[serde(
        rename = "axa_it_phone_number",
        skip_serializing_if = "Option::is_none"
    )]
    pub axa_it_phone_number: Option<String>,
    /// Axa IT phone number formatted.
    #[serde(
        rename = "axa_it_phone_number_displayed",
        skip_serializing_if = "Option::is_none"
    )]
    pub axa_it_phone_number_displayed: Option<String>,
    /// Features enabled.
    #[serde(rename = "features", skip_serializing_if = "Option::is_none")]
    pub features: Option<Vec<String>>,
    #[serde(
        rename = "domesticAlarmOutputCollection",
        skip_serializing_if = "Option::is_none"
    )]
    pub domestic_alarm_output_collection: Option<Box<models::DomesticAlarmOutputCollection>>,
    /// Bundle type for  ILO.
    #[serde(rename = "bundle_type", skip_serializing_if = "Option::is_none")]
    pub bundle_type: Option<BundleType>,
    #[serde(rename = "services")]
    pub services: Box<models::SiteServicesOutput>,
    /// Site label.
    #[serde(rename = "label", skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    /// Site Services Version.
    #[serde(rename = "servicesVersion", skip_serializing_if = "Option::is_none")]
    pub services_version: Option<i32>,
    /// Is site subscription active.
    #[serde(
        rename = "subscription_active",
        skip_serializing_if = "Option::is_none"
    )]
    pub subscription_active: Option<bool>,
    /// Timezone of the site location.
    #[serde(rename = "timezone", skip_serializing_if = "Option::is_none")]
    pub timezone: Option<String>,
    /// Name of the site location.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Address of the site location.
    #[serde(rename = "address1", skip_serializing_if = "Option::is_none")]
    pub address1: Option<String>,
    /// Address complement of the site location.
    #[serde(rename = "address2", skip_serializing_if = "Option::is_none")]
    pub address2: Option<String>,
    /// Post code of the site location.
    #[serde(rename = "zip_code", skip_serializing_if = "Option::is_none")]
    pub zip_code: Option<String>,
    /// City of the site location.
    #[serde(rename = "city", skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,
    /// Region/State of the site location.
    #[serde(rename = "region", skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
    /// Country code of the site location (uppercase two-letter ISO-3166-1 alpha-2 code).
    #[serde(rename = "country", skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    /// Cross street details.
    #[serde(rename = "crosstreet", skip_serializing_if = "Option::is_none")]
    pub crosstreet: Option<String>,
    /// Address complement.
    #[serde(rename = "complement", skip_serializing_if = "Option::is_none")]
    pub complement: Option<String>,
    /// Address latitude.
    #[serde(rename = "latitude", skip_serializing_if = "Option::is_none")]
    pub latitude: Option<String>,
    /// Address longitude.
    #[serde(rename = "longitude", skip_serializing_if = "Option::is_none")]
    pub longitude: Option<String>,
    /// Police phone number of the site area.
    #[serde(
        rename = "police_phone_number",
        skip_serializing_if = "Option::is_none"
    )]
    pub police_phone_number: Option<String>,
    /// Enable KIDS feature.
    #[serde(rename = "kids_enabled", skip_serializing_if = "Option::is_none")]
    pub kids_enabled: Option<bool>,
    /// Display presence for Kids.
    #[serde(
        rename = "display_kid_presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub display_kid_presence: Option<bool>,
    /// Display presence for guests.
    #[serde(
        rename = "display_guest_presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub display_guest_presence: Option<bool>,
    /// Enable automatic shutter (for cameras).
    #[serde(
        rename = "shutter_automatic_enabled",
        skip_serializing_if = "Option::is_none"
    )]
    pub shutter_automatic_enabled: Option<bool>,
    /// Enable user presence display.
    #[serde(rename = "presence_enabled", skip_serializing_if = "Option::is_none")]
    pub presence_enabled: Option<bool>,
    /// Disabled surveillance on disarm (for cameras).
    #[serde(
        rename = "outdoor_shutter_automatic_enabled",
        skip_serializing_if = "Option::is_none"
    )]
    pub outdoor_shutter_automatic_enabled: Option<bool>,
    /// Enable smoke alarm detection.
    #[serde(
        rename = "smoke_alarm_listening_enabled",
        skip_serializing_if = "Option::is_none"
    )]
    pub smoke_alarm_listening_enabled: Option<bool>,
    /// Enable Myfox Around.
    #[serde(rename = "mfa_enabled", skip_serializing_if = "Option::is_none")]
    pub mfa_enabled: Option<bool>,
    /// Myfox Around  Feature.
    #[serde(rename = "mfa_available", skip_serializing_if = "Option::is_none")]
    pub mfa_available: Option<bool>,
    #[serde(rename = "exitDelay", skip_serializing_if = "Option::is_none")]
    pub exit_delay: Option<i32>,
    /// Installer Id.
    #[serde(rename = "installer_id", skip_serializing_if = "Option::is_none")]
    pub installer_id: Option<String>,
}

impl SiteOutput {
    pub fn new(
        site_id: String,
        security_level: SecurityLevel,
        invitation: models::SiteInvitationOutput,
        alarm: models::SiteAlarm,
        restoration_active: bool,
        privacy_active: bool,
        read_only: bool,
        services: models::SiteServicesOutput,
    ) -> SiteOutput {
        SiteOutput {
            site_id,
            security_level,
            invitation: Box::new(invitation),
            alarm: Box::new(alarm),
            diagnosis: None,
            install_completed: None,
            brand: None,
            restoration_active,
            privacy_active,
            read_only,
            hcs_site_id: None,
            ilo_site_id: None,
            origin: None,
            axa_it_phone_number: None,
            axa_it_phone_number_displayed: None,
            features: None,
            domestic_alarm_output_collection: None,
            bundle_type: None,
            services: Box::new(services),
            label: None,
            services_version: None,
            subscription_active: None,
            timezone: None,
            name: None,
            address1: None,
            address2: None,
            zip_code: None,
            city: None,
            region: None,
            country: None,
            crosstreet: None,
            complement: None,
            latitude: None,
            longitude: None,
            police_phone_number: None,
            kids_enabled: None,
            display_kid_presence: None,
            display_guest_presence: None,
            shutter_automatic_enabled: None,
            presence_enabled: None,
            outdoor_shutter_automatic_enabled: None,
            smoke_alarm_listening_enabled: None,
            mfa_enabled: None,
            mfa_available: None,
            exit_delay: None,
            installer_id: None,
        }
    }
}
/// Site security level.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum SecurityLevel {
    #[serde(rename = "disarmed")]
    Disarmed,
    #[serde(rename = "partial")]
    Partial,
    #[serde(rename = "armed")]
    Armed,
}

impl Default for SecurityLevel {
    fn default() -> SecurityLevel {
        Self::Disarmed
    }
}
/// Bundle type for  ILO.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum BundleType {
    #[serde(rename = "home")]
    Home,
    #[serde(rename = "flat")]
    Flat,
}

impl Default for BundleType {
    fn default() -> BundleType {
        Self::Home
    }
}
