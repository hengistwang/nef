/*
 * Nnef_EventExposure
 *
 * NEF Event Exposure Service.   © 2022 , 3GPP Organizational Partners (ARIB, ATIS, CCSA, ETSI, TSDSI, TTA, TTC).   All rights reserved. 
 *
 * The version of the OpenAPI document: 1.2.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// CivicAddress : Indicates a Civic address.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CivicAddress {
    #[serde(rename = "country", skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    #[serde(rename = "A1", skip_serializing_if = "Option::is_none")]
    pub a1: Option<String>,
    #[serde(rename = "A2", skip_serializing_if = "Option::is_none")]
    pub a2: Option<String>,
    #[serde(rename = "A3", skip_serializing_if = "Option::is_none")]
    pub a3: Option<String>,
    #[serde(rename = "A4", skip_serializing_if = "Option::is_none")]
    pub a4: Option<String>,
    #[serde(rename = "A5", skip_serializing_if = "Option::is_none")]
    pub a5: Option<String>,
    #[serde(rename = "A6", skip_serializing_if = "Option::is_none")]
    pub a6: Option<String>,
    #[serde(rename = "PRD", skip_serializing_if = "Option::is_none")]
    pub prd: Option<String>,
    #[serde(rename = "POD", skip_serializing_if = "Option::is_none")]
    pub pod: Option<String>,
    #[serde(rename = "STS", skip_serializing_if = "Option::is_none")]
    pub sts: Option<String>,
    #[serde(rename = "HNO", skip_serializing_if = "Option::is_none")]
    pub hno: Option<String>,
    #[serde(rename = "HNS", skip_serializing_if = "Option::is_none")]
    pub hns: Option<String>,
    #[serde(rename = "LMK", skip_serializing_if = "Option::is_none")]
    pub lmk: Option<String>,
    #[serde(rename = "LOC", skip_serializing_if = "Option::is_none")]
    pub loc: Option<String>,
    #[serde(rename = "NAM", skip_serializing_if = "Option::is_none")]
    pub nam: Option<String>,
    #[serde(rename = "PC", skip_serializing_if = "Option::is_none")]
    pub pc: Option<String>,
    #[serde(rename = "BLD", skip_serializing_if = "Option::is_none")]
    pub bld: Option<String>,
    #[serde(rename = "UNIT", skip_serializing_if = "Option::is_none")]
    pub unit: Option<String>,
    #[serde(rename = "FLR", skip_serializing_if = "Option::is_none")]
    pub flr: Option<String>,
    #[serde(rename = "ROOM", skip_serializing_if = "Option::is_none")]
    pub room: Option<String>,
    #[serde(rename = "PLC", skip_serializing_if = "Option::is_none")]
    pub plc: Option<String>,
    #[serde(rename = "PCN", skip_serializing_if = "Option::is_none")]
    pub pcn: Option<String>,
    #[serde(rename = "POBOX", skip_serializing_if = "Option::is_none")]
    pub pobox: Option<String>,
    #[serde(rename = "ADDCODE", skip_serializing_if = "Option::is_none")]
    pub addcode: Option<String>,
    #[serde(rename = "SEAT", skip_serializing_if = "Option::is_none")]
    pub seat: Option<String>,
    #[serde(rename = "RD", skip_serializing_if = "Option::is_none")]
    pub rd: Option<String>,
    #[serde(rename = "RDSEC", skip_serializing_if = "Option::is_none")]
    pub rdsec: Option<String>,
    #[serde(rename = "RDBR", skip_serializing_if = "Option::is_none")]
    pub rdbr: Option<String>,
    #[serde(rename = "RDSUBBR", skip_serializing_if = "Option::is_none")]
    pub rdsubbr: Option<String>,
    #[serde(rename = "PRM", skip_serializing_if = "Option::is_none")]
    pub prm: Option<String>,
    #[serde(rename = "POM", skip_serializing_if = "Option::is_none")]
    pub pom: Option<String>,
    #[serde(rename = "usageRules", skip_serializing_if = "Option::is_none")]
    pub usage_rules: Option<String>,
    #[serde(rename = "method", skip_serializing_if = "Option::is_none")]
    pub method: Option<String>,
    #[serde(rename = "providedBy", skip_serializing_if = "Option::is_none")]
    pub provided_by: Option<String>,
}

impl CivicAddress {
    /// Indicates a Civic address.
    pub fn new() -> CivicAddress {
        CivicAddress {
            country: None,
            a1: None,
            a2: None,
            a3: None,
            a4: None,
            a5: None,
            a6: None,
            prd: None,
            pod: None,
            sts: None,
            hno: None,
            hns: None,
            lmk: None,
            loc: None,
            nam: None,
            pc: None,
            bld: None,
            unit: None,
            flr: None,
            room: None,
            plc: None,
            pcn: None,
            pobox: None,
            addcode: None,
            seat: None,
            rd: None,
            rdsec: None,
            rdbr: None,
            rdsubbr: None,
            prm: None,
            pom: None,
            usage_rules: None,
            method: None,
            provided_by: None,
        }
    }
}


