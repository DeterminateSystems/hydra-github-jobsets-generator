use crate::config::JobConfig;
use crate::github_types::{PullRequest, PullRequests};
use crate::hydra_types::{
    HydraInputDefinition, HydraJobset, HydraJobsetFlake, HydraJobsetInput, HydraJobsetLegacy,
    HydraJobsets,
};

trait MakeDefinition: Fn(JobConfig, PullRequest) -> HydraInputDefinition + Clone {}

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
    let mut inputs = job_config.input_template;
    inputs.insert(
        String::from("src"),
        HydraJobsetInput {
            r#type: String::from("git"),
            value: format!("{} {}", pr.head.repo.git_url, pr.head.sha),
            emailresponsible: job_config.email_responsible,
        },
    );

    HydraInputDefinition::Legacy(HydraJobsetLegacy {
        inputs,
        nixexprinput: job_config.inputname,
        nixexprpath: job_config.inputpath,
    })
}

fn build_pr_jobsets(
    pull_requests: PullRequests,
    job_config: JobConfig,
    make_definition: impl MakeDefinition,
) -> HydraJobsets {
    let mut jobs = HydraJobsets::new();

    for (key, pr) in pull_requests {
        if let Some(job) = make_job(pr, job_config.clone(), make_definition.clone()) {
            let flattened_job = job.flatten();
            jobs.insert(format!("pr-{}", key), flattened_job);
        }
    }

    jobs
}

// TODO: does this really need to return an option?
fn make_job(
    pr: PullRequest,
    job_config: JobConfig,
    make_definition: impl MakeDefinition,
) -> Option<HydraJobset> {
    let job_config_clone = job_config.clone();

    Some(HydraJobset {
        enabled: true,
        hidden: false,
        description: format!("{} by {}: {}", pr.title, pr.user.login, pr.html_url),
        checkinterval: job_config.checkinterval,
        schedulingshares: job_config.schedulingshares,
        enableemail: job_config.enableemail,
        emailoverride: job_config.emailoverride,
        keepnr: job_config.keepnr,
        definition: make_definition(job_config_clone, pr),
    })
}
