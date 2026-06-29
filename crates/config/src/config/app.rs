use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct AppInfo {
    pub name: String,
}
