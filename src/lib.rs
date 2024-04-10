use std::str::FromStr;

use regex::Regex;
pub use rs_plugin_common_interfaces::PluginCredential;
use rs_plugin_common_interfaces::{RsAudio, RsResolution, RsVideoCodec};
use serde::{Deserialize, Serialize};
use strum_macros::EnumString;

pub mod error;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Default)]
#[serde(rename_all = "camelCase")] 
pub struct RsCookie {
    pub domain: String,
    pub http_only: bool,
    pub path: String,
    pub secure: bool,
    pub expiration: Option<f64>,
    pub name: String,
    pub value: String,
}

impl FromStr for RsCookie {
    type Err = error::RequestError;
    fn from_str(line: &str) -> Result<Self, Self::Err> {
        //let [domain, httpOnly, path, secure, expiration, name, value ] = line.split(';');
        let mut splitted = line.split(';');
        Ok(RsCookie { 
            domain: splitted.next().ok_or(error::RequestError::UnableToParseCookieString("domain".to_owned(), line.to_owned()))?.to_owned(), 
            http_only: "true" == splitted.next().ok_or(error::RequestError::UnableToParseCookieString("http_only".to_owned(), line.to_owned()))?.to_owned(), 
            path: splitted.next().ok_or(error::RequestError::UnableToParseCookieString("path".to_owned(), line.to_owned()))?.to_owned(), 
            secure: "true" == splitted.next().ok_or(error::RequestError::UnableToParseCookieString("secure".to_owned(), line.to_owned()))?.to_owned(), 
            expiration: {
                let t = splitted.next().ok_or(error::RequestError::UnableToParseCookieString("expiration".to_owned(), line.to_owned()))?.to_owned();
                if t == "" {
                    None  
                } else {
                    Some(t.parse().map_err(|_| error::RequestError::UnableToParseCookieString("expiration parsing".to_owned(), line.to_owned()))?)
                }
            }, 
            name: splitted.next().ok_or(error::RequestError::UnableToParseCookieString("name".to_owned(), line.to_owned()))?.to_owned(), 
            value: splitted.next().ok_or(error::RequestError::UnableToParseCookieString("value".to_owned(), line.to_owned()))?.to_owned() })
    }
}

impl  RsCookie {
    pub fn netscape(&self) -> String {
        let second = if self.domain.starts_with(".") {
            "TRUE"
        } else {
            "FALSE"
        };
        let secure = if self.secure {
            "TRUE"
        } else {
            "FALSE"
        };
        let expiration = if let Some(expiration) = self.expiration {
           (expiration as u32).to_string()
        } else {
            "".to_owned()
        };
        //return [domain, domain.startsWith('.') ? 'TRUE' : 'FALSE', path, secure ? 'TRUE' : 'FALSE', expiration.split('.')[0], name, value].join('\t')
        format!("{}\t{}\t{}\t{}\t{}\t{}\t{}", self.domain, second, self.path, secure, expiration, self.name, self.value)
    }

    pub fn header(&self) -> String {
        format!("{}={}", self.name, self.value)
    }
}

pub trait RsCookies {
    fn header_value(&self) -> String;
    fn headers(&self) -> (String, String);
}

impl RsCookies for Vec<RsCookie> {
    fn header_value(&self) -> String {
        self.iter().map(|t| t.header()).collect::<Vec<String>>().join("; ")
    }

    fn headers(&self) -> (String, String) {
        ("cookie".to_owned(), self.iter().map(|t| t.header()).collect::<Vec<String>>().join("; "))
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Default)]
#[serde(rename_all = "camelCase")] 
pub struct RsRequest {
    pub upload_id: Option<String>,
    pub url: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mime: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filename: Option<String>,
    #[serde(default)]
    pub status: RsRequestStatus,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub referer: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub headers: Option<Vec<(String, String)>>,
    /// some downloader like YTDL require detailed cookies. You can create Header equivalent  with `headers` fn on the vector
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cookies: Option<Vec<RsCookie>>,
    /// If must choose between multiple files. Recall plugin with a `selected_file` containing one of the name in this list to get link
    #[serde(skip_serializing_if = "Option::is_none")]
    pub files: Option<Vec<RsRequestFiles>>,
    /// one of the `files` selected for download
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selected_file: Option<String>,

    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub people: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub albums: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub season: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub episode: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resolution: Option<RsResolution>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub videocodec: Option<RsVideoCodec>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audio: Option<Vec<RsAudio>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quality: Option<u64>,
}

impl RsRequest {
    pub fn set_cookies(&mut self, cookies: Vec<RsCookie>) {
        let mut existing = if let Some(headers) = &self.headers {
            headers.to_owned()
        } else{
            vec![]
        };
        existing.push(cookies.headers());
        self.headers = Some(existing);
    }

    pub fn parse_filename(&mut self) {
        if let Some(filename) = &self.filename {
            let resolution = RsResolution::from_filename(filename);
            if resolution != RsResolution::Unknown {
                self.resolution = Some(resolution);
            }
            let videocodec = RsVideoCodec::from_filename(filename);
            if videocodec != RsVideoCodec::Unknown {
                self.videocodec = Some(videocodec);
            }
            let audio = RsAudio::list_from_filename(filename);
            if audio.len() > 0 {
                self.audio = Some(audio);
            }

            let re = Regex::new(r"(?i)s(\d+)e(\d+)").unwrap();
            match re.captures(filename) {
                Some(caps) => {
                    self.season = caps[1].parse::<u32>().ok();
                    self.episode = caps[2].parse::<u32>().ok();
                }
                None => (),
            }
        }
 
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, strum_macros::Display,EnumString, Default)]
#[serde(rename_all = "camelCase")] 
#[strum(serialize_all = "camelCase")]
pub enum RsRequestStatus {
    /// No plugin yet processed this request
    #[default]
	Unprocessed,
    ///if remain in this state after all plugin it will go through YtDl to try to extract medias
    NeedParsing,
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
#[serde(rename_all = "camelCase")] 
pub struct RsRequestFiles {
    pub name: String,
    pub size: u64,
    pub mime: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Default)]
#[serde(rename_all = "camelCase")] 
pub struct RsRequestPluginRequest {
    pub request: RsRequest,
    pub credential: Option<PluginCredential>,
    ///Plugin should only send back request that are usable long term and can be saved to the DB for later use
    #[serde(default)]
    pub savable: bool
}



#[cfg(test)]
mod tests {

    use self::error::RequestError;

    use super::*;

    #[test]
    fn test_cookie_parsing() -> Result<(), RequestError> {
        let parsed = RsCookie::from_str(".twitter.com;false;/;true;1722364794.437907;kdt;w1j")?;
        assert!(parsed.domain == ".twitter.com".to_owned());
        assert!(parsed.http_only == false);
        assert!(parsed.path == "/".to_owned());
        assert!(parsed.secure == true);
        assert!(parsed.expiration == Some(1722364794.437907));
        assert!(parsed.name == "kdt".to_owned());
        assert!(parsed.value == "w1j".to_owned());
        Ok(())
    }
    
    #[test]
    fn test_cookie_parsing_no_expi() -> Result<(), RequestError> {
        let parsed = RsCookie::from_str(".twitter.com;false;/;true;;kdt;w1j")?;
        assert!(parsed.domain == ".twitter.com".to_owned());
        assert!(parsed.http_only == false);
        assert!(parsed.path == "/".to_owned());
        assert!(parsed.secure == true);
        assert!(parsed.expiration == None);
        assert!(parsed.name == "kdt".to_owned());
        assert!(parsed.value == "w1j".to_owned());
        Ok(())
    }

    #[test]
    fn test_netscape() -> Result<(), RequestError> {
        let parsed = RsCookie::from_str(".twitter.com;false;/;true;1722364794.437907;kdt;w1j")?;
        assert!(parsed.netscape() == ".twitter.com\tTRUE\t/\tTRUE\t1722364794\tkdt\tw1j");
        Ok(())
    }
    #[test]
    fn test_netscape_doublequote() -> Result<(), RequestError> {
        let parsed = RsCookie::from_str(".twitter.com;true;/;true;1726506480.700665;ads_prefs;\"HBESAAA=\"")?;
        assert!(parsed.netscape() == ".twitter.com\tTRUE\t/\tTRUE\t1726506480\tads_prefs\t\"HBESAAA=\"");
        Ok(())
    }

    #[test]
    fn test_header() -> Result<(), RequestError> {
        let parsed = vec![RsCookie::from_str(".twitter.com;true;/;true;1726506480.700665;ads_prefs;\"HBESAAA=\"")?, RsCookie::from_str(".twitter.com;false;/;true;1722364794.437907;kdt;w1j")?];
        println!("header: {}", parsed.header_value());
        assert!(parsed.header_value() == "ads_prefs=\"HBESAAA=\"; kdt=w1j");
        Ok(())
    }

    #[test]
    fn test_parse() -> Result<(), RequestError> {
        let mut req = RsRequest { filename: Some("Shogun.2024.S01E01.Anjin.1080p.VOSTFR.DSNP.WEB-DL.DDP5.1.H.264-NTb".to_owned()), ..Default::default()};
        req.parse_filename();
        assert_eq!(req.season.unwrap(), 1);
        assert_eq!(req.episode.unwrap(), 1);
        assert_eq!(req.resolution.unwrap(), RsResolution::FullHD);
        assert_eq!(req.videocodec.unwrap(), RsVideoCodec::H264);
        assert_eq!(req.audio.unwrap().len(), 1);
        Ok(())
    }
}