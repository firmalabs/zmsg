#![allow(unused)]

use std::string::ToString;
use std::process::Command;

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
    #[structopt(short, long)]
    /// Specify all services
    all: bool,

    #[structopt(short, long, default_value = "all")]
    /// Specific a service 
    service: String,

    #[structopt(subcommand)]
    cmd: Cmd,
}

fn run_service_scripts(action: String, service_names: &[String]) -> Result<()> {
    service_names.par_iter()
	.for_each(|name| {
	    Command::new("systemctl")
		.args(&[action.clone(), format!("{}.service", name)])
		.output()
		.with_context(|| format!("Failed to {} the {} script!", action, name));
	});
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
	    term.write_line(&name);
	});
	return Ok(());
    }
    
    if all || service == String::from("all") {
	run_service_scripts(action, services);
	return Ok(());
    }
    
    match (action, service) {
	(action, name) => {
	    run_service_scripts(action, &[name]);
	},
	(action, _) => {
	    run_service_scripts(action, services);
	},
    }

    Ok(())
}

