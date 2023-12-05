/*
 * Namf_EventExposure
 *
 * AMF Event Exposure Service.   © 2023, 3GPP Organizational Partners (ARIB, ATIS, CCSA, ETSI, TSDSI, TTA, TTC).   All rights reserved. 
 *
 * The version of the OpenAPI document: 1.2.3
 * 
 * Generated by: https://openapi-generator.tech
 */

/// ProblemDetails : Provides additional information in an error response.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ProblemDetails {
    /// String providing an URI formatted according to RFC 3986.
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[serde(rename = "title", skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<i32>,
    /// A human-readable explanation specific to this occurrence of the problem.
    #[serde(rename = "detail", skip_serializing_if = "Option::is_none")]
    pub detail: Option<String>,
    /// String providing an URI formatted according to RFC 3986.
    #[serde(rename = "instance", skip_serializing_if = "Option::is_none")]
    pub instance: Option<String>,
    /// A machine-readable application error cause specific to this occurrence of the problem.  This IE should be present and provide application-related error information, if available. 
    #[serde(rename = "cause", skip_serializing_if = "Option::is_none")]
    pub cause: Option<String>,
    #[serde(rename = "invalidParams", skip_serializing_if = "Option::is_none")]
    pub invalid_params: Option<Vec<crate::models::InvalidParam>>,
    /// A string used to indicate the features supported by an API that is used as defined in clause  6.6 in 3GPP TS 29.500. The string shall contain a bitmask indicating supported features in  hexadecimal representation Each character in the string shall take a value of \"0\" to \"9\",  \"a\" to \"f\" or \"A\" to \"F\" and shall represent the support of 4 features as described in  table 5.2.2-3. The most significant character representing the highest-numbered features shall  appear first in the string, and the character representing features 1 to 4 shall appear last  in the string. The list of features and their numbering (starting with 1) are defined  separately for each API. If the string contains a lower number of characters than there are  defined features for an API, all features that would be represented by characters that are not  present in the string are not supported. 
    #[serde(rename = "supportedFeatures", skip_serializing_if = "Option::is_none")]
    pub supported_features: Option<String>,
    #[serde(rename = "accessTokenError", skip_serializing_if = "Option::is_none")]
    pub access_token_error: Option<Box<crate::models::AccessTokenErr>>,
    #[serde(rename = "accessTokenRequest", skip_serializing_if = "Option::is_none")]
    pub access_token_request: Option<Box<crate::models::AccessTokenReq>>,
    /// Fully Qualified Domain Name
    #[serde(rename = "nrfId", skip_serializing_if = "Option::is_none")]
    pub nrf_id: Option<String>,
}

impl ProblemDetails {
    /// Provides additional information in an error response.
    pub fn new() -> ProblemDetails {
        ProblemDetails {
            r#type: None,
            title: None,
            status: None,
            detail: None,
            instance: None,
            cause: None,
            invalid_params: None,
            supported_features: None,
            access_token_error: None,
            access_token_request: None,
            nrf_id: None,
        }
    }
}


