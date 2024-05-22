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
pub struct JobCollection {
    /// List of user's job.
    #[serde(rename = "items")]
    pub items: Vec<models::Job>,
}

impl JobCollection {
    pub fn new(items: Vec<models::Job>) -> JobCollection {
        JobCollection { items }
    }
}
