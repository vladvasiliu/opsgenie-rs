/*
 * Python SDK for Opsgenie REST API
 *
 * Python SDK for Opsgenie REST API
 *
 * The version of the OpenAPI document: 2.0.0
 * Contact: support@opsgenie.com
 * Generated by: https://openapi-generator.tech
 */

use crate::models::filter::Type;

/// MatchAnyCondition : Match any condition filter



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MatchAnyCondition {
    #[serde(rename = "conditions", skip_serializing_if = "Option::is_none")]
    pub conditions: Option<Vec<crate::models::Condition>>,
}

impl MatchAnyCondition {
    /// Match any condition filter
    pub fn new(_type: Type) -> MatchAnyCondition {
        MatchAnyCondition {
            conditions: None,
        }
    }
}


