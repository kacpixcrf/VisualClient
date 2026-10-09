use std::collections::HashMap;

#[derive(serde::Deserialize)]
pub struct VersionManifest {
    pub versions: Vec<VersionEntry>,
}

#[derive(serde::Deserialize)]
pub struct VersionEntry {
    pub id: String,
    pub url: String,
}

#[derive(serde::Deserialize)]
pub struct VersionJson {
    pub downloads: Downloads,
    pub libraries: Vec<Library>,
    #[serde(rename = "assetIndex")]
    pub asset_index: AssetIndex,
}

#[derive(serde::Deserialize)]
pub struct Downloads {
    pub client: DownloadArtifact,
}

#[derive(serde::Deserialize)]
pub struct DownloadArtifact {
    pub url: String,
}

#[derive(serde::Deserialize)]
pub struct Library {
    pub downloads: LibraryDownloads,
    pub rules: Option<Vec<Rule>>,
}

#[derive(serde::Deserialize)]
pub struct LibraryDownloads {
    pub artifact: Option<Artifact>,
    pub classifiers: Option<HashMap<String, Artifact>>,
}

#[derive(serde::Deserialize)]
pub struct Artifact {
    pub path: String,
    pub url: String,
}

#[derive(serde::Deserialize)]
pub struct Rule {
    pub action: String,
    pub os: Option<OsRule>,
}

#[derive(serde::Deserialize)]
pub struct OsRule {
    pub name: String,
}

#[derive(serde::Deserialize)]
pub struct AssetIndex {
    pub id: String,
    pub url: String,
}

#[derive(serde::Deserialize)]
pub struct AssetIndexJson {
    pub objects: HashMap<String, AssetObject>,
}

#[derive(serde::Deserialize)]
pub struct AssetObject {
    pub hash: String,
}

pub fn is_library_allowed(rules: &Option<Vec<Rule>>, os_name: &str) -> bool {
    let Some(rules) = rules else { return true; };
    let mut allowed = false;
    for rule in rules {
        if rule.action == "allow" {
            if let Some(os) = &rule.os {
                if os.name == os_name { allowed = true; }
            } else { allowed = true; }
        } else if rule.action == "disallow" {
            if let Some(os) = &rule.os {
                if os.name == os_name { allowed = false; }
            } else { allowed = false; }
        }
    }
    allowed
}
