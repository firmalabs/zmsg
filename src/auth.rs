use std::{str, env};
use std::io::{self, BufRead, BufReader};
use std::ffi::OsString;
use std::fs::File;
use std::path::Path;
use anyhow::{anyhow, Error};

/// Read the zcash.conf file to get RPC user and password.
pub fn read_auth_creds(path: Option<OsString>) -> Result<(String, String), Error> {
    let mut config_path = OsString::new();
    if path.is_none() {
        config_path = match env::var_os("HOME") {
            Some(path) => path,
            None => {
                return Err(anyhow!("Failed to fetch $HOME. Did you set it?"));
            },
        };
        config_path.push("/.zcash/zcash.conf");
    } else {
        config_path = path.unwrap();
    }

    let mut rpcuser = String::new();
    let mut rpcpass = String::new();

    let file = File::open(config_path)?;
    let reader = BufReader::new(file);

    // Read the file line by line using the lines() iterator from std::io::BufRead.
    for (_, line) in reader.lines().enumerate() {
        if let Ok(li) = line {
            let i = li.split("=").collect::<Vec<&str>>();
            if let [key, val] = i.as_slice() {
                match *key {
                    "rpcuser" => {
                        rpcuser = val.to_string();
                    },
                    "rpcpassword" => {
                        rpcpass = val.to_string();
                    },
                    _ => {},
                }
            }
        }
    }

    Ok((rpcuser, rpcpass))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_auth_creds() {
        if let Ok((u, p)) = read_auth_creds(Some(OsString::from("./test_files/zcash.conf"))) {
            assert!(u == "user");
            assert!(p == "pass");
        } else {
            assert!(false);
        }
    }
}

