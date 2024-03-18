use rs_plugin_common_interfaces::PluginCredential;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Default)]
#[serde(rename_all = "snake_case")] 
pub struct RsRequest {
    pub url: String,
    pub mime: Option<String>,
    pub size: Option<usize>,
    #[serde(default)]
    pub require_add: bool,
    #[serde(default)]
    pub intermediate: bool,
    pub cookies: Option<String>,
    pub headers: Option<Vec<(String, String)>>,
    pub files: Option<Vec<RsRequestFiles>>
}


#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Default)]
#[serde(rename_all = "snake_case")] 
pub struct RsRequestFiles {
    pub name: String,
    pub size: usize,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Default)]
#[serde(rename_all = "snake_case")] 
pub struct RsRequestWithCredential {
    pub request: RsRequest,
    pub credential: Option<PluginCredential>
}