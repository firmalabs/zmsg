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
    Start,
    Stop,
    Status,
    List,
}

impl ToString for Cmd {
    fn to_string(&self) -> String {
	match self {
	    Self::Start => String::from("start"),
	    Self::Stop => String::from("stop"),
	    Self::Status => String::from("status"),
	    Self::List => String::from("list"),
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
	let name = path.unwrap().path();
	if name == PathBuf::from(filename.clone()) {
	    return Some(name.to_str().unwrap().to_string());
	}
    }
    None
}

fn run_service_scripts(action: String, service_names: &[String]) -> Result<()> {
    let _ = check_services_or_err(service_names)
	.with_context(|| format!("One or more service files are missing!"))?;
    service_names.par_iter()
	.for_each(|name| {
	    Command::new("systemctl")
		.args(&[action.clone(), format!("{}.service", name)])
		.output()
		.with_context(|| format!("Failed to {} the {} script!", action, name));
	});
    Ok(())
}

fn check_services_or_err(services: &[String]) -> Result<()> {
    for service in services {
	if let None = check_service_file(service.to_string()) {
	    return Err(anyhow!("Service file for {} not found", service));
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

