use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(author = "NWLNEXUS LLC", version, about)]
/// Exports designated system properties to the upstream service.
struct Cli {
    #[command(subcommand)]
    sub_cmds: SubCmds,
}

#[derive(Subcommand, Debug)]
enum SubCmds {
    #[command(arg_required_else_help(true))]
    /// Manages the cloudflared service.
    Service {},
    #[command(arg_required_else_help(true))]
    /// Runs the application and begins reporting metrics.
    Run {
        #[arg(short, long)]
        token: String,
    },
}

fn main() {
    let args = Cli::parse();

    println!("You chose: {:?}", args);
}
