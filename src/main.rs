use clap::Parser;
use hermesd::cli::{CliConfig, SubCmds};

fn main() {
    let parsed_cli = CliConfig::parse();

    eprintln!("Config: {:?}", parsed_cli);

    match parsed_cli.cmds {
        SubCmds::Run { token } => println!("Token supplied: {}", token),
        SubCmds::Register { register_uri } => register_node(register_uri),
    };
}

fn register_node(_u: String) {}
