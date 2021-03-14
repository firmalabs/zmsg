use structopt::StructOpt;

#[derive(StructOpt)]
#[structopt(rename_all = "kebab-case")]
pub enum Cmd {
    /// Calls `sudo systemctl start <service>`.
    Start,
    /// Calls `sudo systemctl stop <service>`.
    Stop,
    /// Calls `sudo systemctl status <service>`.
    Status,
    /// Calls `sudo systemctl kill -s SIGINT <service>`
    Interrupt,
    /// Lists available systemd unit files in `/etc/systemd/system`.
    List,
}

impl ToString for Cmd {
    fn to_string(&self) -> String {
	match self {
	    Self::Start => String::from("start"),
	    Self::Stop => String::from("stop"),
	    Self::Status => String::from("status"),
	    Self::List => String::from("list"),
	    Self::Interrupt => String::from("interrupt"),
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
pub struct Cli {
    #[structopt(short, long, global = true)]
    /// Specify all services
    pub all: bool,

    #[structopt(short, long, default_value = "all", global = true)]
    /// Specify a service 
    pub service: String,

    #[structopt(subcommand)]
    pub cmd: Cmd,
}
