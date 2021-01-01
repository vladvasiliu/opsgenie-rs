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
pub struct CreateAlertPayloadAllOf {
    /// Message of the alert
    #[serde(rename = "message")]
    pub message: String,
    /// Client-defined identifier of the alert, that is also the key element of alert deduplication.
    #[serde(rename = "alias", skip_serializing_if = "Option::is_none")]
    pub alias: Option<String>,
    /// Description field of the alert that is generally used to provide a detailed information about the alert.
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Responders that the alert will be routed to send notifications
    #[serde(rename = "responders", skip_serializing_if = "Option::is_none")]
    pub responders: Option<Vec<crate::models::Recipient>>,
    /// Teams and users that the alert will become visible to without sending any notification
    #[serde(rename = "visibleTo", skip_serializing_if = "Option::is_none")]
    pub visible_to: Option<Vec<crate::models::Recipient>>,
    /// Custom actions that will be available for the alert
    #[serde(rename = "actions", skip_serializing_if = "Option::is_none")]
    pub actions: Option<Vec<String>>,
    /// Tags of the alert
    #[serde(rename = "tags", skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
    /// Map of key-value pairs to use as custom properties of the alert
    #[serde(rename = "details", skip_serializing_if = "Option::is_none")]
    pub details: Option<::std::collections::HashMap<String, String>>,
    /// Entity field of the alert that is generally used to specify which domain alert is related to
    #[serde(rename = "entity", skip_serializing_if = "Option::is_none")]
    pub entity: Option<String>,
    /// Priority level of the alert
    #[serde(rename = "priority", skip_serializing_if = "Option::is_none")]
    pub priority: Option<Priority>,
}

impl CreateAlertPayloadAllOf {
    pub fn new(message: String) -> CreateAlertPayloadAllOf {
        CreateAlertPayloadAllOf {
            message,
            alias: None,
            description: None,
            responders: None,
            visible_to: None,
            actions: None,
            tags: None,
            details: None,
            entity: None,
            priority: None,
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
