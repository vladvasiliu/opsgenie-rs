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

/// NoRecipient : No recipient



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct NoRecipient {
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}

impl NoRecipient {
    /// No recipient
    pub fn new(_type: Type) -> NoRecipient {
        NoRecipient {
            id: None,
        }
    }
}


