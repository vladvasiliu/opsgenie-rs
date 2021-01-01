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
pub struct AddResponderToAlertPayload {
    /// Display name of the request owner
    #[serde(rename = "user", skip_serializing_if = "Option::is_none")]
    pub user: Option<String>,
    /// Additional note that will be added while creating the alert
    #[serde(rename = "note", skip_serializing_if = "Option::is_none")]
    pub note: Option<String>,
    /// Source field of the alert. Default value is IP address of the incoming request
    #[serde(rename = "source", skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
    #[serde(rename = "responder")]
    pub responder: crate::models::Recipient,
}

impl AddResponderToAlertPayload {
    pub fn new(responder: crate::models::Recipient) -> AddResponderToAlertPayload {
        AddResponderToAlertPayload {
            user: None,
            note: None,
            source: None,
            responder,
        }
    }
}


