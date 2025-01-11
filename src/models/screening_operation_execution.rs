// Fireblocks API
//
// Fireblocks provides a suite of applications to manage digital asset operations and a complete development platform to build your business on the blockchain.  - Visit our website for more information: [Fireblocks Website](https://fireblocks.com) - Visit our developer docs: [Fireblocks DevPortal](https://developers.fireblocks.com)
//
// The version of the OpenAPI document: 1.8.0
// Contact: developers@fireblocks.com
// Generated by: https://openapi-generator.tech

use {
    crate::models,
    serde::{Deserialize, Serialize},
};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ScreeningOperationExecution {
    #[serde(rename = "output", skip_serializing_if = "Option::is_none")]
    pub output: Option<models::ScreeningOperationExecutionOutput>,
    #[serde(rename = "startedAt")]
    pub started_at: f64,
    #[serde(rename = "finishedAt", skip_serializing_if = "Option::is_none")]
    pub finished_at: Option<f64>,
    #[serde(rename = "failure", skip_serializing_if = "Option::is_none")]
    pub failure: Option<models::ScreeningOperationFailure>,
}

impl ScreeningOperationExecution {
    pub fn new(started_at: f64) -> ScreeningOperationExecution {
        ScreeningOperationExecution {
            output: None,
            started_at,
            finished_at: None,
            failure: None,
        }
    }
}
