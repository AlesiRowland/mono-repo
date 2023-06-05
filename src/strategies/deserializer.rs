use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::hash::Hash;
use toml::Value;

#[derive(Deserialize, Serialize, Debug, PartialEq)]
// #[serde(deny_unknown_fields)]
pub struct Pyproject {
    pub tool: Tool,
}

#[derive(Deserialize, Serialize, Debug, PartialEq)]
// #[serde(deny_unknown_fields)]
pub struct Tool {
    pub poetry: Poetry,
    #[serde(flatten)]
    other: HashMap<String, Value>,
}

#[derive(Deserialize, Serialize, Debug, PartialEq)]
// #[serde(deny_unknown_fields)]
pub struct Source {
    pub name: String,
    pub url: String,
}

/// Specced from https://python-poetry.org/docs/pyproject/
#[derive(Deserialize, Serialize, Debug, PartialEq)]
#[serde(rename_all = "kebab-case")]
pub struct Poetry {
    pub name: String,
    pub version: String,
    pub description: String,
    pub license: Option<String>,
    pub authors: Vec<String>,
    pub maintainers: Option<String>,
    pub readme: Option<String>,
    pub homepage: Option<String>,
    pub repository: Option<String>,
    pub documentation: Option<String>,
    pub keywords: Option<Vec<String>>,
    pub classifiers: Vec<String>,
    pub packages: Option<HashMap<String, String>>,
    pub include: Option<HashMap<String, String>>,
    pub exclude: Option<HashMap<String, String>>,
    pub dependencies: HashMap<String, Value>,
    pub dev_dependencies: Option<HashMap<String, Value>>, // This is to support old style.
    pub group: Option<Value>,
    pub scripts: Option<HashMap<String, Value>>,
    pub extras: Option<HashMap<String, Vec<String>>>,
    pub plugins: Option<HashMap<String, String>>,
    pub urls: Option<HashMap<String, String>>,
    pub source: Option<Vec<Source>>,
}
