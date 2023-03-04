use clap::{self, crate_version, Parser, Subcommand};
use lazy_static::lazy_static;

// We need to specify our version in a static because we've painted clap
// into a corner. We've told it that every string we give it will be
// 'static, but we need to build the version string dynamically. We can
// fake the 'static lifetime with lazy_static.
lazy_static! {
    static ref LONG_VERSION: String = long_version(None, true);
}

const ABOUT: &str = "
hermesd registers this node and begins polling to send/receive messagin from the controller.
Use -h for short descriptions and --help for more details.
Project home page: https://github.com/nwlnexus/hermesd
";

const TEMPLATE: &str = "\
{bin} {version}
{author}
{about}

USAGE:
    {usage}

{all-args}
";

const BIN_NAME: &str = "HERMESD";

#[derive(Parser, Debug, Clone)]
#[command(
help_template = TEMPLATE,
version,
author,
long_version = LONG_VERSION.as_str(),
about = ABOUT,
long_about = None
)]
pub struct CliConfig {
    /// Sub commands for the application.
    #[command(subcommand)]
    pub cmds: SubCmds,
}

/// Valid sub commands for the application.
#[derive(Subcommand, Debug, Clone)]
pub enum SubCmds {
    /// Runs the application and begins reporting metrics.
    Run {
        /// Secure token provided by the controller for endpoint check in.
        #[arg(short, long, env = format!("{}_TOKEN", BIN_NAME))]
        token: String,
    },
    /// Registers the node
    Register {
        /// URI of the controller with which to register the application.
        #[arg(short, long, env = format!("{}_REGISTER_URL", BIN_NAME))]
        register_uri: String,
    },
}

/// Return the "long" format of ripgrep's version string.
///
/// If a revision hash is given, then it is used. If one isn't given, then
/// the HERMESD_BUILD_GIT_HASH env var is inspected for it. If that isn't set,
/// then a revision hash is not included in the version string returned.
///
/// If `cpu` is true, then the version string will include the compiled and
/// runtime CPU features.
pub fn long_version(revision_hash: Option<&str>, cpu: bool) -> String {
    // Do we have a git hash?
    // (Yes, if ripgrep was built on a machine with `git` installed.)
    let hash = match revision_hash.or(option_env!("HERMESD_BUILD_GIT_HASH")) {
        None => String::new(),
        Some(githash) => format!(" (rev {})", githash),
    };
    if !cpu {
        format!("{}{}", crate_version!(), hash,)
    } else {
        let runtime = runtime_cpu_features();
        if runtime.is_empty() {
            format!(
                "{}{}\n{} (compiled)",
                crate_version!(),
                hash,
                compile_cpu_features().join(" ")
            )
        } else {
            format!(
                "{}{}\n{} (compiled)\n{} (runtime)",
                crate_version!(),
                hash,
                compile_cpu_features().join(" "),
                runtime.join(" ")
            )
        }
    }
}

/// Returns the relevant CPU features enabled at compile time.
fn compile_cpu_features() -> Vec<&'static str> {
    let mut features = vec![];
    if cfg!(feature = "simd-accel") {
        features.push("+SIMD");
    } else {
        features.push("-SIMD");
    }
    if cfg!(feature = "avx-accel") {
        features.push("+AVX");
    } else {
        features.push("-AVX");
    }
    features
}

/// Returns the relevant CPU features enabled at runtime.
#[cfg(target_arch = "x86_64")]
fn runtime_cpu_features() -> Vec<&'static str> {
    // This is kind of a dirty violation of abstraction, since it assumes
    // knowledge about what specific SIMD features are being used.

    let mut features = vec![];
    if is_x86_feature_detected!("ssse3") {
        features.push("+SIMD");
    } else {
        features.push("-SIMD");
    }
    if is_x86_feature_detected!("avx2") {
        features.push("+AVX");
    } else {
        features.push("-AVX");
    }
    features
}

/// Returns the relevant CPU features enabled at runtime.
#[cfg(not(target_arch = "x86_64"))]
fn runtime_cpu_features() -> Vec<&'static str> {
    vec![]
}
