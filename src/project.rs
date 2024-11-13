use std::path::PathBuf;
use std::time::SystemTime;
use std::{env, fs};

use chrono::{DateTime, Utc};
use sha2::{Digest, Sha256};

use crate::files::get_paths;

#[derive(Debug)]
pub struct Project {
    pub root_path: PathBuf,
    pub config_path: PathBuf,
    pub index_path: PathBuf,
    pub modified_since: Option<SystemTime>,
}

impl Project {
    // TODO: Return Result<Self, _>
    pub fn new(project_root_path: PathBuf) -> Self {
        let config_root_path = dirs_next::home_dir()
            .expect("Failed to get home path")
            .join(".config/exmodhop");

        let config_path = project_config_path(&project_root_path, &config_root_path)
            .expect("Failed to get config path");

        fs::create_dir_all(&config_path).expect("Failed to create config path");

        let index_path = config_path.join("modules.index");

        let modified_since = match env::var("MODIFIED_SINCE") {
            Ok(input) => parse_datetime(input),
            Err(_) => last_modified(&index_path),
        };

        Project {
            root_path: project_root_path,
            config_path,
            index_path,
            modified_since,
        }
    }

    // TODO: move this to a separate module
    pub fn get_elixir_source_paths(&self) -> Option<Vec<PathBuf>> {
        let project_source_path = self.root_path.join("lib");
        let glob_pattern = format!("{}/**/*.ex", project_source_path.to_string_lossy());
        get_paths(glob_pattern, self.modified_since)
    }
}

fn hash_abspath(path: &PathBuf) -> Option<String> {
    let path_name = path.to_str()?;
    let mut hasher = Sha256::new();
    hasher.update(path_name);
    Some(format!("{:x}", hasher.finalize()))
}

fn last_modified(path: &PathBuf) -> Option<SystemTime> {
    fs::metadata(path).ok()?.modified().ok()
}

fn parse_datetime(input: String) -> Option<SystemTime> {
    let datetime = input.parse::<DateTime<Utc>>().ok()?;
    Some(datetime.into())
}

fn project_config_path(project_root_path: &PathBuf, config_root_path: &PathBuf) -> Option<PathBuf> {
    // Dirname of project path, e.g. "hello_world"
    let project_dirname = project_root_path.file_name()?.to_str()?;

    // Hash of the full project path
    let project_path_hash = hash_abspath(&project_root_path)?;

    // Pathname for this project config
    let project_config_pathname = format!("{project_dirname}-{project_path_hash}");

    Some(config_root_path.join(project_config_pathname))
}
