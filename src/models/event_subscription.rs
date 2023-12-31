/*
 * Nsmf_EventExposure
 *
 * Session Management Event Exposure Service.   © 2023, 3GPP Organizational Partners (ARIB, ATIS, CCSA, ETSI, TSDSI, TTA, TTC).   All rights reserved. 
 *
 * The version of the OpenAPI document: 1.2.2
 * 
 * Generated by: https://openapi-generator.tech
 */

/// EventSubscription : Represents a subscription to a single event.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EventSubscription {
    #[serde(rename = "event")]
    pub event: Box<crate::models::SmfEvent>,
    #[serde(rename = "dnaiChgType", skip_serializing_if = "Option::is_none")]
    pub dnai_chg_type: Option<Box<crate::models::DnaiChangeType>>,
    #[serde(rename = "dddTraDescriptors", skip_serializing_if = "Option::is_none")]
    pub ddd_tra_descriptors: Option<Vec<crate::models::DddTrafficDescriptor>>,
    #[serde(rename = "dddStati", skip_serializing_if = "Option::is_none")]
    pub ddd_stati: Option<Vec<crate::models::DlDataDeliveryStatus>>,
    #[serde(rename = "appIds", skip_serializing_if = "Option::is_none")]
    pub app_ids: Option<Vec<String>>,
    #[serde(rename = "targetPeriod", skip_serializing_if = "Option::is_none")]
    pub target_period: Option<Box<crate::models::TimeWindow>>,
    /// Indicates the subscription for UE transaction dispersion collectionon, if it is included and set to \"true\". Default value is \"false\". 
    #[serde(rename = "transacDispInd", skip_serializing_if = "Option::is_none")]
    pub transac_disp_ind: Option<bool>,
    /// Indicates Session Management Transaction metrics.
    #[serde(rename = "transacMetrics", skip_serializing_if = "Option::is_none")]
    pub transac_metrics: Option<Vec<crate::models::TransactionMetric>>,
    #[serde(rename = "ueIpAddr", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub ue_ip_addr: Option<Option<Box<crate::models::IpAddr>>>,
}

impl EventSubscription {
    /// Represents a subscription to a single event.
    pub fn new(event: crate::models::SmfEvent) -> EventSubscription {
        EventSubscription {
            event: Box::new(event),
            dnai_chg_type: None,
            ddd_tra_descriptors: None,
            ddd_stati: None,
            app_ids: None,
            target_period: None,
            transac_disp_ind: None,
            transac_metrics: None,
            ue_ip_addr: None,
        }
    }
}


