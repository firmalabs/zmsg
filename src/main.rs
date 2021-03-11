#![allow(unused)]

use std::{
    fs,
    path::PathBuf,
    string::ToString,
    process::Command,
};

use structopt::StructOpt;
use rayon::prelude::*;
use console::Term;
use anyhow::{
    anyhow,
    Context, Result,
};


#[derive(StructOpt)]
#[structopt(rename_all = "kebab-case")]
enum Cmd {
    /// Call `sudo systemctl start <service>`.
    Start,
    /// Call `sudo systemctl stop <service>`.
    Stop,
    /// Call `sudo systemctl status <service>`.
    Status,
    /// Call `sudo systemctl kill -s SIGINT <service>`
    Interrupt,
    /// List available systemd unit files in `/etc/systemd/system`.
    List,
}

impl ToString for Cmd {
    fn to_string(&self) -> String {
	match self {
	    Self::Start => String::from("start"),
	    Self::Stop => String::from("stop"),
	    Self::Status => String::from("status"),
	    Self::List => String::from("list"),
	    Self::Interrupt => String::from("interrupt"),
	}
    }
}

#[derive(StructOpt)]
#[structopt(
    name = "cardano-systemd",
    author = "Pancy <pan@xerberus.net>",
    rename_all = "kebab-case",
)]
/// Tool for managing systemd services for Cardano node.
struct Cli {
    #[structopt(short, long, global = true)]
    /// Specify all services
    all: bool,

    #[structopt(short, long, default_value = "all", global = true)]
    /// Specific a service 
    service: String,

    #[structopt(subcommand)]
    cmd: Cmd,
}

fn check_service_file(filename: String) -> Option<String> {
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

fn run_service_scripts(action: String, service_names: &[String]) -> Result<()> {
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

fn check_services_or_err(services: &[String]) -> Result<()> {
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

fn main() -> Result<()> {
    let mut cli = Cli::from_args();
    let term = Term::stdout();
    
    let Cli{ all, service, cmd } = cli;
    
    let services: &[String] = &[
	"node-exporter".to_string(),
	"prometheus".to_string(),
	"cardano-node".to_string(),
    ];
    let action = cmd.to_string();

    if action == "list" {
	services.iter().for_each(|name| {
	    let filename = format!("{}.service", name);
	    let mut exist = false;
	    let no = "✕";
	    let ok = "✓";
	    if let Some(_) = check_service_file(filename.clone()) {
		exist = true;
	    }
	    let msg = format!(
		"{} ... {}",
		filename,
		if exist { ok } else { no });
	    term.write_line(&msg.as_str());
	});
	return Ok(());
    }
    
    if all || service == String::from("all") {
	if let Err(err) = run_service_scripts(action, services) {
	    term.write_line("Service file missing");
	    return Err(err);
	}
	return Ok(());
    }
    
    match (action, service) {
	(action, name) => {
	    if let Err(err) = run_service_scripts(action, &[name]) {
		term.write_line("Service file missing");
		return Err(err);
	    }
	},
	(action, _) => {
	    if let Err(err) = run_service_scripts(action, services) {
		term.write_line("Service file missing");
		return Err(err);
	    }
	},
    }

    Ok(())
}

