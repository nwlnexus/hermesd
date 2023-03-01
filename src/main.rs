use clap::CommandFactory;
use hermesd::cli::{CliOptions, DFT_CFG_FILE};
use twelf::Layer;

fn main() {
    let matches = CliOptions::command().get_matches();
    let mut config_layers = Vec::with_capacity(2);
    if std::path::Path::new(DFT_CFG_FILE).exists() {
        config_layers.push(Layer::Toml(DFT_CFG_FILE.into()));
    }
    config_layers.push(Layer::Clap(matches));
    let config = CliOptions::with_layers(&config_layers).unwrap();

    eprintln!("Config: {:?}", config);
}
