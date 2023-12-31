/*
 * Nsmf_EventExposure
 *
 * Session Management Event Exposure Service.   © 2023, 3GPP Organizational Partners (ARIB, ATIS, CCSA, ETSI, TSDSI, TTA, TTC).   All rights reserved. 
 *
 * The version of the OpenAPI document: 1.2.2
 * 
 * Generated by: https://openapi-generator.tech
 */

/// TransactionInfo : Represents SMF Transaction Information.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TransactionInfo {
    /// Unsigned Integer, i.e. only value 0 and integers above 0 are permissible.
    #[serde(rename = "transaction")]
    pub transaction: i32,
    #[serde(rename = "snssai", skip_serializing_if = "Option::is_none")]
    pub snssai: Option<Box<crate::models::Snssai>>,
    #[serde(rename = "appIds", skip_serializing_if = "Option::is_none")]
    pub app_ids: Option<Vec<String>>,
    #[serde(rename = "transacMetrics", skip_serializing_if = "Option::is_none")]
    pub transac_metrics: Option<Vec<crate::models::TransactionMetric>>,
}

impl TransactionInfo {
    /// Represents SMF Transaction Information.
    pub fn new(transaction: i32) -> TransactionInfo {
        TransactionInfo {
            transaction,
            snssai: None,
            app_ids: None,
            transac_metrics: None,
        }
    }
}


