use crate::tag::*;
use reqwest::header::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::Path;
use std::{env, fs};
use url::{ParseError, Url};

pub mod client;
pub mod device;
pub mod tag;
pub mod inventory;

/// Wraps error types when working with CloudVision APIs or parsing
#[derive(Debug)]
pub enum CloudVisionError {
    NoToken,
    Request(reqwest::Error),
    JsonParse(serde_json::Error),
    UrlParse(url::ParseError),
    BadClientPort,
}
impl From<reqwest::Error> for CloudVisionError {
    fn from(err: reqwest::Error) -> Self {
        CloudVisionError::Request(err)
    }
}
impl From<serde_json::Error> for CloudVisionError {
    fn from(err: serde_json::Error) -> Self {
        CloudVisionError::JsonParse(err)
    }
}
impl From<url::ParseError> for CloudVisionError {
    fn from(err: url::ParseError) -> Self {
        CloudVisionError::UrlParse(err)
    }
}

#[derive(Debug)]
pub struct Client {
    base_url: Url,
    token: String,
    accept_invalid_certs: bool,
}

impl Client {
    /// Create a new client based on a config
    pub fn new(config: Config) -> Result<Self, CloudVisionError> {
        let url = format!("https://{}/", &config.hostname);
        let mut url = Url::parse(&url)?;
        url.set_port(config.port)
            .map_err(|_| CloudVisionError::BadClientPort)?;
        Ok(Self {
            base_url: url,
            token: config.token,
            accept_invalid_certs: config.accept_invalid_certs,
        })
    }

    /// Use to allow or disallow invalid certificates when making calls, default is false, use this
    /// to set to true
    pub fn set_accept_invalid_certs(&mut self, accept: bool) {
        self.accept_invalid_certs = accept;
    }

    /// Returns a clone of the base url
    pub fn base_url(&self) -> Url {
        self.base_url.clone()
    }

    /// Takes a path and returns a full url built upon the base
    pub fn build_url(&self, path: &str) -> Url {
        let mut url = self.base_url.clone();
        url.set_path(path);
        url
    }

    /// Given an API path, perform a GET and return the result or Error
    /// TODO: return something better than a String maybe return the raw response
    /// then you can run the .json() decoder from reqwest?
    pub async fn get(&self, path: &str) -> Result<String, CloudVisionError> {
        let url = self.build_url(path);
        let client = reqwest::Client::builder()
            .danger_accept_invalid_certs(self.accept_invalid_certs)
            .build()?;
        let response = client
            .get(url)
            .header(ACCEPT, "application/json")
            .bearer_auth(&self.token)
            .send()
            .await?
            .text()
            .await?;
        Ok(response)
    }

    /// Given an API path, perform a GET and return the result or Error
    /// TODO: return something better than a String
    pub async fn post(&self, path: &str, body: String) -> Result<String, CloudVisionError> {
        let url = self.build_url(path);
        let client = reqwest::Client::builder()
            .danger_accept_invalid_certs(self.accept_invalid_certs)
            .build()?;
        let response = client
            .post(url)
            .header(ACCEPT, "application/json")
            .bearer_auth(&self.token)
            .body(body)
            .send()
            .await?
            .text()
            .await?;
        Ok(response)
    }

    pub async fn get_change_control(&self, key: &str) -> Result<String, CloudVisionError> {
        let path = "/api/resources/tag/v2/ChangeControl/all";
        self.get(path).await
    }

    /// Gets inventory matching the specified key and filter to get all use an empty filter
    pub async fn get_devices(
        &self,
        filter: &PartialEqFilter,
    ) -> Result<Vec<device::DeviceServiceResponse>, CloudVisionError> {
        let path = "/api/resources/inventory/v1/Device/all";
        let json_data = serde_json::to_string(filter)?;
        let response = self.post(path, json_data).await?;
        // Using a stream Deserializer to parse the returned stream of JSON
        let dsr: Vec<device::DeviceServiceResponse> = serde_json::Deserializer::from_str(&response)
            .into_iter::<device::DeviceServiceResponse>()
            .filter_map(|x| x.ok())
            .collect();
        Ok(dsr)
    }

    /// get_tag_assignment_config will retreive configurations for tag assignment between network
    /// element and tag
    pub async fn get_tag_assignment_config(
        &self,
        filter: &PartialEqFilter,
    ) -> Result<Vec<tag::TagAssignmentServiceResponse>, CloudVisionError> {
        let path = tag::TAG_ASSIGNMENT_CONFIG_URL;
        let json_data = serde_json::to_string(filter)?;
        let response = self.post(path, json_data).await?;
        // Using a stream Deserializer to parse the returned stream of JSON
        let tacr: Vec<tag::TagAssignmentServiceResponse> =
            serde_json::Deserializer::from_str(&response)
                .into_iter::<tag::TagAssignmentServiceResponse>()
                .filter_map(|x| x.ok())
                .collect();
        Ok(tacr)
    }

    /// Gets tags matching the specified key and filter
    pub async fn get_tags(
        &self,
        filter: &PartialEqFilter,
    ) -> Result<Vec<tag::TagServiceResponse>, CloudVisionError> {
        let path = "/api/resources/tag/v2/Tag/all";
        let json_data = serde_json::to_string(filter)?;
        let response = self.post(path, json_data).await?;
        let tsr: Vec<tag::TagServiceResponse> = serde_json::Deserializer::from_str(&response)
            .into_iter::<tag::TagServiceResponse>()
            .filter_map(|x| x.ok())
            .collect();
        Ok(tsr)
    }

    /// Gets all tags
    pub async fn get_all_tags(&self) -> Result<Vec<tag::TagServiceResponse>, CloudVisionError> {
        let workspace_key = TagKey::new();
        let filter = Tag::new(workspace_key);
        let data = PartialEqFilter {
            partial_eq_filter: vec![filter],
        };
        self.get_tags(&data).await
    }

    /// Creates a tag
    pub async fn create_tag(
        &self,
        tag_config: TagConfig,
    ) -> Result<tag::TagConfigResponse, CloudVisionError> {
        let path = tag::TAG_CONFIG_URL;
        let json_data = serde_json::to_string(&tag_config)?;
        let response = self.post(path, json_data).await?;
        println!("{}", &response);
        let tcsr = serde_json::from_str(&response)?;
        Ok(tcsr)
    }
}

/// Config stores the information need to connect to CloudVision
#[derive(Debug, PartialEq, Deserialize)]
pub struct Config {
    hostname: String,
    port: Option<u16>,
    token: String,
    #[serde(default)]
    accept_invalid_certs: bool,
}

impl Config {
    /// Builds a new configuration with given parameters
    pub fn new(hostname: String, port: Option<u16>, token: String) -> Self {
        Self {
            hostname,
            port,
            token,
            accept_invalid_certs: false,
        }
    }
    /// Builds a configuration from environment variables
    pub fn from_env() -> Self {
        let hostname = env::var("CLOUDVISION_HOSTNAME").unwrap();
        let port = Some(env::var("CLOUDVISION_PORT").unwrap().parse().unwrap());
        let token = env::var("CLOUDVISION_TOKEN").unwrap();
        Self::new(hostname, port, token)
    }

    /// Builds a configuration reading a file in TOML format
    pub fn from_file(path: &Path) -> Self {
        let toml = fs::read_to_string(path).unwrap();
        toml::from_str(&toml).unwrap()
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PartialEqFilter {
    partial_eq_filter: Vec<Tag>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Change {
    name: String,
    root_stage_id: String,
    stages: StageMap,
    notes: String,
    time: Option<String>,
    user: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct StageMap {
    values: HashMap<String, Stage>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Stage {
    name: String,
    action: Action,
    rows: String,
    status: StageStatus,
    error: Option<String>,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct Action {
    name: String,
    timeout: u32,
    args: Arg,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct Arg {
    values: HashMap<String, String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum StageStatus {
    StageStatusUnspecified,
    StageStatusRunning,
    StageStatusCompleted,
}

#[cfg(test)]
mod tests {
    use super::*;
    struct Setup {
        config: Config,
    }
    impl Setup {
        fn new() -> Self {
            Self {
                config: Config::new(
                    "www.cv-staging.corp.arista.io".to_string(),
                    Some(443),
                    "token".to_string(),
                ),
            }
        }
    }
    #[test]
    fn test_client_new() {
        let base = Setup::new();
        let client = Client::new(base.config).unwrap();
        assert_eq!(
            client.base_url().to_string(),
            "https://www.cv-staging.corp.arista.io/".to_string()
        );
        // TODO some negative cases, bad url, bad port, assigning port to bad url
    }
    #[test]
    fn test_build_url() {
        let base = Setup::new();
        let client = Client::new(base.config).unwrap();
        let url = client.build_url("/api/resources/v2/tagAll/");
        assert_eq!(
            url.to_string(),
            "https://www.cv-staging.corp.arista.io/api/resources/v2/tagAll/".to_string()
        );
    }
    #[tokio::test]
    async fn test_get() {
        let client =
            Client::new(Config::from_file(Path::new("config/cloudvision.config"))).unwrap();
        let results = client.get("/api/resources/v1/Event/all").await.unwrap();
        assert!(!results.is_empty());
    }
    #[tokio::test]
    async fn test_post() {
        let client =
            Client::new(Config::from_file(Path::new("config/cloudvision.config"))).unwrap();
        let results = client
            .post("/api/resources/v1/Event/all", "foo".to_string())
            .await
            .unwrap();
        assert!(!results.is_empty());
    }
    #[tokio::test]
    async fn test_get_all_tag_assignment_config() {
        let client =
            Client::new(Config::from_file(Path::new("config/cloudvision.config"))).unwrap();
        let filter = PartialEqFilter {
            partial_eq_filter: Vec::new(),
        };
        let stream = client.get_tag_assignment_config(&filter).await.unwrap();
        println!("{:?}", &stream);
        // Using an arbitrary number assuming the demo account has 4 devices at all times
        assert!(!stream.len() > 4);
    }
    #[tokio::test]
    async fn test_get_all_devices() {
        let client =
            Client::new(Config::from_file(Path::new("config/cloudvision.config"))).unwrap();
        let filter = PartialEqFilter {
            partial_eq_filter: Vec::new(),
        };
        let stream = client.get_devices(&filter).await.unwrap();
        // Using an arbitrary number assuming the demo account has 4 devices at all times
        assert!(!stream.len() > 4);
    }
    #[tokio::test]
    async fn test_get_tags() {
        // Getting everything matching a specific tag
        let client =
            Client::new(Config::from_file(Path::new("config/cloudvision.config"))).unwrap();
        let mut tag_key = TagKey::new();
        tag_key.set_label("router_bgp.as", "65002");
        let tag = Tag::new(tag_key);

        let filter = PartialEqFilter {
            partial_eq_filter: vec![tag],
        };
        let results = client.get_tags(&filter).await.unwrap();
        assert!(results.len() == 1);

        // Testing bogus label that should return no results
        let mut tag_key = TagKey::new();
        tag_key.set_label("foo.as", "65002");
        let tag = Tag::new(tag_key);

        let filter = PartialEqFilter {
            partial_eq_filter: vec![tag],
        };
        let results = client.get_tags(&filter).await.unwrap();
        assert!(results.len() == 0);
    }
    #[tokio::test]
    async fn test_get_all_tags() {
        let client =
            Client::new(Config::from_file(Path::new("config/cloudvision.config"))).unwrap();
        let results = client.get_all_tags().await.unwrap();
        println!("there are {} results", results.len());
        assert!(!results.is_empty());
    }
    #[tokio::test]
    async fn test_create_tag() {
        let client =
            Client::new(Config::from_file(Path::new("config/cloudvision.config"))).unwrap();
        let mut tag_key = TagKey::new();
        tag_key.set_workspace_id("8b0eecf4-cd27-430d-ab25-e5aeaca4cf14");
        tag_key.set_element_type(ElementType::Device);
        tag_key.set_label("createtag", "foo");
        let tag_config = TagConfig::new(tag_key, false);
        let results = client.create_tag(tag_config).await.unwrap();
        assert!(results.value.key.get_label().unwrap() == "createtag");
    }
    #[tokio::test]
    async fn test_get_change_control() {
        let client =
            Client::new(Config::from_file(Path::new("config/cloudvision.config"))).unwrap();
        let results = client.get_change_control("").await.unwrap();
        println!("{:?}", results);
        assert!(!results.is_empty());
    }
    #[test]
    fn test_config_new() {
        let config = Config::new(
            "www.cv-staging.arista.io".to_string(),
            Some(443),
            "token".to_string(),
        );
        assert_eq!(config.hostname, "www.cv-staging.arista.io".to_string());
        assert_eq!(config.port, Some(443));
        assert_eq!(config.token, "token".to_string());
    }
    #[test]
    fn test_config_from_file() {
        let base = Setup::new();
        let config = Config::from_file(Path::new("test.config"));
        assert_eq!(config, base.config);
    }
    #[test]
    fn test_config_from_env() {
        let base = Setup::new();
        env::set_var("CLOUDVISION_HOSTNAME", "www.cv-staging.corp.arista.io");
        env::set_var("CLOUDVISION_PORT", "443");
        env::set_var("CLOUDVISION_TOKEN", "token");
        let config = Config::from_env();
        assert_eq!(config, base.config);
    }
}
