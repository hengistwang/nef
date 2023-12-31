/*
 * Namf_EventExposure
 *
 * AMF Event Exposure Service.   © 2023, 3GPP Organizational Partners (ARIB, ATIS, CCSA, ETSI, TSDSI, TTA, TTC).   All rights reserved. 
 *
 * The version of the OpenAPI document: 1.2.3
 * 
 * Generated by: https://openapi-generator.tech
 */

/// TargetArea : TA list or TAI range list or any TA



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TargetArea {
    #[serde(rename = "taList", skip_serializing_if = "Option::is_none")]
    pub ta_list: Option<Vec<crate::models::Tai>>,
    #[serde(rename = "taiRangeList", skip_serializing_if = "Option::is_none")]
    pub tai_range_list: Option<Vec<crate::models::TaiRange>>,
    #[serde(rename = "anyTa", skip_serializing_if = "Option::is_none")]
    pub any_ta: Option<bool>,
}

impl TargetArea {
    /// TA list or TAI range list or any TA
    pub fn new() -> TargetArea {
        TargetArea {
            ta_list: None,
            tai_range_list: None,
            any_ta: None,
        }
    }
}


