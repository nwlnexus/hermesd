use clap::{Parser, Subcommand, ValueEnum};
use hermesd::generate_service_agent;
use std::path::PathBuf;

#[cfg(target_os = "macos")]
static DFT_CFG_FILE: &str = "/config/config.toml";
#[cfg(target_os = "linux")]
static DFT_CFG_FILE: &str = "/etc/hermesd/config.toml";

#[derive(Parser, Debug)]
#[command(author, version, about)]
#[command(propagate_version = true)]
/// Exports designated system properties to the upstream service.
struct Cli {
    /// Sets a custom config file
    #[arg(
        short,
        long,
        default_value = DFT_CFG_FILE,
        env = "HERMESD_CONFIG_FILE",
        value_name = "CONFIG_FILE"
    )]
    config: Option<PathBuf>,

    #[command(subcommand)]
    sub_cmds: SubCmds,
}

#[derive(Subcommand, Debug)]
enum SubCmds {
    #[command(arg_required_else_help(true))]
    /// Manages the hermesd service.
    Service {
        #[arg(value_enum)]
        action: ServiceAction,
    },
    #[command(arg_required_else_help(true))]
    /// Runs the application and begins reporting metrics.
    Run {
        #[arg(short, long)]
        token: String,
    },
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug)]
enum ServiceAction {
    /// Detect and install appropriate service/launch agent.
    Install,
}

fn main() {
    let cli = Cli::parse();

    match &cli.sub_cmds {
        SubCmds::Run { token } => {
            println!("You chose to run app with token: {token:?}")
        }
        SubCmds::Service { action: _action } => generate_service_agent(),
    }
}
