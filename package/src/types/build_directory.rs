use crate::{errors, types::Result};
use std::path::PathBuf;
use tempfile::TempDir;

pub struct BuildDirectory {
    pub path: PathBuf,
    temp_dir: Option<TempDir>,
}

impl BuildDirectory {
    pub fn new(path: Option<PathBuf>) -> Result<Self> {
        Ok(match path {
            Some(path) => Self {
                path,
                temp_dir: None,
            },
            None => {
                let temp_dir = Some(TempDir::new().map_err(errors::io_error)?);

                Self {
                    path: temp_dir.as_ref().unwrap().path().to_owned().to_path_buf(),
                    temp_dir,
                }
            }
        })
    }
}
