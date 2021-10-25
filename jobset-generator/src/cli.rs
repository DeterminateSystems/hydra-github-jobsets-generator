use std::{fs::File, io::BufReader};
use serde;
use serde::{Deserialize, Serialize};
use structopt::StructOpt;

use crate::Result;
use crate::{
    config::JobConfig,
    hydra_types::JobInputCollection,
    pr_builder::{build_pr_jobsets, make_flake_definition, make_legacy_definition},
};

#[derive(StructOpt)]
struct Args {
    pull_requests_file: String,
    config_file: String,
}

#[derive(Serialize,Deserialize)]
#[serde(deny_unknown_fields)] 
struct GeneratorConfig {
    #[serde(default)]
    flakes: bool,

    #[serde(default = "default_check_interval")]
    check_interval: u64,

    #[serde(default = "default_scheduling_shares")]
    scheduling_shares: u64,

    #[serde(default)]
    email_enable: bool,

    email_override: Option<String>,

    #[serde(default)]
    email_responsible: bool,

    #[serde(default = "default_keep_evaluations")]
    keep_evaluations: u64,

    #[serde(default = "default_input_name")]
    input_name: String,

    #[serde(default = "default_input_path")]
    input_path: String,

    #[serde(default)]
    inputs: JobInputCollection
}

fn default_check_interval() -> u64 {
    300
}

fn default_scheduling_shares() -> u64 {
    1
}

fn default_keep_evaluations() -> u64 {
    3
}

fn default_input_name() -> String {
    String::from("src")
}

fn default_input_path() -> String {
    String::from("default.nix")
}

pub fn cli() -> Result<()> {
    let args = Args::from_args();

    let config: GeneratorConfig = {
        let config_file = File::open(args.config_file)?;
        serde_json::from_reader(BufReader::new(config_file))?
    };

    eprintln!("Expanded the provided configuration input to:");
    eprintln!("{}", serde_json::to_string_pretty(&config).unwrap());

    let job_config = JobConfig {
        checkinterval: config.check_interval,
        emailoverride: config.email_override.unwrap_or_default(),
        enableemail: config.email_enable,
        email_responsible: config.email_responsible,
        inputname: config.input_name,
        inputpath: config.input_path,
        keepnr: config.keep_evaluations,
        schedulingshares: config.scheduling_shares,
        input_template: config.inputs,
    };

    let make_definition = if config.flakes {
        make_flake_definition
    } else {
        make_legacy_definition
    };

    let jobsets = build_pr_jobsets(args.pull_requests_file, job_config, &make_definition)?;
    let json = serde_json::to_string(&jobsets)?;

    println!("{}", json);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::GeneratorConfig;

    #[test]
    fn can_decode_empty_config() {
        let _cfg: GeneratorConfig = serde_json::from_str("{}").expect("Failed to decode all-defaults");
    }
}