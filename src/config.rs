use std::{
    fs,
    path::{Path, PathBuf},
};

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};

pub const CONFIG_FILE: &str = "clyx.toml";

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ProjectConfig {
    pub name: String,
    #[serde(default = "default_language")]
    pub language: String,
    #[serde(default = "default_source")]
    pub source: String,
    #[serde(default = "default_build_dir")]
    pub build_dir: String,
    #[serde(default)]
    pub cmake_minimum: Option<String>,
    #[serde(default)]
    pub defaults: Defaults,
    #[serde(default)]
    pub build: BuildConfig,
    #[serde(default)]
    pub ui: UiConfig,
    #[serde(default)]
    pub compiler: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Defaults {
    #[serde(default = "default_standard")]
    pub standard: String,
    #[serde(default = "default_template")]
    pub template: String,
    #[serde(default)]
    pub tests: bool,
}
impl Default for Defaults {
    fn default() -> Self {
        Self {
            standard: default_standard(),
            template: default_template(),
            tests: false,
        }
    }
}
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct BuildConfig {
    #[serde(default = "default_generator")]
    pub generator: String,
    #[serde(default = "default_build_type")]
    pub build_type: String,
}
impl Default for BuildConfig {
    fn default() -> Self {
        Self {
            generator: default_generator(),
            build_type: default_build_type(),
        }
    }
}
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub struct UiConfig {
    #[serde(default)]
    pub color: bool,
}

fn default_language() -> String {
    "cpp".to_owned()
}
fn default_source() -> String {
    "src/main.cpp".to_owned()
}
fn default_build_dir() -> String {
    "build".to_owned()
}
fn default_standard() -> String {
    "c++17".to_owned()
}
fn default_template() -> String {
    "executable".to_owned()
}
fn default_generator() -> String {
    "default".to_owned()
}
fn default_build_type() -> String {
    "Debug".to_owned()
}

impl ProjectConfig {
    pub fn new(name: impl Into<String>, language: impl Into<String>) -> Self {
        let language = language.into();
        Self {
            name: name.into(),
            source: if language.eq_ignore_ascii_case("c") {
                "src/main.c".to_owned()
            } else {
                default_source()
            },
            language,
            build_dir: default_build_dir(),
            cmake_minimum: None,
            defaults: Defaults::default(),
            build: BuildConfig::default(),
            ui: UiConfig { color: true },
            compiler: None,
        }
    }

    pub fn load(path: &Path) -> Result<Self> {
        let file = path.join(CONFIG_FILE);
        let text =
            fs::read_to_string(&file).with_context(|| format!("reading {}", file.display()))?;
        toml::from_str(&text).with_context(|| format!("parsing {}", file.display()))
    }

    pub fn write(&self, path: &Path) -> Result<()> {
        let file = path.join(CONFIG_FILE);
        let text = toml::to_string_pretty(self).context("serializing project configuration")?;
        fs::write(&file, text).with_context(|| format!("writing {}", file.display()))
    }

    pub fn build_path(&self, root: &Path) -> PathBuf {
        root.join(&self.build_dir)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn config_round_trips() {
        let directory = tempdir().expect("temporary directory");
        let original = ProjectConfig::new("demo", "cpp");
        original.write(directory.path()).expect("write config");
        let loaded = ProjectConfig::load(directory.path()).expect("read config");
        assert_eq!(original, loaded);
    }
}
