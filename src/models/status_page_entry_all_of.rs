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
pub struct StatusPageEntryAllOf {
    /// The message to be added using status page into incident
    #[serde(rename = "title", skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// Description of the message being added via stats page into incident
    #[serde(rename = "details", skip_serializing_if = "Option::is_none")]
    pub details: Option<String>,
}

impl StatusPageEntryAllOf {
    pub fn new() -> StatusPageEntryAllOf {
        StatusPageEntryAllOf {
            title: None,
            details: None,
        }
    }
}


