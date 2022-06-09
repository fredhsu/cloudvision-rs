use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
pub enum DeviceServiceResponse {
    Result(Box<DeviceStreamResponse>),
    Error,
}

// TODO use proper time instead of string
#[derive(Serialize, Deserialize, Debug)]
pub struct DeviceRequest {
    key: DeviceKey,
    time: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DeviceResponse {
    value: Device,
    time: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DeviceStreamRequest {
    partial_eq_filter: Vec<Device>,
    time: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DeviceStreamResponse {
    value: Device,
    time: String,
    #[serde(rename = "type")]
    operation_type: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Device {
    key: DeviceKey,
    software_version: String,
    model_name: String,
    hardware_revision: String,
    fqdn: String,
    hostname: String,
    domain_name: String,
    system_mac_address: String,
    boot_time: String,
    streaming_status: StreamingStatus,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct DeviceKey {
    device_id: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum StreamingStatus {
    #[serde(rename = "STREAMING_STATUS_UNSPECIFIED")]
    Unspecified,
    #[serde(rename = "STREAMING_STATUS_INACTIVE")]
    Inactive,
    #[serde(rename = "STREAMING_STATUS_ACTIVE")]
    Active,
}
