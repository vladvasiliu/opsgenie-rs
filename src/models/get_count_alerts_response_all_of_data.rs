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
pub struct GetCountAlertsResponseAllOfData {
    #[serde(rename = "count", skip_serializing_if = "Option::is_none")]
    pub count: Option<i32>,
}

impl GetCountAlertsResponseAllOfData {
    pub fn new() -> GetCountAlertsResponseAllOfData {
        GetCountAlertsResponseAllOfData {
            count: None,
        }
    }
}


