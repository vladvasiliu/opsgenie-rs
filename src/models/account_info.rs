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
pub struct AccountInfo {
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "userCount", skip_serializing_if = "Option::is_none")]
    pub user_count: Option<i32>,
    #[serde(rename = "plan", skip_serializing_if = "Option::is_none")]
    pub plan: Option<crate::models::AccountPlan>,
}

impl AccountInfo {
    pub fn new() -> AccountInfo {
        AccountInfo {
            name: None,
            user_count: None,
            plan: None,
        }
    }
}


