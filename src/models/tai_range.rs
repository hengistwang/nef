/*
 * Namf_EventExposure
 *
 * AMF Event Exposure Service.   © 2023, 3GPP Organizational Partners (ARIB, ATIS, CCSA, ETSI, TSDSI, TTA, TTC).   All rights reserved. 
 *
 * The version of the OpenAPI document: 1.2.3
 * 
 * Generated by: https://openapi-generator.tech
 */

/// TaiRange : Range of TAIs (Tracking Area Identities)



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TaiRange {
    #[serde(rename = "plmnId")]
    pub plmn_id: Box<crate::models::PlmnId>,
    #[serde(rename = "tacRangeList")]
    pub tac_range_list: Vec<crate::models::TacRange>,
    /// This represents the Network Identifier, which together with a PLMN ID is used to identify an SNPN (see 3GPP TS 23.003 and 3GPP TS 23.501 clause 5.30.2.1).  
    #[serde(rename = "nid", skip_serializing_if = "Option::is_none")]
    pub nid: Option<String>,
}

impl TaiRange {
    /// Range of TAIs (Tracking Area Identities)
    pub fn new(plmn_id: crate::models::PlmnId, tac_range_list: Vec<crate::models::TacRange>) -> TaiRange {
        TaiRange {
            plmn_id: Box::new(plmn_id),
            tac_range_list,
            nid: None,
        }
    }
}


