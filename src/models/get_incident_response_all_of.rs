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
pub struct GetIncidentResponseAllOf {
    #[serde(rename = "data", skip_serializing_if = "Option::is_none")]
    pub data: Option<crate::models::Incident>,
}

impl GetIncidentResponseAllOf {
    pub fn new() -> GetIncidentResponseAllOf {
        GetIncidentResponseAllOf {
            data: None,
        }
    }
}

