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
pub struct UpdateAlertPriorityPayload {
    /// Priority level of the alert
    #[serde(rename = "priority")]
    pub priority: Priority,
}

impl UpdateAlertPriorityPayload {
    pub fn new(priority: Priority) -> UpdateAlertPriorityPayload {
        UpdateAlertPriorityPayload {
            priority,
        }
    }
}

/// Priority level of the alert
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Priority {
    #[serde(rename = "P1")]
    P1,
    #[serde(rename = "P2")]
    P2,
    #[serde(rename = "P3")]
    P3,
    #[serde(rename = "P4")]
    P4,
    #[serde(rename = "P5")]
    P5,
}
