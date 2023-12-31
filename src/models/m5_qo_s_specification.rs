/*
 * Nnef_EventExposure
 *
 * NEF Event Exposure Service.   © 2022 , 3GPP Organizational Partners (ARIB, ATIS, CCSA, ETSI, TSDSI, TTA, TTC).   All rights reserved. 
 *
 * The version of the OpenAPI document: 1.2.0
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct M5QoSSpecification {
    /// String representing a bit rate; the prefixes follow the standard symbols from The International System of Units, and represent x1000 multipliers, with the exception that prefix \"K\" is used to represent the standard symbol \"k\". 
    #[serde(rename = "marBwDlBitRate")]
    pub mar_bw_dl_bit_rate: String,
    /// String representing a bit rate; the prefixes follow the standard symbols from The International System of Units, and represent x1000 multipliers, with the exception that prefix \"K\" is used to represent the standard symbol \"k\". 
    #[serde(rename = "marBwUlBitRate")]
    pub mar_bw_ul_bit_rate: String,
    /// String representing a bit rate; the prefixes follow the standard symbols from The International System of Units, and represent x1000 multipliers, with the exception that prefix \"K\" is used to represent the standard symbol \"k\". 
    #[serde(rename = "minDesBwDlBitRate", skip_serializing_if = "Option::is_none")]
    pub min_des_bw_dl_bit_rate: Option<String>,
    /// String representing a bit rate; the prefixes follow the standard symbols from The International System of Units, and represent x1000 multipliers, with the exception that prefix \"K\" is used to represent the standard symbol \"k\". 
    #[serde(rename = "minDesBwUlBitRate", skip_serializing_if = "Option::is_none")]
    pub min_des_bw_ul_bit_rate: Option<String>,
    /// String representing a bit rate; the prefixes follow the standard symbols from The International System of Units, and represent x1000 multipliers, with the exception that prefix \"K\" is used to represent the standard symbol \"k\". 
    #[serde(rename = "mirBwDlBitRate")]
    pub mir_bw_dl_bit_rate: String,
    /// String representing a bit rate; the prefixes follow the standard symbols from The International System of Units, and represent x1000 multipliers, with the exception that prefix \"K\" is used to represent the standard symbol \"k\". 
    #[serde(rename = "mirBwUlBitRate")]
    pub mir_bw_ul_bit_rate: String,
    #[serde(rename = "desLatency", skip_serializing_if = "Option::is_none")]
    pub des_latency: Option<i32>,
    #[serde(rename = "desLoss", skip_serializing_if = "Option::is_none")]
    pub des_loss: Option<i32>,
}

impl M5QoSSpecification {
    pub fn new(mar_bw_dl_bit_rate: String, mar_bw_ul_bit_rate: String, mir_bw_dl_bit_rate: String, mir_bw_ul_bit_rate: String) -> M5QoSSpecification {
        M5QoSSpecification {
            mar_bw_dl_bit_rate,
            mar_bw_ul_bit_rate,
            min_des_bw_dl_bit_rate: None,
            min_des_bw_ul_bit_rate: None,
            mir_bw_dl_bit_rate,
            mir_bw_ul_bit_rate,
            des_latency: None,
            des_loss: None,
        }
    }
}


