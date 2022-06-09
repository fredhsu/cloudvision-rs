use crate::client::{self, CloudVisionError};
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

pub async fn get_device(
    device_id: &str,
    client: client::Client,
) -> Result<Device, CloudVisionError> {
    let path = "/api/resources/inventory/v1/Device";
    let query = "key.deviceId=".to_owned() + device_id;
    let response = client.get(&path, Some(&query)).await?;
    let dr: DeviceResponse = serde_json::from_str(&response)?;
    Ok(dr.value)
}

// TODO find a better way to store creds for testing
#[cfg(test)]
mod tests {
    use super::*;
    fn build_client() -> client::Client {
        let config = client::Config::new(
            "www.cv-staging.corp.arista.io".to_string(),
            Some(443),
            "eyJhbGciOiJFUzI1NiIsInR5cCI6IkpXVCJ9.eyJkaWQiOjMyNzI1MiwiZHNuIjoiZnJlZC1ydXN0eSIsImRzdCI6ImFjY291bnQiLCJleHAiOjE2NzM0NzYxOTcsImlhdCI6MTY0NDQ0NTgwNywic2lkIjoiOWFlNGUzZTMzMjRkODJhNmMzMTRmOGQ0MjNiNmRjMjRmMjE2OGQ4ZTc3YjM5ZmFkZmVmMTZjNDFiMmMzYjliNi11aFVySlMzVWd6bHJYanBFMi1LV0xuSXA1U2xHd0xzTGlJNmVhMUEzIn0.DzGnwSBWYZO66FejB-R9FzkifUVTNkUC7gTMJqPbI4uP64jemMjRT8jZ7fMwQj6O4hiP_Bn0CAzzhvvUxmpHfw".to_string(),
        );
        client::Client::new(config).unwrap()
    }
    #[tokio::test]
    async fn test_get_device() {
        let client = build_client();
        let device_id = "SSJ17200818";
        let device = get_device(device_id, client).await.unwrap();
        assert_eq!(device.model_name, "DCS-7280SR2-48YC6");
    }
}
