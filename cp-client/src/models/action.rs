/*
 * Tembo Cloud
 *
 * Platform API for Tembo Cloud             </br>             </br>             To find a Tembo Data API, please find it here:             </br>             </br>             [AWS US East 1](https://api.data-1.use1.tembo.io/swagger-ui/)
 *
 * The version of the OpenAPI document: v1.0.0
 *
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct Action {
    #[serde(rename = "description")]
    pub description: String,
    /// A valid Action ID. Available Action IDs include 'CreateInstance' and 'ManagePermissions'. Find all available Actions on the Actions API.
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "name")]
    pub name: String,
}

impl Action {
    pub fn new(description: String, id: String, name: String) -> Action {
        Action {
            description,
            id,
            name,
        }
    }
}
