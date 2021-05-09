#![allow(unused)]

mod app;
mod hex;

use app::{Cmd, Cli};
use std::{
    fs,
    path::PathBuf,
    string::ToString,
    process::Command,
};

use structopt::StructOpt;
use console::{Term, Style};
use anyhow::{anyhow, Context, Result};

fn main() -> Result<()> {
    let mut cli = Cli::from_args();
    let term = Term::stdout();
    
    // let Cli{ cmd } = cli;
    // let action = cmd.to_string();

    let Cli{ cmd } = cli;
    match cmd {
        Cmd::Sendmsg{ to } => {
            let msg = format!("Sending msg to {}", to);
	    term.write_line(&msg.as_str());
        }
    }

    Ok(())
}

