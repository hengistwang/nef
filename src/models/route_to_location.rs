/*
 * Nsmf_EventExposure
 *
 * Session Management Event Exposure Service.   © 2023, 3GPP Organizational Partners (ARIB, ATIS, CCSA, ETSI, TSDSI, TTA, TTC).   All rights reserved. 
 *
 * The version of the OpenAPI document: 1.2.2
 * 
 * Generated by: https://openapi-generator.tech
 */

/// RouteToLocation : At least one of the \"routeInfo\" attribute and the \"routeProfId\" attribute shall be included in the \"RouteToLocation\" data type. 



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RouteToLocation {
    /// DNAI (Data network access identifier), see clause 5.6.7 of 3GPP TS 23.501.
    #[serde(rename = "dnai")]
    pub dnai: String,
    #[serde(rename = "routeInfo", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub route_info: Option<Option<Box<crate::models::RouteInformation>>>,
    /// Identifies the routing profile Id.
    #[serde(rename = "routeProfId", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub route_prof_id: Option<Option<String>>,
}

impl RouteToLocation {
    /// At least one of the \"routeInfo\" attribute and the \"routeProfId\" attribute shall be included in the \"RouteToLocation\" data type. 
    pub fn new(dnai: String) -> RouteToLocation {
        RouteToLocation {
            dnai,
            route_info: None,
            route_prof_id: None,
        }
    }
}


