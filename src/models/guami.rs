/*
 * Namf_EventExposure
 *
 * AMF Event Exposure Service.   © 2023, 3GPP Organizational Partners (ARIB, ATIS, CCSA, ETSI, TSDSI, TTA, TTC).   All rights reserved. 
 *
 * The version of the OpenAPI document: 1.2.3
 * 
 * Generated by: https://openapi-generator.tech
 */

/// Guami : Globally Unique AMF Identifier constructed out of PLMN, Network and AMF identity.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Guami {
    #[serde(rename = "plmnId")]
    pub plmn_id: Box<crate::models::PlmnIdNid>,
    /// String identifying the AMF ID composed of AMF Region ID (8 bits), AMF Set ID (10 bits) and AMF  Pointer (6 bits) as specified in clause 2.10.1 of 3GPP TS 23.003. It is encoded as a string of  6 hexadecimal characters (i.e., 24 bits).  
    #[serde(rename = "amfId")]
    pub amf_id: String,
}

impl Guami {
    /// Globally Unique AMF Identifier constructed out of PLMN, Network and AMF identity.
    pub fn new(plmn_id: crate::models::PlmnIdNid, amf_id: String) -> Guami {
        Guami {
            plmn_id: Box::new(plmn_id),
            amf_id,
        }
    }
}


