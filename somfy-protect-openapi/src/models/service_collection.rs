/*
 * Somfy Protect API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 3
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ServiceCollection {
    /// List of services.
    #[serde(rename = "items")]
    pub items: Vec<crate::models::Service>,
}

impl ServiceCollection {
    pub fn new(items: Vec<crate::models::Service>) -> ServiceCollection {
        ServiceCollection {
            items,
        }
    }
}


