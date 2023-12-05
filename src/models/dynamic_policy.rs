/*
 * Nnef_EventExposure
 *
 * NEF Event Exposure Service.   © 2022 , 3GPP Organizational Partners (ARIB, ATIS, CCSA, ETSI, TSDSI, TTA, TTC).   All rights reserved. 
 *
 * The version of the OpenAPI document: 1.2.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// DynamicPolicy : A representation of a Dynamic Policy resource.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DynamicPolicy {
    /// String chosen by the 5GMS AF to serve as an identifier in a resource URI.
    #[serde(rename = "dynamicPolicyId")]
    pub dynamic_policy_id: String,
    /// String chosen by the 5GMS AF to serve as an identifier in a resource URI.
    #[serde(rename = "policyTemplateId")]
    pub policy_template_id: String,
    #[serde(rename = "serviceDataFlowDescriptions")]
    pub service_data_flow_descriptions: Vec<crate::models::ServiceDataFlowDescription>,
    #[serde(rename = "mediaType", skip_serializing_if = "Option::is_none")]
    pub media_type: Option<Box<crate::models::MediaType>>,
    /// String chosen by the 5GMS AF to serve as an identifier in a resource URI.
    #[serde(rename = "provisioningSessionId")]
    pub provisioning_session_id: String,
    #[serde(rename = "qosSpecification", skip_serializing_if = "Option::is_none")]
    pub qos_specification: Option<Box<crate::models::M5QoSSpecification>>,
    #[serde(rename = "enforcementMethod", skip_serializing_if = "Option::is_none")]
    pub enforcement_method: Option<String>,
    #[serde(rename = "enforcementBitRate", skip_serializing_if = "Option::is_none")]
    pub enforcement_bit_rate: Option<i32>,
}

impl DynamicPolicy {
    /// A representation of a Dynamic Policy resource.
    pub fn new(dynamic_policy_id: String, policy_template_id: String, service_data_flow_descriptions: Vec<crate::models::ServiceDataFlowDescription>, provisioning_session_id: String) -> DynamicPolicy {
        DynamicPolicy {
            dynamic_policy_id,
            policy_template_id,
            service_data_flow_descriptions,
            media_type: None,
            provisioning_session_id,
            qos_specification: None,
            enforcement_method: None,
            enforcement_bit_rate: None,
        }
    }
}


