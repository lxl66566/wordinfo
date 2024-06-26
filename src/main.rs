mod cli;
mod config;
mod core;

use core::info::info;

use cli::{Commands, CLI};
use config::{Config, CONFIG};
use die_exit::*;

#[tokio::main]
async fn main() {
    // info word
    if CLI.command.is_none() {
        return info(CLI.words.clone()).await;
    }
    // export
    match CLI.command.as_ref().unwrap() {
        Commands::Export(e) => CONFIG.save(&e.format).die_with(|err| {
            format!(
                "Fail to save config to `{}` because of: {}",
                Config::config_path().display(),
                err
            )
        }),
        Commands::Show => CONFIG.show_all_selectors(),
        Commands::Config(_) => unimplemented!(),
    }
}
