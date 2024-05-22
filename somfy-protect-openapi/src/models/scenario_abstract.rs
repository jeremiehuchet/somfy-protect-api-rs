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
pub struct ScenarioAbstract {
    /// Start time of Scenario in format HH:MM.
    #[serde(rename = "time")]
    pub time: String,
    /// Days of week when Scenario must be applied. Possible values for days: mo, tu, we, th, fr, sa, su.
    #[serde(rename = "days")]
    pub days: Vec<String>,
    /// Security level.
    #[serde(rename = "security_level")]
    pub security_level: SecurityLevel,
    /// Is scenario enabled?
    #[serde(rename = "enabled", skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[serde(rename = "scenario_action", skip_serializing_if = "Option::is_none")]
    pub scenario_action: Option<Box<models::ScenarioAction>>,
}

impl ScenarioAbstract {
    pub fn new(time: String, days: Vec<String>, security_level: SecurityLevel) -> ScenarioAbstract {
        ScenarioAbstract {
            time,
            days,
            security_level,
            enabled: None,
            scenario_action: None,
        }
    }
}
/// Security level.
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
