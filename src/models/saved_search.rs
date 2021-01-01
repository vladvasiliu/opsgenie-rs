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
pub struct SavedSearch {
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "createdAt", skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(rename = "updatedAt", skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
    #[serde(rename = "owner", skip_serializing_if = "Option::is_none")]
    pub owner: Option<crate::models::SavedSearchEntity>,
    #[serde(rename = "teams", skip_serializing_if = "Option::is_none")]
    pub teams: Option<Vec<crate::models::SavedSearchEntity>>,
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "query", skip_serializing_if = "Option::is_none")]
    pub query: Option<String>,
}

impl SavedSearch {
    pub fn new() -> SavedSearch {
        SavedSearch {
            id: None,
            name: None,
            created_at: None,
            updated_at: None,
            owner: None,
            teams: None,
            description: None,
            query: None,
        }
    }
}

