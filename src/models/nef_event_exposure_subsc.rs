/*
 * Nnef_EventExposure
 *
 * NEF Event Exposure Service.   © 2022 , 3GPP Organizational Partners (ARIB, ATIS, CCSA, ETSI, TSDSI, TTA, TTC).   All rights reserved. 
 *
 * The version of the OpenAPI document: 1.2.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// NefEventExposureSubsc : Represents an Individual Network Exposure Event Subscription resource.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct NefEventExposureSubsc {
    #[serde(rename = "dataAccProfId", skip_serializing_if = "Option::is_none")]
    pub data_acc_prof_id: Option<String>,
    #[serde(rename = "eventsSubs")]
    pub events_subs: Vec<crate::models::NefEventSubs>,
    #[serde(rename = "eventsRepInfo", skip_serializing_if = "Option::is_none")]
    pub events_rep_info: Option<Box<crate::models::ReportingInformation>>,
    /// String providing an URI formatted according to RFC 3986.
    #[serde(rename = "notifUri")]
    pub notif_uri: String,
    #[serde(rename = "notifId")]
    pub notif_id: String,
    #[serde(rename = "eventNotifs", skip_serializing_if = "Option::is_none")]
    pub event_notifs: Option<Vec<crate::models::NefEventNotification>>,
    /// A string used to indicate the features supported by an API that is used as defined in clause  6.6 in 3GPP TS 29.500. The string shall contain a bitmask indicating supported features in  hexadecimal representation Each character in the string shall take a value of \"0\" to \"9\",  \"a\" to \"f\" or \"A\" to \"F\" and shall represent the support of 4 features as described in  table 5.2.2-3. The most significant character representing the highest-numbered features shall  appear first in the string, and the character representing features 1 to 4 shall appear last  in the string. The list of features and their numbering (starting with 1) are defined  separately for each API. If the string contains a lower number of characters than there are  defined features for an API, all features that would be represented by characters that are not  present in the string are not supported. 
    #[serde(rename = "suppFeat", skip_serializing_if = "Option::is_none")]
    pub supp_feat: Option<String>,
}

impl NefEventExposureSubsc {
    /// Represents an Individual Network Exposure Event Subscription resource.
    pub fn new(events_subs: Vec<crate::models::NefEventSubs>, notif_uri: String, notif_id: String) -> NefEventExposureSubsc {
        NefEventExposureSubsc {
            data_acc_prof_id: None,
            events_subs,
            events_rep_info: None,
            notif_uri,
            notif_id,
            event_notifs: None,
            supp_feat: None,
        }
    }
}

