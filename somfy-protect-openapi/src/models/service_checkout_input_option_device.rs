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
pub struct ServiceCheckoutInputOptionDevice {
    /// Device identifier.
    #[serde(rename = "device_id")]
    pub device_id: String,
}

impl ServiceCheckoutInputOptionDevice {
    pub fn new(device_id: String) -> ServiceCheckoutInputOptionDevice {
        ServiceCheckoutInputOptionDevice { device_id }
    }
}
