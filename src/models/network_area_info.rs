/*
 * Nnef_EventExposure
 *
 * NEF Event Exposure Service.   © 2022 , 3GPP Organizational Partners (ARIB, ATIS, CCSA, ETSI, TSDSI, TTA, TTC).   All rights reserved. 
 *
 * The version of the OpenAPI document: 1.2.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// NetworkAreaInfo : Describes a network area information in which the NF service consumer requests the number of UEs. 



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct NetworkAreaInfo {
    /// Contains a list of E-UTRA cell identities.
    #[serde(rename = "ecgis", skip_serializing_if = "Option::is_none")]
    pub ecgis: Option<Vec<crate::models::Ecgi>>,
    /// Contains a list of NR cell identities.
    #[serde(rename = "ncgis", skip_serializing_if = "Option::is_none")]
    pub ncgis: Option<Vec<crate::models::Ncgi>>,
    /// Contains a list of NG RAN nodes.
    #[serde(rename = "gRanNodeIds", skip_serializing_if = "Option::is_none")]
    pub g_ran_node_ids: Option<Vec<crate::models::GlobalRanNodeId>>,
    /// Contains a list of tracking area identities.
    #[serde(rename = "tais", skip_serializing_if = "Option::is_none")]
    pub tais: Option<Vec<crate::models::Tai>>,
}

impl NetworkAreaInfo {
    /// Describes a network area information in which the NF service consumer requests the number of UEs. 
    pub fn new() -> NetworkAreaInfo {
        NetworkAreaInfo {
            ecgis: None,
            ncgis: None,
            g_ran_node_ids: None,
            tais: None,
        }
    }
}


