#![allow(unused)]

mod app;
mod systemd;
use app::{Cmd, Cli};
use systemd::{
    check_services_or_err,
    run_service_scripts,
    check_service_file,
    check_exec_start,
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
	let mut msg = format!(
	    "\n|{:^30}|{:^15}|{:^15}|\n|{:=^30}|{:=^15}|{:=^15}|",
            "Service", "Unit File", "ExecStart",
            "","","",
	);
	services.iter().for_each(|name| {
	    let filename = format!("{}.service", name);
	    let mut service_exist = false;
	    let mut daemon_exist = false;
	    let no = "✕";
	    let ok = "✓";
	    if let Some(_) = check_service_file(filename.clone()) {
		service_exist = true;
	    }
	    if let Ok(_) = check_exec_start(filename.as_str()) {
		daemon_exist = true;
	    }
	    let row = format!(
		"\n|{:^30}|{:^15}|{:^15}|",
		filename,
		if service_exist { ok } else { no },
		if daemon_exist { ok } else { no },
	    );
	    msg += &row;
	});
	msg += "\n";
	term.write_line(&msg.as_str());
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

