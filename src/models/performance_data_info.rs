/*
 * Nnef_EventExposure
 *
 * NEF Event Exposure Service.   © 2022 , 3GPP Organizational Partners (ARIB, ATIS, CCSA, ETSI, TSDSI, TTA, TTC).   All rights reserved. 
 *
 * The version of the OpenAPI document: 1.2.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// PerformanceDataInfo : Contains Performance Data Analytics related information collection.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PerformanceDataInfo {
    /// String providing an application identifier.
    #[serde(rename = "appId", skip_serializing_if = "Option::is_none")]
    pub app_id: Option<String>,
    #[serde(rename = "ueIpAddr", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub ue_ip_addr: Option<Option<Box<crate::models::IpAddr>>>,
    #[serde(rename = "ipTrafficFilter", skip_serializing_if = "Option::is_none")]
    pub ip_traffic_filter: Option<Box<crate::models::FlowInfo>>,
    #[serde(rename = "userLoc", skip_serializing_if = "Option::is_none")]
    pub user_loc: Option<Box<crate::models::UserLocation>>,
    #[serde(rename = "appLocs", skip_serializing_if = "Option::is_none")]
    pub app_locs: Option<Vec<String>>,
    #[serde(rename = "asAddr", skip_serializing_if = "Option::is_none")]
    pub as_addr: Option<Box<crate::models::AddrFqdn>>,
    #[serde(rename = "perfData")]
    pub perf_data: Box<crate::models::PerformanceData>,
    /// string with format 'date-time' as defined in OpenAPI.
    #[serde(rename = "timeStamp")]
    pub time_stamp: String,
}

impl PerformanceDataInfo {
    /// Contains Performance Data Analytics related information collection.
    pub fn new(perf_data: crate::models::PerformanceData, time_stamp: String) -> PerformanceDataInfo {
        PerformanceDataInfo {
            app_id: None,
            ue_ip_addr: None,
            ip_traffic_filter: None,
            user_loc: None,
            app_locs: None,
            as_addr: None,
            perf_data: Box::new(perf_data),
            time_stamp,
        }
    }
}

