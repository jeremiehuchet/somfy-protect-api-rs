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
pub struct ServiceUnsubscribeInput {
    /// Option to send mail.
    #[serde(rename = "email")]
    pub email: bool,
}

impl ServiceUnsubscribeInput {
    pub fn new(email: bool) -> ServiceUnsubscribeInput {
        ServiceUnsubscribeInput { email }
    }
}
