/*
 * Python SDK for Opsgenie REST API
 *
 * Python SDK for Opsgenie REST API
 *
 * The version of the OpenAPI document: 2.0.0
 * Contact: support@opsgenie.com
 * Generated by: https://openapi-generator.tech
 */

use crate::models::time_restriction_interval::Type;

/// TimeOfDayRestrictionInterval : Time of day restriction interval



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TimeOfDayRestrictionInterval {
    #[serde(rename = "restriction", skip_serializing_if = "Option::is_none")]
    pub restriction: Option<crate::models::TimeOfDayRestriction>,
}

impl TimeOfDayRestrictionInterval {
    /// Time of day restriction interval
    pub fn new(_type: Type) -> TimeOfDayRestrictionInterval {
        TimeOfDayRestrictionInterval {
            restriction: None,
        }
    }
}


