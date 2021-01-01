/*
 * Python SDK for Opsgenie REST API
 *
 * Python SDK for Opsgenie REST API
 *
 * The version of the OpenAPI document: 2.0.0
 * Contact: support@opsgenie.com
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct HeartbeatMeta {
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "enabled", skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[serde(rename = "expired", skip_serializing_if = "Option::is_none")]
    pub expired: Option<bool>,
}

impl HeartbeatMeta {
    pub fn new() -> HeartbeatMeta {
        HeartbeatMeta {
            name: None,
            enabled: None,
            expired: None,
        }
    }
}

