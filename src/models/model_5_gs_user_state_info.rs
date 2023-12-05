/*
 * Namf_EventExposure
 *
 * AMF Event Exposure Service.   © 2023, 3GPP Organizational Partners (ARIB, ATIS, CCSA, ETSI, TSDSI, TTA, TTC).   All rights reserved. 
 *
 * The version of the OpenAPI document: 1.2.3
 * 
 * Generated by: https://openapi-generator.tech
 */

/// Model5GsUserStateInfo : Represents the 5GS User state of the UE for an access type



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Model5GsUserStateInfo {
    #[serde(rename = "5gsUserState")]
    pub param_5gs_user_state: Box<crate::models::Model5GsUserState>,
    #[serde(rename = "accessType")]
    pub access_type: crate::models::AccessType,
}

impl Model5GsUserStateInfo {
    /// Represents the 5GS User state of the UE for an access type
    pub fn new(param_5gs_user_state: crate::models::Model5GsUserState, access_type: crate::models::AccessType) -> Model5GsUserStateInfo {
        Model5GsUserStateInfo {
            param_5gs_user_state: Box::new(param_5gs_user_state),
            access_type,
        }
    }
}


