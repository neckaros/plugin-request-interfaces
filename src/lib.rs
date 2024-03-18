use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Default)]
#[serde(rename_all = "snake_case")] 
pub struct RsRequest {
    pub url: String,
    pub require_add: bool,
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