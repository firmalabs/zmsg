use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use anyhow::{anyhow, Context, Result};
use systemd_parser;

const DEFAULT_SYSTEMD_DIR: &str = "/etc/systemd/system/";

#[cfg(test)]
mod test {
    use super::*;
    use std::env::current_dir;
    
    #[test]
    fn test_file_exists() {
	let path = current_dir().unwrap().join("unit_files/cardano-node.service");
	let noop = current_dir().unwrap().join("noop/noop.sh");
	let path_str = path.to_str().unwrap();
	let noop_str = noop.to_str().unwrap();
	assert!(file_exists(path_str));
	assert!(!file_exists(noop_str));
    }
}

fn file_exists(path: &str) -> bool {
    Path::new(path).exists()
}

pub fn check_exec_start(filename: &str) -> Result<String> {
    let service_file = Path::new(DEFAULT_SYSTEMD_DIR).join(filename);
    let filepath = service_file.to_str().unwrap();
    if let Ok(u) = systemd_parser::parse(filepath) {
	if let Some(s) = u.get(&"Service".to_string()) {
	    if let Some(path) = s.get(&"ExecStart".to_string()) {
		match path {
		    systemd_parser::SystemdValue::Str(p) => {
			if file_exists(&p) {
			    Ok(p.to_string()) 
			} else {
			    Err(anyhow!("file does not exist"))
			}
		    },
		    _ => Err(anyhow!("multiple ExecStart key")),
		}
	    } else {
		return Err(anyhow!("fail to parse ExecStart in file {}", filename));
	    }
	} else {
	    return Err(anyhow!("fail to parse Service in file {}", filename));
	}
    } else {
	Err(anyhow!("fail to parse service file {}", filename))
    }
}

pub fn check_service_file(filename: String) -> Option<String> {
    let paths = fs::read_dir(DEFAULT_SYSTEMD_DIR)
	.with_context(|| format!("Failed to read {}", DEFAULT_SYSTEMD_DIR))
	.unwrap();
    for path in paths {
	if let Some(file_name) = path.unwrap().path().file_name() {
	    if file_name == PathBuf::from(filename.clone()) {
		return Some(file_name.to_str().unwrap().to_string());
	    }
	}
    }
    None
}

pub fn run_service_scripts(action: String, service_names: &[String]) -> Result<()> {
    let _ = check_services_or_err(service_names)
	.with_context(|| format!("Missing service file"))?;

    for service_name in service_names {
	let mut cmd: &mut Command = &mut Command::new("sudo");
	cmd = cmd.arg("systemctl");
	
	if action == "interrupt" {
	    cmd = cmd.args(&["kill", "-s", "SIGINT", &service_name.clone()]);
	} else {
	    cmd = cmd.args(&[action.clone(), service_name.clone()]);
	}
	
	let _ = cmd
	    .output()
	    .with_context(|| format!("Failed to {} the {} script!", action, service_name))?;
    }

    Ok(())
}

pub fn check_services_or_err(services: &[String]) -> Result<()> {
    let srv = services
	.iter()
	.map(|s| format!("{}.service", s));

    for s in srv {
	if let None = check_service_file(s.clone()) {
	    return Err(anyhow!("Service file for {} not found", s));
	}
    }
    Ok(())
}
