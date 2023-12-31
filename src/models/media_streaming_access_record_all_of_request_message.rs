/*
 * Nnef_EventExposure
 *
 * NEF Event Exposure Service.   © 2022 , 3GPP Organizational Partners (ARIB, ATIS, CCSA, ETSI, TSDSI, TTA, TTC).   All rights reserved. 
 *
 * The version of the OpenAPI document: 1.2.0
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MediaStreamingAccessRecordAllOfRequestMessage {
    #[serde(rename = "method")]
    pub method: String,
    /// Absolute Uniform Resource Locator, conforming with the \"absolute-URI\" production specified in IETF RFC 3986, section 4.3 in which the scheme part is \"http\" or \"https\". Note that the \"query\" suffix is permitted by this production but the \"fragment\" suffix is not.
    #[serde(rename = "url")]
    pub url: String,
    #[serde(rename = "protocolVersion")]
    pub protocol_version: String,
    #[serde(rename = "range", skip_serializing_if = "Option::is_none")]
    pub range: Option<String>,
    /// Unsigned Integer, i.e. only value 0 and integers above 0 are permissible.
    #[serde(rename = "size")]
    pub size: i32,
    /// Unsigned Integer, i.e. only value 0 and integers above 0 are permissible.
    #[serde(rename = "bodySize")]
    pub body_size: i32,
    #[serde(rename = "contentType", skip_serializing_if = "Option::is_none")]
    pub content_type: Option<String>,
    #[serde(rename = "userAgent", skip_serializing_if = "Option::is_none")]
    pub user_agent: Option<String>,
    #[serde(rename = "userIdentity", skip_serializing_if = "Option::is_none")]
    pub user_identity: Option<String>,
    /// Absolute Uniform Resource Locator, conforming with the \"absolute-URI\" production specified in IETF RFC 3986, section 4.3 in which the scheme part is \"http\" or \"https\". Note that the \"query\" suffix is permitted by this production but the \"fragment\" suffix is not.
    #[serde(rename = "referer", skip_serializing_if = "Option::is_none")]
    pub referer: Option<String>,
}

impl MediaStreamingAccessRecordAllOfRequestMessage {
    pub fn new(method: String, url: String, protocol_version: String, size: i32, body_size: i32) -> MediaStreamingAccessRecordAllOfRequestMessage {
        MediaStreamingAccessRecordAllOfRequestMessage {
            method,
            url,
            protocol_version,
            range: None,
            size,
            body_size,
            content_type: None,
            user_agent: None,
            user_identity: None,
            referer: None,
        }
    }
}


