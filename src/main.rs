#![allow(unused)]

mod app;
mod hex;
mod auth;
mod rpc;

use app::{Cmd, Cli};
use std::{
    fs,
    path::PathBuf,
    string::ToString,
    process::Command,
};

use structopt::StructOpt;
use console::{Term, Style};
use anyhow::{anyhow, Context, Result, Error};

const DEFAULT_AMOUNT: f32 = 0.0001;

fn main() -> Result<(), Error> {
    let mut cli = Cli::from_args();
    let term = Term::stdout();
    let Cli{ cmd } = cli;

    let (u, p) = auth::read_auth_creds(None)?;
    let rpc_client = rpc::ZClient::builder()
        .with_url("http://127.0.0.1:9999".to_owned())?
        .with_auth(u, Some(p))
        .build();
        
    match cmd {
        Cmd::Sendmsg{ to, msg, .. } => {
            let opid = send_msg_to(&rpc_client, &to, &msg, None)?;
            let notify = format!("Message sent to {} with opid = {}", to, opid);
	    term.write_line(&notify);
        },

        Cmd::Zaddr{ all } => {
            if all {
                let addrs = rpc_client.z_listaddresses()?;
                for addr in addrs {
                    term.write_line(&addr);
                }
            } else {
                let addr = rpc_client.z_listaddresses()?[0].clone();
                term.write_line(&addr);
            }
        },

    }

    Ok(())
}

fn send_msg_to(c: &rpc::ZClient, to: &str, msg: &str, amount: Option<f32>) -> Result<String, Error> {
    let my_addr = c.z_listaddresses()?[0].clone();
    let opid = c.z_sendmany(&my_addr, to, amount.unwrap_or(DEFAULT_AMOUNT), hex::str_to_hex(msg)?)?;
    Ok(opid)
}

