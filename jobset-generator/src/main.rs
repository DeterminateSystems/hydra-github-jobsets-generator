#![allow(dead_code)]

use std::error::Error;

mod cli;
mod config;
mod github_types;
mod hydra_types;
mod pr_builder;

pub type Result<T, E = Box<dyn Error + Send + Sync + 'static>> = core::result::Result<T, E>;

fn main() {
    if let Err(e) = cli::cli() {
        eprintln!("{}", e);

        std::process::exit(1);
    }
}
