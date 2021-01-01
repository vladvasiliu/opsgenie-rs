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
pub struct BaseResponse {
    #[serde(rename = "requestId")]
    pub request_id: String,
    #[serde(rename = "took")]
    pub took: f32,
}

impl BaseResponse {
    pub fn new(request_id: String, took: f32) -> BaseResponse {
        BaseResponse {
            request_id,
            took,
        }
    }
}

