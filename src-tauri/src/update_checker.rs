use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use thiserror::Error;
use which::which;

#[async_trait]
pub trait UpdateChecker {
    async fn check_update(&self) -> Result<Vec<UpdateStatus>, CheckError>;
    async fn update_all(&self) -> bool;
    async fn update(&self) -> bool;
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateStatus {
    name: String,
    current_version: String,
    new_version: String,
}

impl UpdateStatus {
    pub fn new(name: String, current_version: String, new_version: String) -> UpdateStatus {
        UpdateStatus {
            name,
            current_version,
            new_version,
        }
    }
}

#[derive(Debug, Error)]
pub enum CheckError {
    #[error("{app_name} could not be found")]
    AppNotFound { app_name: String },

    #[error(transparent)]
    Io(std::io::Error),
}

pub fn check_app_command(app_name: &str) -> Result<PathBuf, CheckError> {
    which(app_name).map_err(|_| CheckError::AppNotFound {
        app_name: app_name.to_string(),
    })
}
