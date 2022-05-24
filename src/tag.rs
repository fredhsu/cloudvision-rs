use serde::{Deserialize, Serialize};

pub const TAG_ASSIGNMENT_CONFIG_URL: &str = "/api/resources/tag/v2/TagAssignmentConfig/all";
pub const TAG_CONFIG_URL: &str = "/api/resources/tag/v2/TagConfig";
pub const TAG_URL: &str = "/api/resources/tag/v2/Tag/all";

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum Response {
    TasrResult(Box<TagAssignmentConfigStreamResponse>),
    TsrResult(Box<TagStreamResponse>),
    Error,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
pub enum TagAssignmentServiceResponse {
    Result(Box<TagAssignmentConfigStreamResponse>),
    Error,
}

/// TagServiceResponse handles responses from method calls against the tag service
// TODO handle a single getOne vs tagstream
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
pub enum TagServiceResponse {
    Result(Box<TagStreamResponse>),
    Error,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TagStreamResponse {
    value: Tag,
    time: String,
    #[serde(rename = "type")]
    operation_type: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Tag {
    key: TagKey,
    creator_type: Option<CreatorType>,
}

impl Tag {
    pub fn new(key: TagKey) -> Self {
        Tag {
            key,
            creator_type: None,
        }
    }
}

/// TagConfig is used to CRUD tags
#[derive(Serialize, Deserialize, Debug)]
pub struct TagConfig {
    pub key: TagKey,
    pub remove: bool,
}

impl TagConfig {
    pub fn new(key: TagKey, remove: bool) -> Self {
        TagConfig { key, remove }
    }
}

// TODO work through tag config options
#[derive(Serialize, Deserialize, Debug)]
pub struct TagConfigRequest {
    pub key: TagKey,
    pub time: Option<String>,
}

/// TagConfigResponse handles all the single action TagConfig responses: TagConfig, TagConfigSet,
/// and TagConfigDelete
#[derive(Serialize, Deserialize, Debug)]
pub struct TagConfigResponse {
    pub value: TagConfig,
    pub time: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TagKey {
    workspace_id: Option<String>,
    element_type: Option<ElementType>,
    label: Option<String>,
    value: Option<String>,
}

impl TagKey {
    pub fn new() -> Self {
        TagKey {
            workspace_id: None,
            element_type: None,
            label: None,
            value: None,
        }
    }
    pub fn set_workspace_id(&mut self, workspace_id: &str) {
        self.workspace_id = Some(workspace_id.to_owned());
    }
    pub fn get_label(&self) -> Option<&String> {
        self.label.as_ref()
    }
    pub fn set_label(&mut self, label: &str, value: &str) {
        self.label = Some(label.to_owned());
        self.value = Some(value.to_owned());
    }
    pub fn set_element_type(&mut self, et: ElementType) {
        self.element_type = Some(et);
    }
    pub fn get_workspace_id(&self) -> Option<&String> {
        self.workspace_id.as_ref()
    }
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

#[derive(Serialize, Deserialize, Debug)]
pub enum CreatorType {
    #[serde(rename = "CREATOR_TYPE_UNSPECIFIED")]
    Unspecified,
    #[serde(rename = "CREATOR_TYPE_SYSTEM")]
    System,
    #[serde(rename = "CREATOR_TYPE_USER")]
    User,
}
