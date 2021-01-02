/*
 * Python SDK for Opsgenie REST API
 *
 * Python SDK for Opsgenie REST API
 *
 * The version of the OpenAPI document: 2.0.0
 * Contact: support@opsgenie.com
 * Generated by: https://openapi-generator.tech
 */

/// Filter : Defines the conditions that will be checked before applying rules and type of the operations that will be applied on conditions


#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum Filter {
    #[serde(rename="match-all")]
    MatchAll {
    },
    #[serde(rename="match-all-conditions")]
    MatchAllConditions {
        #[serde(rename = "conditions", skip_serializing_if = "Option::is_none")]
        conditions: Option<Vec<crate::models::Condition>>,
    },
    #[serde(rename="match-any-condition")]
    MatchAnyCondition {
        #[serde(rename = "conditions", skip_serializing_if = "Option::is_none")]
        conditions: Option<Vec<crate::models::Condition>>,
    },
}



/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "match-all")]
    All,
    #[serde(rename = "match-any-condition")]
    AnyCondition,
    #[serde(rename = "match-all-conditions")]
    AllConditions,
}

