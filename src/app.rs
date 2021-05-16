use structopt::StructOpt;

#[derive(StructOpt)]
#[structopt(rename_all = "snake_case")]
pub enum Cmd {
    /// Send a 512-byte encrypted memo to a target z_address
    Sendmsg {
        #[structopt(short = "to", long)]
        /// a z_address of the recipient
        to: String,
    },
    /// Get my available shielded address(es)
    Zaddr {
        #[structopt(short, long)]
        /// a z_address of the recipient
        all: bool,
    },
}

impl ToString for Cmd {
    fn to_string(&self) -> String {
	match self {
            Self::Sendmsg{ .. } => String::from("sendmsg"),
            Self::Zaddr{ .. } => String::from("zaddr"),
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
