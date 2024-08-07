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
pub struct SiteEventCollection {
    /// List of SiteEvent objects.
    #[serde(rename = "items")]
    pub items: Vec<models::SiteEvent>,
}

impl SiteEventCollection {
    pub fn new(items: Vec<models::SiteEvent>) -> SiteEventCollection {
        SiteEventCollection { items }
    }
}
