use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct ConfigFile {
    pub path: String,
    pub name: String,
    #[serde(default)]
    pub description: String,
    #[serde(default)]
    pub readonly: bool,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ConfigDirectory {
    pub path: String,
    pub name: String,
    #[serde(default = "default_depth")]
    pub depth: usize,
    #[serde(default)]
    pub types: Vec<String>,
    #[serde(default)]
    pub description: String,
    #[serde(default)]
    pub readonly: bool,
}

fn default_depth() -> usize {
    3
}

#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    #[serde(default)]
    pub files: Vec<ConfigFile>,
    #[serde(default)]
    pub directories: Vec<ConfigDirectory>,
}
