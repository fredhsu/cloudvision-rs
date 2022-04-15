use serde::{Deserialize, Serialize};

pub const TAG_ASSIGNMENT_CONFIG_URL: &str = "/api/resources/tag/v2/TagAssignmentConfig/all";

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
pub enum TagAssignmentServiceResponse {
    Result(Box<TagAssignmentConfigStreamResponse>),
    Error,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TagAssignmentConfigStreamResponse {
    value: TagAssignmentConfig,
    time: String,
    #[serde(rename = "type")]
    operation_type: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TagAssignmentConfig {
    key: TagAssignmentKey,
    remove: Option<bool>,
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TagAssignmentKey {
    workspace_id: String,
    element_type: ElementType,
    label: String,
    value: String,
    device_id: String,
    interface_id: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum ElementType {
    #[serde(rename = "ELEMENT_TYPE_UNSPECIFIED")]
    Unspecified,
    #[serde(rename = "ELEMENT_TYPE_DEVICE")]
    Device,
    #[serde(rename = "ELEMENT_TYPE_INTERFACE")]
    Interface,
}
