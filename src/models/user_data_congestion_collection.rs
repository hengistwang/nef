/*
 * Nnef_EventExposure
 *
 * NEF Event Exposure Service.   © 2022 , 3GPP Organizational Partners (ARIB, ATIS, CCSA, ETSI, TSDSI, TTA, TTC).   All rights reserved. 
 *
 * The version of the OpenAPI document: 1.2.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// UserDataCongestionCollection : Contains User Data Congestion Analytics related information collection.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UserDataCongestionCollection {
    /// String providing an application identifier.
    #[serde(rename = "appId", skip_serializing_if = "Option::is_none")]
    pub app_id: Option<String>,
    #[serde(rename = "ipTrafficFilter", skip_serializing_if = "Option::is_none")]
    pub ip_traffic_filter: Option<Box<crate::models::FlowInfo>>,
    #[serde(rename = "timeInterv", skip_serializing_if = "Option::is_none")]
    pub time_interv: Option<Box<crate::models::TimeWindow>>,
    /// String representing a bit rate; the prefixes follow the standard symbols from The International System of Units, and represent x1000 multipliers, with the exception that prefix \"K\" is used to represent the standard symbol \"k\". 
    #[serde(rename = "thrputUl", skip_serializing_if = "Option::is_none")]
    pub thrput_ul: Option<String>,
    /// String representing a bit rate; the prefixes follow the standard symbols from The International System of Units, and represent x1000 multipliers, with the exception that prefix \"K\" is used to represent the standard symbol \"k\". 
    #[serde(rename = "thrputDl", skip_serializing_if = "Option::is_none")]
    pub thrput_dl: Option<String>,
    /// String representing a bit rate; the prefixes follow the standard symbols from The International System of Units, and represent x1000 multipliers, with the exception that prefix \"K\" is used to represent the standard symbol \"k\". 
    #[serde(rename = "thrputPkUl", skip_serializing_if = "Option::is_none")]
    pub thrput_pk_ul: Option<String>,
    /// String representing a bit rate; the prefixes follow the standard symbols from The International System of Units, and represent x1000 multipliers, with the exception that prefix \"K\" is used to represent the standard symbol \"k\". 
    #[serde(rename = "thrputPkDl", skip_serializing_if = "Option::is_none")]
    pub thrput_pk_dl: Option<String>,
}

impl UserDataCongestionCollection {
    /// Contains User Data Congestion Analytics related information collection.
    pub fn new() -> UserDataCongestionCollection {
        UserDataCongestionCollection {
            app_id: None,
            ip_traffic_filter: None,
            time_interv: None,
            thrput_ul: None,
            thrput_dl: None,
            thrput_pk_ul: None,
            thrput_pk_dl: None,
        }
    }
}


