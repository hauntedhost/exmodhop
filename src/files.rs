use std::fs::File;
use std::path::{Path, PathBuf};
use std::time::SystemTime;
use std::{fs, io};

use glob::glob;
use memmap2::Mmap;

enum FilterBy {
    None,
    ModifiedSince(SystemTime),
}

impl FilterBy {
    fn new(last_updated: Option<SystemTime>) -> Self {
        match last_updated {
            Some(time) => FilterBy::ModifiedSince(time),
            None => FilterBy::None,
        }
    }
}

// TODO: Return Result<Vec<PathBuf>, _>
pub fn get_paths(glob_pattern: String, modified_since: Option<SystemTime>) -> Option<Vec<PathBuf>> {
    let filter_by = FilterBy::new(modified_since);

    let paths: Vec<PathBuf> = glob(&glob_pattern)
        .expect("Failed to read glob pattern")
        .filter_map(|entry| {
            let path = entry.ok()?;
            return match filter_by {
                FilterBy::None => Some(path),
                FilterBy::ModifiedSince(modified_since) => {
                    let metadata = fs::metadata(&path).ok()?;
                    let last_modified = metadata.modified().ok()?;
                    if last_modified > modified_since {
                        Some(path)
                    } else {
                        None
                    }
                }
            };
        })
        .collect();

    Some(paths)
}

pub fn read_file_contents(path: &Path) -> io::Result<String> {
    let file = File::open(path)?;
    let mmap = unsafe { Mmap::map(&file)? };

    let contents = std::str::from_utf8(&mmap)
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?
        .to_string();

    Ok(contents)
}
