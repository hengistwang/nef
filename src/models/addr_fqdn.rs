/*
 * Nsmf_EventExposure
 *
 * Session Management Event Exposure Service.   © 2023, 3GPP Organizational Partners (ARIB, ATIS, CCSA, ETSI, TSDSI, TTA, TTC).   All rights reserved. 
 *
 * The version of the OpenAPI document: 1.2.2
 * 
 * Generated by: https://openapi-generator.tech
 */

/// AddrFqdn : IP address and/or FQDN.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AddrFqdn {
    #[serde(rename = "ipAddr", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub ip_addr: Option<Option<Box<crate::models::IpAddr>>>,
    /// Indicates an FQDN.
    #[serde(rename = "fqdn", skip_serializing_if = "Option::is_none")]
    pub fqdn: Option<String>,
}

impl AddrFqdn {
    /// IP address and/or FQDN.
    pub fn new() -> AddrFqdn {
        AddrFqdn {
            ip_addr: None,
            fqdn: None,
        }
    }
}


