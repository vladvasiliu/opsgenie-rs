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
#[serde(tag = "type")]
pub enum Recipient {
    #[serde(rename="all")]
    AllRecipient {
        #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
        id: Option<String>,
    },
    #[serde(rename="escalation")]
    EscalationRecipient {
        #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
        id: Option<String>,
        #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
        name: Option<String>,
    },
    #[serde(rename="group")]
    GroupRecipient {
        #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
        id: Option<String>,
        #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
        name: Option<String>,
    },
    #[serde(rename="none")]
    NoRecipient {
        #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
        id: Option<String>,
    },
    #[serde(rename="schedule")]
    ScheduleRecipient {
        #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
        id: Option<String>,
        #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
        name: Option<String>,
    },
    #[serde(rename="team")]
    TeamRecipient {
        #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
        id: Option<String>,
        #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
        name: Option<String>,
    },
    #[serde(rename="user")]
    UserRecipient {
        #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
        id: Option<String>,
        #[serde(rename = "username", skip_serializing_if = "Option::is_none")]
        username: Option<String>,
    },
}



/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "all")]
    All,
    #[serde(rename = "none")]
    None,
    #[serde(rename = "user")]
    User,
    #[serde(rename = "escalation")]
    Escalation,
    #[serde(rename = "schedule")]
    Schedule,
    #[serde(rename = "team")]
    Team,
    #[serde(rename = "group")]
    Group,
}
