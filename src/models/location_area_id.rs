/*
 * Namf_EventExposure
 *
 * AMF Event Exposure Service.   © 2023, 3GPP Organizational Partners (ARIB, ATIS, CCSA, ETSI, TSDSI, TTA, TTC).   All rights reserved. 
 *
 * The version of the OpenAPI document: 1.2.3
 * 
 * Generated by: https://openapi-generator.tech
 */

/// LocationAreaId : Contains a Location area identification as defined in 3GPP TS 23.003, clause 4.1.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LocationAreaId {
    #[serde(rename = "plmnId")]
    pub plmn_id: Box<crate::models::PlmnId>,
    /// Location Area Code.
    #[serde(rename = "lac")]
    pub lac: String,
}

impl LocationAreaId {
    /// Contains a Location area identification as defined in 3GPP TS 23.003, clause 4.1.
    pub fn new(plmn_id: crate::models::PlmnId, lac: String) -> LocationAreaId {
        LocationAreaId {
            plmn_id: Box::new(plmn_id),
            lac,
        }
    }
}


