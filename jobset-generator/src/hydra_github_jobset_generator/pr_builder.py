#!/usr/bin/env python3

import json
from typing import Dict, Optional, Callable
from pprint import pprint  # noqa
import urllib.parse
from .hydra_types import (
    HydraJob,
    FlattenedHydraJob,
    HydraJobInput,
    HydraJobLegacy,
    HydraJobFlake,
    HydraInputDefinition,
    JobInputCollection,
    flatten_hydra_job,
)
from .github_types import PullRequest
from .config import JobConfig


MakeDefinition = Callable[[JobConfig, PullRequest], HydraInputDefinition]


def make_flake_definition(
    _job_config: JobConfig, pr: PullRequest
) -> HydraInputDefinition:
    return HydraJobFlake(
        flake="git+ssh://{}?{}".format(
            pr["head"]["repo"]["ssh_url"],
            urllib.parse.urlencode(
                {
                    "ref": pr["head"]["ref"],
                    "rev": pr["head"]["sha"],
                }
            ),
        )
    )


def make_legacy_definition(
    job_config: JobConfig, pr: PullRequest
) -> HydraInputDefinition:
    inputs: JobInputCollection = job_config["input_template"]

    inputs["src"] = HydraJobInput(
        type="git",
        value="{} {}".format(pr["head"]["repo"]["git_url"], pr["head"]["sha"]),
        emailresponsible=job_config["email_responsible"],
    )

    return HydraJobLegacy(
        nixexprinput=job_config["inputname"],
        nixexprpath=job_config["inputpath"],
        inputs=inputs,
    )


def build_pr_jobsets(
    pull_request_json: str, job_config: JobConfig, make_definition
) -> Dict[str, FlattenedHydraJob]:
    prs: Dict[str, PullRequest] = json.load(open(pull_request_json))

    jobs: Dict[str, FlattenedHydraJob] = {}
    for (prkey, pr) in prs.items():
        job = make_job(pr, job_config, make_definition)
        if job is not None:
            jobs[f"pr-{prkey}"] = flatten_hydra_job(job)

    return jobs


def make_job(
    pr: PullRequest, job_config: JobConfig, make_definition: MakeDefinition
) -> Optional[HydraJob]:

    job = HydraJob(
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
