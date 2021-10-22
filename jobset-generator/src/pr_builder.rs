use crate::config::JobConfig;
use crate::github_types::{PullRequest, PullRequests};
use crate::hydra_types::{
    FlattenedHydraJobset, HydraInputDefinition, HydraJobset, HydraJobsetFlake, HydraJobsetInput,
    HydraJobsetLegacy, HydraJobsets, JobInputCollection,
};
use serde_json;
use std::collections::BTreeMap;
use std::path::Path;

fn make_flake_definition(_job_config: JobConfig, pr: PullRequest) -> HydraInputDefinition {
    HydraInputDefinition::Flake(HydraJobsetFlake {
        flake_uri: format!(
            "git+ssh://{}?{}",
            pr.head.repo.ssh_url,
            url_encoded_data::stringify(&[("ref", &pr.head.r#ref), ("rev", &pr.head.sha)])
        ),
    })
}

fn make_legacy_definition(job_config: JobConfig, pr: PullRequest) -> HydraInputDefinition {
    HydraInputDefinition::Legacy(HydraJobsetLegacy {
        inputs: job_config.input_template,
        nixexprinput: job_config.inputname,
        nixexprpath: job_config.inputpath,
    })
}

fn build_pr_jobsets(
    pull_requests: PullRequests,
    job_config: JobConfig,
    make_definition: (),
) -> HydraJobsets {
    let mut jobs = HydraJobsets::new();

    jobs
}

/*
import urllib.parse


MakeDefinition = Callable[[JobConfig, PullRequest], HydraInputDefinition]



def build_pr_jobsets(
    pull_request_json: str, job_config: JobConfig, make_definition
) -> Dict[str, FlattenedHydraJobset]:
    prs: Dict[str, PullRequest] = json.load(open(pull_request_json))

    jobs: Dict[str, FlattenedHydraJobset] = {}
    for (prkey, pr) in prs.items():
        job = make_job(pr, job_config, make_definition)
        if job is not None:
            jobs[f"pr-{prkey}"] = flatten_hydra_job(job)

    return jobs


def make_job(
    pr: PullRequest, job_config: JobConfig, make_definition: MakeDefinition
) -> Optional[HydraJobset]:

    job = HydraJobset(
        enabled=True,
        hidden=False,
        description="{title} by {username}: {link}".format(
            title=pr["title"],
            username=pr["user"]["login"],
            link=pr["html_url"],
        ),
        checkinterval=job_config["checkinterval"],
        schedulingshares=job_config["schedulingshares"],
        enableemail=job_config["enableemail"],
        emailoverride=job_config["emailoverride"],
        keepnr=job_config["keepnr"],
        definition=make_definition(job_config, pr),
    )

    return job
*/
