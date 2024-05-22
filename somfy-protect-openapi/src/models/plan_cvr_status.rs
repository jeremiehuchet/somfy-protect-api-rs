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
pub struct PlanCvrStatus {
    /// CVR span.
    #[serde(rename = "cvr_span", skip_serializing_if = "Option::is_none")]
    pub cvr_span: Option<i32>,
    /// List of cameras.
    #[serde(rename = "cameras")]
    pub cameras: Vec<models::PlanCvrCamera>,
    /// Service provider.
    #[serde(rename = "provider", skip_serializing_if = "Option::is_none")]
    pub provider: Option<String>,
    /// Service is displayed.
    #[serde(rename = "displayed")]
    pub displayed: bool,
    /// Service is available.
    #[serde(rename = "available")]
    pub available: bool,
    /// Service has warning.
    #[serde(rename = "warning")]
    pub warning: bool,
    /// Service status.
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// Service link to next plan.
    #[serde(rename = "linkedCode", skip_serializing_if = "Option::is_none")]
    pub linked_code: Option<String>,
}

impl PlanCvrStatus {
    pub fn new(
        cameras: Vec<models::PlanCvrCamera>,
        displayed: bool,
        available: bool,
        warning: bool,
    ) -> PlanCvrStatus {
        PlanCvrStatus {
            cvr_span: None,
            cameras,
            provider: None,
            displayed,
            available,
            warning,
            status: None,
            linked_code: None,
        }
    }
}
