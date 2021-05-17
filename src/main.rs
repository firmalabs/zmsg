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

        Cmd::Check => {
            let txs = rpc_client.z_listaddresses()?
                .iter()
                .flat_map(|addr| rpc_client.z_listreceivedbyaddress(&addr).unwrap())
                .filter(|tx| !tx.change)
                .collect::<Vec<_>>();
                
            // let target = addrs[2].clone();
            // check_msg(&rpc_client, &term, &target);

            let num_msg = txs.len();
            let mut heading = format!(
                "{:=<90}\n> Got {} messages.\n{:=<90}",
                "", num_msg, "",
            );
            term.write_line(&heading);

            txs.iter().enumerate().for_each(|(i, tx)| {
                let line = format!(
                    "{:<2}Message #{} (val = {})\n",
                    "|", i, tx.amount,
                );
                term.write_line(&line);
            });
        },
    }

    Ok(())
}

fn check_msg(c: &rpc::ZClient, t: &Term, addr: &str) -> Result<(), Error> {
    let txs = c.z_listreceivedbyaddress(addr)?;
    let num_msg = txs.len();
    let mut heading = format!(
        "\n{:=<90}\n> Got {} messages.\n{:=<90}\n",
        "", num_msg, "",
    );
    t.write_line(&heading);
    Ok(())
    /*
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
    */
}

fn send_msg_to(c: &rpc::ZClient, to: &str, msg: &str, amount: Option<f32>) -> Result<String, Error> {
    let my_addr = c.z_listaddresses()?[0].clone();
    let opid = c.z_sendmany(&my_addr, to, amount.unwrap_or(DEFAULT_AMOUNT), hex::str_to_hex(msg)?)?;
    Ok(opid)
}

