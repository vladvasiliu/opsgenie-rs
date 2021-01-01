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
pub struct EscalateAlertToNextPayloadAllOf {
    #[serde(rename = "escalation")]
    pub escalation: crate::models::EscalationRecipient,
}

impl EscalateAlertToNextPayloadAllOf {
    pub fn new(escalation: crate::models::EscalationRecipient) -> EscalateAlertToNextPayloadAllOf {
        EscalateAlertToNextPayloadAllOf {
            escalation,
        }
    }
}

