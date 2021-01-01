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
pub struct CreateIncidentPayloadAllOf {
    /// Message of the incident
    #[serde(rename = "message", skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    /// Description field of the incident that is generally used to provide a detailed information about the incident.
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Responders that the incident will be routed to send notifications
    #[serde(rename = "responders", skip_serializing_if = "Option::is_none")]
    pub responders: Option<Vec<crate::models::Recipient>>,
    /// Tags of the incident.
    #[serde(rename = "tags", skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
    /// Map of key-value pairs to use as custom properties of the incident
    #[serde(rename = "details", skip_serializing_if = "Option::is_none")]
    pub details: Option<::std::collections::HashMap<String, String>>,
    /// Priority level of the incident
    #[serde(rename = "priority", skip_serializing_if = "Option::is_none")]
    pub priority: Option<Priority>,
    /// Additional note that will be added while creating the incident
    #[serde(rename = "note", skip_serializing_if = "Option::is_none")]
    pub note: Option<String>,
    /// Service on which incident will be created.
    #[serde(rename = "serviceId", skip_serializing_if = "Option::is_none")]
    pub service_id: Option<String>,
    /// Status page entry fields. If this field is leaved blank, message and description of incident will be used for title and detail respectively.
    #[serde(rename = "statusPageEntry", skip_serializing_if = "Option::is_none")]
    pub status_page_entry: Option<::std::collections::HashMap<String, crate::models::StatusPageEntry>>,
    /// Indicate whether stakeholders are notified or not. Default value is false.
    #[serde(rename = "notifyStakeholders", skip_serializing_if = "Option::is_none")]
    pub notify_stakeholders: Option<bool>,
}

impl CreateIncidentPayloadAllOf {
    pub fn new() -> CreateIncidentPayloadAllOf {
        CreateIncidentPayloadAllOf {
            message: None,
            description: None,
            responders: None,
            tags: None,
            details: None,
            priority: None,
            note: None,
            service_id: None,
            status_page_entry: None,
            notify_stakeholders: None,
        }
    }
}

/// Priority level of the incident
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

