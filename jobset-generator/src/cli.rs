use std::{collections::BTreeMap, fs::File, io::BufReader};

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
    #[structopt(long, conflicts_with = "flakes")]
    template: Option<String>,
    #[structopt(long, conflicts_with = "template")]
    flakes: bool,
    #[structopt(long, default_value = "300")]
    check_interval: u64,
    #[structopt(long, default_value = "1")]
    scheduling_shares: u64,
    #[structopt(long)]
    email_enable: bool,
    #[structopt(long, default_value = "")]
    email_override: String,
    #[structopt(long)]
    email_responsible: bool,
    #[structopt(long, default_value = "3")]
    keep_evaluations: u64,
    #[structopt(long, default_value = "src")]
    input_name: String,
    #[structopt(long, default_value = "input_path")]
    input_path: String,
}

pub fn cli() -> Result<()> {
    let args = Args::from_args();

    let input_template: JobInputCollection = if let Some(template) = args.template {
        let template_file = File::open(template)?;
        let template_rdr = BufReader::new(template_file);

        serde_json::from_reader(template_rdr)?
    } else {
        BTreeMap::new()
    };

    let job_config = JobConfig {
        checkinterval: args.check_interval,
        emailoverride: args.email_override,
        enableemail: args.email_enable,
        email_responsible: args.email_responsible,
        inputname: args.input_name,
        inputpath: args.input_path,
        keepnr: args.keep_evaluations,
        schedulingshares: args.scheduling_shares,
        input_template,
    };

    let make_definition = if args.flakes {
        make_flake_definition
    } else {
        make_legacy_definition
    };

    let jobsets = build_pr_jobsets(args.pull_requests_file, job_config, &make_definition)?;
    let json = serde_json::to_string(&jobsets)?;

    println!("{}", json);

    Ok(())
}
