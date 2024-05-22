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
pub struct DeviceAction {
    /// Action to perform.
    #[serde(rename = "action")]
    pub action: Action,
    #[serde(rename = "duration", skip_serializing_if = "Option::is_none")]
    pub duration: Option<i32>,
    #[serde(rename = "video_backend", skip_serializing_if = "Option::is_none")]
    pub video_backend: Option<VideoBackend>,
    #[serde(rename = "sessionId", skip_serializing_if = "Option::is_none")]
    pub session_id: Option<String>,
}

impl DeviceAction {
    pub fn new(action: Action) -> DeviceAction {
        DeviceAction {
            action,
            duration: None,
            video_backend: None,
            session_id: None,
        }
    }
}
/// Action to perform.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Action {
    #[serde(rename = "shutter_close")]
    ShutterClose,
    #[serde(rename = "shutter_open")]
    ShutterOpen,
    #[serde(rename = "test_start")]
    TestStart,
    #[serde(rename = "test_stop")]
    TestStop,
    #[serde(rename = "test_extend")]
    TestExtend,
    #[serde(rename = "stream_start")]
    StreamStart,
    #[serde(rename = "stream_stop")]
    StreamStop,
    #[serde(rename = "push_to_talk_start")]
    PushToTalkStart,
    #[serde(rename = "push_to_talk_stop")]
    PushToTalkStop,
    #[serde(rename = "change_video_backend")]
    ChangeVideoBackend,
    #[serde(rename = "light_on")]
    LightOn,
    #[serde(rename = "light_off")]
    LightOff,
    #[serde(rename = "measure_ambient_light")]
    MeasureAmbientLight,
}

impl Default for Action {
    fn default() -> Action {
        Self::ShutterClose
    }
}
///
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum VideoBackend {
    #[serde(rename = "evostream")]
    Evostream,
    #[serde(rename = "webrtc")]
    Webrtc,
}

impl Default for VideoBackend {
    fn default() -> VideoBackend {
        Self::Evostream
    }
}
