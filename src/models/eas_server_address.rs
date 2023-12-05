/*
 * Nsmf_EventExposure
 *
 * Session Management Event Exposure Service.   © 2023, 3GPP Organizational Partners (ARIB, ATIS, CCSA, ETSI, TSDSI, TTA, TTC).   All rights reserved. 
 *
 * The version of the OpenAPI document: 1.2.2
 * 
 * Generated by: https://openapi-generator.tech
 */

/// EasServerAddress : Represents the IP address and port of an EAS server.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EasServerAddress {
    #[serde(rename = "ip", deserialize_with = "Option::deserialize")]
    pub ip: Option<Box<crate::models::IpAddr>>,
    /// Unsigned Integer, i.e. only value 0 and integers above 0 are permissible.
    #[serde(rename = "port")]
    pub port: i32,
}

impl EasServerAddress {
    /// Represents the IP address and port of an EAS server.
    pub fn new(ip: Option<crate::models::IpAddr>, port: i32) -> EasServerAddress {
        EasServerAddress {
            ip: if let Some(x) = ip {Some(Box::new(x))} else {None},
            port,
        }
    }
}

