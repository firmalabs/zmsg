use structopt::StructOpt;

#[derive(StructOpt)]
#[structopt(rename_all = "snake_case")]
pub enum Cmd {
    /// Send a 512-byte encrypted memo to a target z_address
    /// with a default spare ZEC of 0.0001 ZEC
    Sendmsg {
        #[structopt(long)]
        /// a z_address of the recipient
        to: String,
        /// 512-byte max ASCII or Unicode message
        msg: String,
        /// Optional ZEC amount to be sent with the message
        #[structopt(long)]
        txval: Option<f32>,
    },
    /// Get my available shielded address(es)
    Zaddr {
        #[structopt(short, long)]
        /// a z_address of the recipient
        all: bool,
    },
    /// Check incoming messages
    Check,
}

impl ToString for Cmd {
    fn to_string(&self) -> String {
	match self {
            Self::Sendmsg{ .. } => String::from("sendmsg"),
            Self::Zaddr{ .. } => String::from("zaddr"),
            Self::Check => String::from("check"),
	}
    }
}

#[derive(StructOpt)]
#[structopt(
    name = "zmsg",
    author = "Pancy <pancy@firma.org>",
    about = "A zero knowledge messaging system built on zcash.",
    rename_all = "kebab-case",
)]
/// Tool for managing systemd services for Cardano node.
pub struct Cli {
    #[structopt(subcommand)]
    pub cmd: Cmd,
}
