/*
 * Nnef_EventExposure
 *
 * NEF Event Exposure Service.   © 2022 , 3GPP Organizational Partners (ARIB, ATIS, CCSA, ETSI, TSDSI, TTA, TTC).   All rights reserved. 
 *
 * The version of the OpenAPI document: 1.2.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// CommunicationCollection : Contains communication information.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CommunicationCollection {
    /// string with format 'date-time' as defined in OpenAPI.
    #[serde(rename = "startTime")]
    pub start_time: String,
    /// string with format 'date-time' as defined in OpenAPI.
    #[serde(rename = "endTime")]
    pub end_time: String,
    /// Unsigned integer identifying a volume in units of bytes.
    #[serde(rename = "ulVol")]
    pub ul_vol: i64,
    /// Unsigned integer identifying a volume in units of bytes.
    #[serde(rename = "dlVol")]
    pub dl_vol: i64,
}

impl CommunicationCollection {
    /// Contains communication information.
    pub fn new(start_time: String, end_time: String, ul_vol: i64, dl_vol: i64) -> CommunicationCollection {
        CommunicationCollection {
            start_time,
            end_time,
            ul_vol,
            dl_vol,
        }
    }
}


