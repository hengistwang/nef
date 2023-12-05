/*
 * Nnef_EventExposure
 *
 * NEF Event Exposure Service.   © 2022 , 3GPP Organizational Partners (ARIB, ATIS, CCSA, ETSI, TSDSI, TTA, TTC).   All rights reserved. 
 *
 * The version of the OpenAPI document: 1.2.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// MsConsumptionCollection : Contains the Media Streaming Consumption information collected for an UE Application via AF. 



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MsConsumptionCollection {
    #[serde(rename = "msConsumps")]
    pub ms_consumps: Vec<String>,
}

impl MsConsumptionCollection {
    /// Contains the Media Streaming Consumption information collected for an UE Application via AF. 
    pub fn new(ms_consumps: Vec<String>) -> MsConsumptionCollection {
        MsConsumptionCollection {
            ms_consumps,
        }
    }
}


