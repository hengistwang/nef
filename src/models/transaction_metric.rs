/*
 * Nsmf_EventExposure
 *
 * Session Management Event Exposure Service.   © 2023, 3GPP Organizational Partners (ARIB, ATIS, CCSA, ETSI, TSDSI, TTA, TTC).   All rights reserved. 
 *
 * The version of the OpenAPI document: 1.2.2
 * 
 * Generated by: https://openapi-generator.tech
 */

/// TransactionMetric : Possible values are: - PDU_SES_EST: PDU Session Establishment - PDU_SES_AUTH: PDU Session Authentication - PDU_SES_MODIF: PDU Session Modification - PDU_SES_REL: PDU Session Release 



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TransactionMetric {
}

impl TransactionMetric {
    /// Possible values are: - PDU_SES_EST: PDU Session Establishment - PDU_SES_AUTH: PDU Session Authentication - PDU_SES_MODIF: PDU Session Modification - PDU_SES_REL: PDU Session Release 
    pub fn new() -> TransactionMetric {
        TransactionMetric {
        }
    }
}


