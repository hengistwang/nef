/*
 * Namf_EventExposure
 *
 * AMF Event Exposure Service.   © 2023, 3GPP Organizational Partners (ARIB, ATIS, CCSA, ETSI, TSDSI, TTA, TTC).   All rights reserved. 
 *
 * The version of the OpenAPI document: 1.2.3
 * 
 * Generated by: https://openapi-generator.tech
 */

/// PlmnId : When PlmnId needs to be converted to string (e.g. when used in maps as key), the string  shall be composed of three digits \"mcc\" followed by \"-\" and two or three digits \"mnc\". 



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PlmnId {
    /// Mobile Country Code part of the PLMN, comprising 3 digits, as defined in clause 9.3.3.5 of 3GPP TS 38.413.  
    #[serde(rename = "mcc")]
    pub mcc: String,
    /// Mobile Network Code part of the PLMN, comprising 2 or 3 digits, as defined in clause 9.3.3.5 of 3GPP TS 38.413.
    #[serde(rename = "mnc")]
    pub mnc: String,
}

impl PlmnId {
    /// When PlmnId needs to be converted to string (e.g. when used in maps as key), the string  shall be composed of three digits \"mcc\" followed by \"-\" and two or three digits \"mnc\". 
    pub fn new(mcc: String, mnc: String) -> PlmnId {
        PlmnId {
            mcc,
            mnc,
        }
    }
}


