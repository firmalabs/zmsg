#![allow(unused)]

mod app;
mod systemd;
use app::{Cmd, Cli};
use systemd::{
    check_services_or_err,
    run_service_scripts,
    check_service_file,
};
use std::{
    fs,
    path::PathBuf,
    string::ToString,
    process::Command,
};

use structopt::StructOpt;
use console::{Term, Style};
use anyhow::{anyhow, Context, Result};
use systemd_parser;

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

