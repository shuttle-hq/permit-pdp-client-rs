/*
 * Permit.io PDP
 *
 * The PDP (Policy decision point) container wraps Open Policy Agent (OPA) with a higher-level API intended for fine grained application-level authorization. The PDP automatically handles pulling policy updates in real-time from a centrally managed cloud-service (api.permit.io).
 *
 * The version of the OpenAPI document: 0.2.0
 * 
 * Generated by: https://openapi-generator.tech
 */

use crate::models;

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct User {
    #[serde(rename = "key")]
    pub key: String,
    #[serde(rename = "firstName", skip_serializing_if = "Option::is_none")]
    pub first_name: Option<String>,
    #[serde(rename = "lastName", skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,
    #[serde(rename = "email", skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(rename = "attributes", skip_serializing_if = "Option::is_none")]
    pub attributes: Option<serde_json::Value>,
}

impl User {
    pub fn new(key: String) -> User {
        User {
            key,
            first_name: None,
            last_name: None,
            email: None,
            attributes: None,
        }
    }
}

