/*
 * Python SDK for Opsgenie REST API
 *
 * Python SDK for Opsgenie REST API
 *
 * The version of the OpenAPI document: 2.0.0
 * Contact: support@opsgenie.com
 * Generated by: https://openapi-generator.tech
 */

/// UserResponder : User responder



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UserResponder {
    #[serde(rename = "type")]
    pub _type: Type,
    #[serde(rename = "id")]
    pub id: String,
}

impl UserResponder {
    /// User responder
    pub fn new(_type: Type, id: String) -> UserResponder {
        UserResponder {
            _type,
            id,
        }
    }
}

/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "user")]
    User,
    #[serde(rename = "team")]
    Team,
    #[serde(rename = "escalation")]
    Escalation,
    #[serde(rename = "schedule")]
    Schedule,
}

