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
pub struct ServiceDataFlowDescription {
    #[serde(rename = "flowDescription", skip_serializing_if = "Option::is_none")]
    pub flow_description: Option<Box<crate::models::IpPacketFilterSet>>,
    #[serde(rename = "domainName", skip_serializing_if = "Option::is_none")]
    pub domain_name: Option<String>,
}

impl ServiceDataFlowDescription {
    pub fn new() -> ServiceDataFlowDescription {
        ServiceDataFlowDescription {
            flow_description: None,
            domain_name: None,
        }
    }
}


