/*
 * Python SDK for Opsgenie REST API
 *
 * Python SDK for Opsgenie REST API
 *
 * The version of the OpenAPI document: 2.0.0
 * Contact: support@opsgenie.com
 * Generated by: https://openapi-generator.tech
 */

use crate::models::recipient::Type;

/// TeamRecipient : Team recipient



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TeamRecipient {
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

impl TeamRecipient {
    /// Team recipient
    pub fn new(_type: Type) -> TeamRecipient {
        TeamRecipient {
            id: None,
            name: None,
        }
    }
}


