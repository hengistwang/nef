/*
 * Namf_EventExposure
 *
 * AMF Event Exposure Service.   © 2023, 3GPP Organizational Partners (ARIB, ATIS, CCSA, ETSI, TSDSI, TTA, TTC).   All rights reserved. 
 *
 * The version of the OpenAPI document: 1.2.3
 * 
 * Generated by: https://openapi-generator.tech
 */

/// AmfEventSubscriptionInfo : Individual AMF Event Subscription Information



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AmfEventSubscriptionInfo {
    /// String providing an URI formatted according to RFC 3986.
    #[serde(rename = "subId")]
    pub sub_id: String,
    #[serde(rename = "notifyCorrelationId", skip_serializing_if = "Option::is_none")]
    pub notify_correlation_id: Option<String>,
    #[serde(rename = "refIdList")]
    pub ref_id_list: Vec<i32>,
    /// String providing an URI formatted according to RFC 3986.
    #[serde(rename = "oldSubId", skip_serializing_if = "Option::is_none")]
    pub old_sub_id: Option<String>,
}

impl AmfEventSubscriptionInfo {
    /// Individual AMF Event Subscription Information
    pub fn new(sub_id: String, ref_id_list: Vec<i32>) -> AmfEventSubscriptionInfo {
        AmfEventSubscriptionInfo {
            sub_id,
            notify_correlation_id: None,
            ref_id_list,
            old_sub_id: None,
        }
    }
}


