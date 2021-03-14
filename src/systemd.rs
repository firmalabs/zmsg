use std::{fs, path::PathBuf};
use anyhow::{anyhow, Context, Result};

pub fn check_daemon_path(path: PathBuf) -> Result<String> {
    let mut file_name = "";
    if let Some(f) = path.file_name().unwrap().to_str() {
	file_name = f;
    }

    if let Some(dir_path) = path.parent() {
	if let Some(path) = dir_path.to_str() {
	    let paths = fs::read_dir(path)
		.with_context(|| {
		    format!("failed to read {}", path)
		})?;
	    for path in paths {
		if let Some(fname) = path.unwrap().path().file_name() {
		    if fname == file_name {
			if let Ok(name) = fname.to_os_string().into_string() {
			    return Ok(name);
			}
		    }
		}
	    }
	}
    }
    Err(anyhow!("cannot file the daemon file"))
}
