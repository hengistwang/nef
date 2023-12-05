/*
 * Nnef_EventExposure
 *
 * NEF Event Exposure Service.   © 2022 , 3GPP Organizational Partners (ARIB, ATIS, CCSA, ETSI, TSDSI, TTA, TTC).   All rights reserved. 
 *
 * The version of the OpenAPI document: 1.2.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// FlowInfo : Represents IP flow information.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FlowInfo {
    /// Indicates the IP flow identifier.
    #[serde(rename = "flowId")]
    pub flow_id: i32,
    /// Indicates the packet filters of the IP flow. Refer to clause 5.3.8 of 3GPP TS 29.214 for encoding. It shall contain UL and/or DL IP flow description. 
    #[serde(rename = "flowDescriptions", skip_serializing_if = "Option::is_none")]
    pub flow_descriptions: Option<Vec<String>>,
}

impl FlowInfo {
    /// Represents IP flow information.
    pub fn new(flow_id: i32) -> FlowInfo {
        FlowInfo {
            flow_id,
            flow_descriptions: None,
        }
    }
}


