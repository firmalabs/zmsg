use std::fs;
use std::path::PathBuf;
use std::process::Command;
use anyhow::{anyhow, Context, Result};
use systemd_parser;

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

pub fn check_exec_start(filename: &str) -> Result<String> {
    if let Ok(u) = systemd_parser::parse(filename) {
	if let Some(s) = u.get(&"Service".to_string()) {
	    if let Some(path) = s.get(&"ExecStart".to_string()) {
		match path {
		    systemd_parser::SystemdValue::Str(p) => {
			check_daemon_path(PathBuf::from(&p))
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
    let service_dir = "/etc/systemd/system/";
    let paths = fs::read_dir(service_dir)
	.with_context(|| format!("Failed to read {}", service_dir))
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
