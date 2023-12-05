/*
 * Nsmf_EventExposure
 *
 * Session Management Event Exposure Service.   © 2023, 3GPP Organizational Partners (ARIB, ATIS, CCSA, ETSI, TSDSI, TTA, TTC).   All rights reserved. 
 *
 * The version of the OpenAPI document: 1.2.2
 * 
 * Generated by: https://openapi-generator.tech
 */

/// NotificationMethod : Possible values are: - PERIODIC - ONE_TIME - ON_EVENT_DETECTION 



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct NotificationMethod {
}

impl NotificationMethod {
    /// Possible values are: - PERIODIC - ONE_TIME - ON_EVENT_DETECTION 
    pub fn new() -> NotificationMethod {
        NotificationMethod {
        }
    }
}


