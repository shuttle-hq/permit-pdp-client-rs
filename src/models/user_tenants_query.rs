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
pub struct UserTenantsQuery {
    #[serde(rename = "user")]
    pub user: Box<models::User>,
    #[serde(rename = "context", skip_serializing_if = "Option::is_none")]
    pub context: Option<serde_json::Value>,
}

impl UserTenantsQuery {
    pub fn new(user: models::User) -> UserTenantsQuery {
        UserTenantsQuery {
            user: Box::new(user),
            context: None,
        }
    }
}

