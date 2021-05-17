#![allow(unused)]

mod app;
mod hex;
mod auth;
mod rpc;

use app::{Cmd, Cli};
use std::{
    fs,
    time,
    path::PathBuf,
    string::ToString,
    process::Command,
};

use structopt::StructOpt;
use console::{Term, Style};
use anyhow::{anyhow, Context, Result, Error};
use chrono::{DateTime, NaiveDateTime, TimeZone, Utc, Local};

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

            let num_msg = &txs.len();
            let mut heading = format!(
                "{:=<90}\n> Got {} messages.\n{:=<90}",
                "", num_msg, "",
            );
            term.write_line(&heading);
            
            txs.iter().enumerate().for_each(|(i, tx)| {
                let rpc::Tx{ txid, amount, memo, .. } = tx;


                let wtx: rpc::WalletTx = rpc_client.gettransaction(txid).unwrap();
                let dt: DateTime<Local> = Local.from_utc_datetime(
                    &NaiveDateTime::from_timestamp((wtx.time as u32).into(), 0)
                );

                // let formatted_dt = dt.to_rfc3339();
                let format_str = format!("%a %b %e{} %Y {} %T", ",", "at");
                let formatted_dt = dt.format(&format_str);

                let line1 = format!(
                    "{:<2}Message #{} (val = {})\n",
                    "|", i, amount,
                );
                let line2 = &format!("{:<2}To:\n", "|");
                let line3 = &format!("{:<2}Date: {}\n", "|", formatted_dt);
                let line4 = &format!("{:<2}\n", "|");
                let line5 = &format!("{:<4}{}\n", "|", hex::hex_to_string(&memo).unwrap_or("".to_string()));
                let end = &format!("{:=<90}", "");
                let block = line1 + line2 + line3 + line4 + line5 + end;
                term.write_line(&block);
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
}

fn send_msg_to(c: &rpc::ZClient, to: &str, msg: &str, amount: Option<f32>) -> Result<String, Error> {
    let my_addr = c.z_listaddresses()?[0].clone();
    let opid = c.z_sendmany(&my_addr, to, amount.unwrap_or(DEFAULT_AMOUNT), hex::str_to_hex(msg)?)?;
    Ok(opid)
}

