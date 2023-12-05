/*
 * Nnef_EventExposure
 *
 * NEF Event Exposure Service.   © 2022 , 3GPP Organizational Partners (ARIB, ATIS, CCSA, ETSI, TSDSI, TTA, TTC).   All rights reserved. 
 *
 * The version of the OpenAPI document: 1.2.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// UeCommunicationInfo : Contains UE communication information associated with an application.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UeCommunicationInfo {
    /// String identifying a Supi that shall contain either an IMSI, a network specific identifier, a Global Cable Identifier (GCI) or a Global Line Identifier (GLI) as specified in clause  2.2A of 3GPP TS 23.003. It shall be formatted as follows  - for an IMSI \"imsi-<imsi>\", where <imsi> shall be formatted according to clause 2.2    of 3GPP TS 23.003 that describes an IMSI.  - for a network specific identifier \"nai-<nai>, where <nai> shall be formatted    according to clause 28.7.2 of 3GPP TS 23.003 that describes an NAI.  - for a GCI \"gci-<gci>\", where <gci> shall be formatted according to clause 28.15.2    of 3GPP TS 23.003.  - for a GLI \"gli-<gli>\", where <gli> shall be formatted according to clause 28.16.2 of    3GPP TS 23.003.To enable that the value is used as part of an URI, the string shall    only contain characters allowed according to the \"lower-with-hyphen\" naming convention    defined in 3GPP TS 29.501. 
    #[serde(rename = "supi", skip_serializing_if = "Option::is_none")]
    pub supi: Option<String>,
    /// String identifying a group of devices network internal globally unique ID which identifies a set of IMSIs, as specified in clause 19.9 of 3GPP TS 23.003.  
    #[serde(rename = "interGroupId", skip_serializing_if = "Option::is_none")]
    pub inter_group_id: Option<String>,
    /// String providing an application identifier.
    #[serde(rename = "appId", skip_serializing_if = "Option::is_none")]
    pub app_id: Option<String>,
    #[serde(rename = "comms")]
    pub comms: Vec<crate::models::CommunicationCollection>,
}

impl UeCommunicationInfo {
    /// Contains UE communication information associated with an application.
    pub fn new(comms: Vec<crate::models::CommunicationCollection>) -> UeCommunicationInfo {
        UeCommunicationInfo {
            supi: None,
            inter_group_id: None,
            app_id: None,
            comms,
        }
    }
}

