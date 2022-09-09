use async_trait::async_trait;
use once_cell::sync::Lazy;
use regex::Regex;
use std::process;

use crate::update_checker::get_app_command_path;
use crate::{update_checker::CheckError, UpdateChecker, UpdateStatus};

#[derive(Debug)]
pub struct ScoopUpdateChecker {
    app_name: String,
}

impl ScoopUpdateChecker {
    pub fn new(app_name: String) -> Self {
        Self { app_name }
    }
}

impl Default for ScoopUpdateChecker {
    fn default() -> Self {
        Self {
            app_name: "scoop".into(),
        }
    }
}

static RE_CHECK_UPDATE: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"^(?P<NAME>\S+)\s+(?P<CUR>\S+)\s+(?P<NEW>\S+).*$").unwrap());

#[async_trait]
impl UpdateChecker for ScoopUpdateChecker {
    async fn check_update(&self) -> Result<Vec<UpdateStatus>, CheckError> {
        let app = get_app_command_path(&self.app_name)?;

        let output = process::Command::new(app)
            .arg("status")
            .output()
            .map_err(CheckError::Io)?;

        let stdout = String::from_utf8(output.stdout).unwrap();
        Ok(stdout
            .lines()
            .skip_while(|line| !line.contains("----"))
            .skip(1)
            .filter_map(|line| {
                RE_CHECK_UPDATE.captures(line).map(|cap| {
                    UpdateStatus::new(
                        cap.name("NAME").unwrap().as_str().into(),
                        cap.name("CUR").unwrap().as_str().into(),
                        cap.name("NEW").unwrap().as_str().into(),
                    )
                })
            })
            .collect())
    }

    async fn update_all(&self) -> bool {
        todo!()
    }
    async fn update(&self) -> bool {
        todo!()
    }
}
