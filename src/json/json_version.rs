use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, HashMap};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct JsonVersion {
    #[serde(rename = "id")]
    pub id: String,
    #[serde(with = "serde_date_format")]
    pub time: DateTime<Utc>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub patches: Option<Vec<JsonVersion>>,
    #[serde(rename = "releaseTime", with = "serde_date_format")]
    pub release_time: DateTime<Utc>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "type")]
    pub type_: Option<String>,
    #[serde(rename = "minecraftArguments")]
    pub arguments: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "minimumLauncherVersion")]
    pub minimum_launcher_version: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub libraries: Option<Vec<JsonLibrary>>,
    #[serde(rename = "assetIndex")]
    pub asset_index: Option<JsonAssetsIndex>,
    #[serde(rename = "inheritsFrom")]
    pub override_: Option<String>,
    pub jar: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub assets: Option<String>,
    #[serde(rename = "mainClass")]
    pub main_class: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub downloads: Option<HashMap<String, JsonDownload>>,
}


impl JsonVersion {
    pub fn set_to_json(&self, json: &mut JsonVersion) {
        json.id = self.id.clone();
        json.time = self.time;
        json.release_time = self.release_time;
        json.type_ = self.type_.clone().or_else(|| self.patches.as_ref().and_then(|p| p.get(0)).and_then(|p| p.type_.clone()));
        json.arguments = self.arguments.clone();
        json.minimum_launcher_version = self.minimum_launcher_version.or_else(|| self.patches.as_ref().and_then(|p| p.get(0)).and_then(|p| p.minimum_launcher_version));
        json.asset_index = self.asset_index.clone().or_else(|| self.patches.as_ref().and_then(|p| p.get(0)).and_then(|p| p.asset_index.clone()));
        json.override_ = self.override_.clone();
        json.jar = self.jar.clone();
        json.assets = self.assets.clone().or_else(|| self.patches.as_ref().and_then(|p| p.get(0)).and_then(|p| p.assets.clone()));
        json.main_class = self.main_class.clone();
        if let Some(libraries) = &self.libraries {
            if let Some(json_libraries) = &mut json.libraries {
                json_libraries.extend(libraries.iter().cloned());
            } else {
                json.libraries = Some(libraries.clone());
            }
        }
        if let Some(downloads) = &self.downloads {
            if let Some(json_downloads) = &mut json.downloads {
                for (key, value) in downloads {
                    if let Some(json_value) = json_downloads.get_mut(key) {
                        *json_value = value.clone();
                    } else {
                        json_downloads.insert(key.clone(), value.clone());
                    }
                }
            } else {
                json.downloads = Some(downloads.clone());
            }
        }
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct JsonAssetsIndex {
    pub id: String,
    pub sha1: String,
    #[serde(default = "zero_default")]
    pub size: i32,
    #[serde(default = "zero_default")]
    pub totalSize: i32,
    pub url: String,
    #[serde(default)]
    pub known: bool,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct JsonLibrary {
    pub name: String,
    #[serde(default)]
    pub url: Option<String>,
    #[serde(default)]
    pub natives: Option<HashMap<String, String>>,
    #[serde(default)]
    pub rules: Option<Vec<JsonRule>>,
    #[serde(default)]
    pub extract: Option<JsonExtract>,
    #[serde(default)]
    pub checksums: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub downloads: Option<JsonDownloads>,
    #[serde(default)]
    #[serde(rename = "clientreq")]
    pub is_client_requirement: bool,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct JsonRule {
    pub action: String,
    #[serde(default)]
    pub os: Option<JsonOperatingSystem>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct JsonOperatingSystem {
    pub name: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct JsonExtract {
    pub exclude: Vec<String>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct JsonDownloads {
    pub artifact: JsonDownload,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub classifiers: Option<HashMap<String, JsonDownload>>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct JsonDownload {
    pub url: Option<String>,
    pub sha1: Option<String>,
    pub size: Option<i32>,
    pub path: Option<String>,
}

mod serde_date_format {
    use chrono::{DateTime, Utc, TimeZone};
    use serde::{self, Deserialize, Deserializer, Serializer};

    const FORMAT: &str = "%Y-%m-%dT%H:%M:%S%z";

    pub fn serialize<S>(date: &DateTime<Utc>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let s = date.format(FORMAT).to_string();
        serializer.serialize_str(&s)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<DateTime<Utc>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Utc.datetime_from_str(&s, FORMAT).map_err(serde::de::Error::custom)
    }
}

fn zero_default() -> i32 {
    0
}
