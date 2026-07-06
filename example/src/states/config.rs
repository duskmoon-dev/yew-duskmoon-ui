use serde::{Deserialize, Serialize};
use yewdux::prelude::*;

fn default_theme_mode() -> String {
    "auto".to_string()
}

#[derive(PartialEq, Serialize, Deserialize, Store, Clone)]
#[store(storage = "session")]
pub struct ConfigStore {
    pub name: String,
    pub header_text: String,
    #[serde(default = "default_theme_mode")]
    pub theme_mode: String,
}

impl Default for ConfigStore {
    fn default() -> Self {
        Self {
            name: "yew app sample".to_string(),
            header_text: "capitalize".to_string(),
            theme_mode: default_theme_mode(),
        }
    }
}
