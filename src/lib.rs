use rs_plugin_common_interfaces::PluginCredential;
use serde::{Deserialize, Serialize};
use strum_macros::EnumString;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Default)]
#[serde(rename_all = "snake_case")] 
pub struct RsRequest {
    pub url: String,
    pub mime: Option<String>,
    pub size: Option<u64>,
    pub filename: Option<String>,
    #[serde(default)]
    pub status: RsRequestStatus,
    pub headers: Option<Vec<(String, String)>>,
    /// If must choose between multiple files. Recall plugin with a `selected_file` containing one of the name in this list to get link
    pub files: Option<Vec<RsRequestFiles>>,
    /// one of the `files` selected for download
    pub selected_file: Option<String>
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, strum_macros::Display,EnumString, Default)]
#[serde(rename_all = "camelCase")] 
#[strum(serialize_all = "camelCase")]
pub enum RsRequestStatus {
    /// No plugin yet processed this request
    #[default]
	Unprocessed,
    /// Link can be processed but first need to be added to the service and downloaded
    ///   -First call this plugin again with `add` method
    ///   -Check status and once ready call `process` again
    RequireAdd,
    /// Other plugin can process it
    Intermediate,
    /// Multiple files found, current plugin need to be recalled with a `selected_file``
    NeedFileSelection,
    /// `url` is ready but should be proxied by the server as it contains sensitive informations (like token)
    FinalPrivate,
    /// `url` is ready and can be directly sent to _any_ user directly (using redirect)
    FinalPublic
}



#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Default)]
#[serde(rename_all = "snake_case")] 
pub struct RsRequestFiles {
    pub name: String,
    pub size: u64,
    pub mime: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Default)]
#[serde(rename_all = "snake_case")] 
pub struct RsRequestWithCredential {
    pub request: RsRequest,
    pub credential: Option<PluginCredential>
}