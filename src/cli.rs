use clap::{Parser, Subcommand};
use serde::{Deserialize, Serialize};
use std::{ffi::OsStr, fs::File, path::PathBuf};
use twelf::config;

#[cfg(target_os = "macos")]
pub static DFT_CFG_FILE: &str = "/config/config.toml";
#[cfg(target_os = "linux")]
pub static DFT_CFG_FILE: &str = "/etc/hermesd/config.toml";

pub const HELP_TEMPLATE: &str = "{bin} {version}

{about}

USAGE:
    {usage}

{all-args}
";

#[config]
#[derive(Parser, Debug, Clone)]
#[clap(
help_template = HELP_TEMPLATE,
version,
about = "Exports designated system properties to the upstream service.",
long_about = None
)]
pub struct CliConfig {
    #[clap(short, long, env = "HERMES_CFG_FILE")]
    #[clap(required = false)]
    #[serde(default)]
    pub config: Option<PathBuf>,

    #[clap(subcommand)]
    pub sub_cmds: SubCmds,
}

#[derive(Subcommand, Debug, Deserialize, Serialize)]
pub enum SubCmds {
    /// Runs the application and begins reporting metrics.
    Run(RunCmd),
}

#[derive(Parser, Debug, Deserialize, Serialize)]
pub struct RunCmd {
    #[clap(short, long, env = "HERMESD_TOKEN")]
    token: String,
}

mod defaults {
    pub fn default_bundle_dir() -> String {
        "_pagefind".into()
    }
    pub fn default_false() -> bool {
        false
    }
}

pub fn path_readable_file(value: &OsStr) -> Result<(), String> {
    let path = PathBuf::from(value); //.as_ref();

    if path.is_dir() {
        return Err(format!(
            "{}: Input path must be a file, not a directory",
            path.display()
        ));
    }

    File::open(&path)
        .map(|_| ())
        .map_err(|e| format!("{}: {}", path.display(), e))
}
