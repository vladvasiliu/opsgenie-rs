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
pub struct AlertLog {
    #[serde(rename = "log", skip_serializing_if = "Option::is_none")]
    pub log: Option<String>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub _type: Option<String>,
    #[serde(rename = "owner", skip_serializing_if = "Option::is_none")]
    pub owner: Option<String>,
    #[serde(rename = "createdAt", skip_serializing_if = "Option::is_none")]
    pub created_at: Option<chrono::DateTime<chrono::offset::Utc>>,
    #[serde(rename = "offset", skip_serializing_if = "Option::is_none")]
    pub offset: Option<String>,
}

impl AlertLog {
    pub fn new() -> AlertLog {
        AlertLog {
            log: None,
            _type: None,
            owner: None,
            created_at: None,
            offset: None,
        }
    }
}


