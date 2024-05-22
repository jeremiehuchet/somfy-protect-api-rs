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
pub struct ServicePeriod {
    /// Quantity.
    #[serde(rename = "quantity")]
    pub quantity: i32,
    /// Period unit.
    #[serde(rename = "unit")]
    pub unit: Unit,
}

impl ServicePeriod {
    pub fn new(quantity: i32, unit: Unit) -> ServicePeriod {
        ServicePeriod { quantity, unit }
    }
}
/// Period unit.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Unit {
    #[serde(rename = "year")]
    Year,
    #[serde(rename = "month")]
    Month,
    #[serde(rename = "day")]
    Day,
}

impl Default for Unit {
    fn default() -> Unit {
        Self::Year
    }
}
