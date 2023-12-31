/*
 * Namf_EventExposure
 *
 * AMF Event Exposure Service.   © 2023, 3GPP Organizational Partners (ARIB, ATIS, CCSA, ETSI, TSDSI, TTA, TTC).   All rights reserved. 
 *
 * The version of the OpenAPI document: 1.2.3
 * 
 * Generated by: https://openapi-generator.tech
 */

/// DispersionArea : Dispersion Area



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DispersionArea {
    #[serde(rename = "taiList", skip_serializing_if = "Option::is_none")]
    pub tai_list: Option<Vec<crate::models::Tai>>,
    #[serde(rename = "ncgiList", skip_serializing_if = "Option::is_none")]
    pub ncgi_list: Option<Vec<crate::models::Ncgi>>,
    #[serde(rename = "ecgiList", skip_serializing_if = "Option::is_none")]
    pub ecgi_list: Option<Vec<crate::models::Ecgi>>,
    #[serde(rename = "n3gaInd", skip_serializing_if = "Option::is_none")]
    pub n3ga_ind: Option<bool>,
}

impl DispersionArea {
    /// Dispersion Area
    pub fn new() -> DispersionArea {
        DispersionArea {
            tai_list: None,
            ncgi_list: None,
            ecgi_list: None,
            n3ga_ind: None,
        }
    }
}


