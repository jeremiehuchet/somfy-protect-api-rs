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
pub struct PlanPriceCamera {
    /// Period.
    #[serde(rename = "period")]
    pub period: String,
    /// Price.
    #[serde(rename = "price")]
    pub price: i32,
    /// Text.
    #[serde(rename = "text")]
    pub text: String,
}

impl PlanPriceCamera {
    pub fn new(period: String, price: i32, text: String) -> PlanPriceCamera {
        PlanPriceCamera {
            period,
            price,
            text,
        }
    }
}
