/*
 * Namf_EventExposure
 *
 * AMF Event Exposure Service.   © 2023, 3GPP Organizational Partners (ARIB, ATIS, CCSA, ETSI, TSDSI, TTA, TTC).   All rights reserved. 
 *
 * The version of the OpenAPI document: 1.2.3
 * 
 * Generated by: https://openapi-generator.tech
 */

/// GnbId : Provides the G-NB identifier.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GnbId {
    /// Unsigned integer representing the bit length of the gNB ID as defined in clause 9.3.1.6 of 3GPP TS 38.413 [11], within the range 22 to 32. 
    #[serde(rename = "bitLength")]
    pub bit_length: i32,
    /// This represents the identifier of the gNB. The value of the gNB ID shall be encoded in hexadecimal representation. Each character in the string shall take a value of \"0\" to \"9\", \"a\" to \"f\" or \"A\" to \"F\" and shall represent 4 bits. The padding 0 shall be added to make multiple nibbles,  the most significant character representing the padding 0 if required together with the 4 most significant bits of the gNB ID shall appear first in the string, and the character representing the 4 least significant bit of the gNB ID shall appear last in the string. 
    #[serde(rename = "gNBValue")]
    pub g_nb_value: String,
}

impl GnbId {
    /// Provides the G-NB identifier.
    pub fn new(bit_length: i32, g_nb_value: String) -> GnbId {
        GnbId {
            bit_length,
            g_nb_value,
        }
    }
}


