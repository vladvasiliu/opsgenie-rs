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
pub struct Duration {
    #[serde(rename = "timeAmount")]
    pub time_amount: i64,
    #[serde(rename = "timeUnit", skip_serializing_if = "Option::is_none")]
    pub time_unit: Option<TimeUnit>,
}

impl Duration {
    pub fn new(time_amount: i64) -> Duration {
        Duration {
            time_amount,
            time_unit: None,
        }
    }
}

/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum TimeUnit {
    #[serde(rename = "days")]
    Days,
    #[serde(rename = "hours")]
    Hours,
    #[serde(rename = "minutes")]
    Minutes,
    #[serde(rename = "seconds")]
    Seconds,
    #[serde(rename = "miliseconds")]
    Miliseconds,
    #[serde(rename = "micros")]
    Micros,
    #[serde(rename = "nanos")]
    Nanos,
}
