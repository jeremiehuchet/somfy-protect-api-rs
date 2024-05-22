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
pub struct ServiceAvailable {
    /// List of current subscriptions.
    #[serde(rename = "subscriptions")]
    pub subscriptions: Vec<models::ServiceSubscription>,
}

impl ServiceAvailable {
    pub fn new(subscriptions: Vec<models::ServiceSubscription>) -> ServiceAvailable {
        ServiceAvailable { subscriptions }
    }
}
