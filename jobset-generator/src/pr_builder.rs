use std::fs::File;
use std::io::BufReader;

use crate::config::JobConfig;
use crate::github_types::{PullRequest, PullRequests};
use crate::hydra_types::{
    HydraInputDefinition, HydraJobset, HydraJobsetFlake, HydraJobsetInput, HydraJobsetLegacy,
    HydraJobsets,
};
use crate::Result;

pub type MakeDefinition = dyn Fn(JobConfig, PullRequest) -> HydraInputDefinition;

pub fn make_flake_definition(_job_config: JobConfig, pr: PullRequest) -> HydraInputDefinition {
    HydraInputDefinition::Flake(HydraJobsetFlake {
        flake_uri: format!(
            "git+ssh://{}?{}",
            pr.head.repo.ssh_url,
            url_encoded_data::stringify(&[("ref", &pr.head.r#ref), ("rev", &pr.head.sha)])
        ),
    })
}

pub fn make_legacy_definition(job_config: JobConfig, pr: PullRequest) -> HydraInputDefinition {
    let mut inputs = job_config.input_template;
    inputs.insert(
        job_config.inputname.clone(),
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

pub fn build_pr_jobsets(
    pull_requests_path: String,
    job_config: JobConfig,
    make_definition: &MakeDefinition,
) -> Result<HydraJobsets> {
    let pull_requests_file = File::open(pull_requests_path)?;
    let pull_requests_rdr = BufReader::new(pull_requests_file);
    let pull_requests: PullRequests = serde_json::from_reader(pull_requests_rdr)?;

    let mut jobs = HydraJobsets::new();
    for (key, pr) in pull_requests {
        if let Some(job) = make_job(pr, job_config.clone(), make_definition) {
            let flattened_job = job.flatten();
            jobs.insert(format!("{}", key), flattened_job);
        }
    }

    Ok(jobs)
}

// TODO: does this really need to return an option?
fn make_job(
    pr: PullRequest,
    job_config: JobConfig,
    make_definition: &MakeDefinition,
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::github_types::{AuthorAssociation, Branch, Repo, User};
    use crate::hydra_types::JobInputCollection;

    #[test]
    fn input_name_is_used() {
        let cfg = JobConfig {
            checkinterval: 1,
            email_responsible: false,
            emailoverride: String::from(""),
            enableemail: false,
            input_template: JobInputCollection::new(),
            inputname: String::from("foobar"),
            inputpath: String::from("ci.nix"),
            keepnr: 3,
            schedulingshares: 1,
        };
        let pr = PullRequest {
            author_association: AuthorAssociation::Member,
            head: Branch {
                r#ref: String::from("head-ref"),
                sha: String::from("head-sha"),
                repo: Repo {
                    git_url: String::from("head-git_url"),
                    ssh_url: String::from("head-ssh_url"),
                },
                user: User {
                    login: String::from("head-login"),
                },
            },
            html_url: String::from("html_url"),
            number: String::from("123"),
            title: String::from("Title"),
            user: User {
                login: String::from("login"),
            },
            base: Branch {
                r#ref: String::from("base-ref"),
                sha: String::from("base-sha"),
                repo: Repo {
                    git_url: String::from("base-git_url"),
                    ssh_url: String::from("base-ssh_url"),
                },
                user: User {
                    login: String::from("base-login"),
                },
            },
        };

        let defn = make_legacy_definition(cfg, pr);

        if let HydraInputDefinition::Legacy(legacydef) = defn {
            assert_eq!(legacydef.nixexprinput, "foobar");
            assert_eq!(legacydef.nixexprpath, "ci.nix");
        } else {
            panic!("make_legacy_definition didn't make a legacy definition");
        }
    }
}
