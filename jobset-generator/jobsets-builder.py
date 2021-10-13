#!/usr/bin/env python3

import json
import sys
from typing import TypedDict, Union, Literal, Dict, Optional, Callable
from pprint import pprint  # noqa
import argparse
import urllib.parse

AuthorAssociation = Union[Literal["MEMBER"]]


class User(TypedDict):
    login: str


class Repo(TypedDict):
    git_url: str
    ssh_url: str


class Branch(TypedDict):
    ref: str
    repo: Repo
    sha: str
    user: User


class PullRequest(TypedDict):
    title: str
    author_association: AuthorAssociation
    head: Branch
    base: Branch
    user: User
    html_url: str
    number: int


class HydraJobInput(TypedDict):
    type: str
    value: str
    emailresponsible: bool


JobInputCollection = Dict[str, HydraJobInput]


class HydraJobLegacy(TypedDict):
    nixexprinput: str
    nixexprpath: str
    inputs: JobInputCollection


class HydraJobFlake(TypedDict):
    flake: str


HydraInputDefinition = Union[HydraJobLegacy, HydraJobFlake]


class HydraJob(TypedDict):
    enabled: bool
    hidden: bool
    description: str
    checkinterval: int
    schedulingshares: int
    enableemail: bool
    emailoverride: str
    keepnr: int
    definition: HydraInputDefinition


class FlattenedHydraJob(TypedDict):
    enabled: bool
    hidden: bool
    description: str
    checkinterval: int
    schedulingshares: int
    enableemail: bool
    emailoverride: str
    keepnr: int
    flake: Optional[str]
    nixexprinput: Optional[str]
    nixexprpath: Optional[str]
    inputs: Optional[JobInputCollection]


class JobConfig(TypedDict):
    checkinterval: int
    emailoverride: str
    enableemail: bool
    keepnr: int
    schedulingshares: int
    input_template: JobInputCollection
    email_responsible: bool
    inputname: str
    inputpath: str


def flatten_hydra_job(job: HydraJob) -> FlattenedHydraJob:
    flat = FlattenedHydraJob(
        enabled=job["enabled"],
        hidden=job["hidden"],
        description=job["description"],
        checkinterval=job["checkinterval"],
        schedulingshares=job["schedulingshares"],
        enableemail=job["enableemail"],
        emailoverride=job["emailoverride"],
        keepnr=job["keepnr"],
        flake=None,
        nixexprinput=None,
        nixexprpath=None,
        inputs=None,
    )

    if "flake" in job["definition"]:
        flake: HydraJobFlake = job["definition"]  # type: ignore
        flat["flake"] = flake["flake"]
    elif "nixexprinput" in job["definition"]:
        legacy: HydraJobLegacy = job["definition"]  # type: ignore
        flat["nixexprinput"] = legacy["nixexprinput"]
        flat["nixexprpath"] = legacy["nixexprpath"]
        flat["inputs"] = legacy["inputs"]

    return flat


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


def main(
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


if __name__ == "__main__":
    parser = argparse.ArgumentParser(
        description="Generate declarative jobsets given pull request data."
    )
    parser.add_argument(
        "pull_requests_file",
        type=str,
        help="Path to the file containing pull request data.",
    )
    parser.add_argument(
        "--flakes",
        dest="use_flakes",
        action="store_true",
        help="Generate declarative flakes",
    )
    parser.add_argument(
        "--check_interval",
        default=300,
        help="Generate declarative flakes",
    )
    parser.add_argument(
        "--scheduling_shares",
        default=1,
        help="Generate declarative flakes",
    )
    parser.add_argument(
        "--email_override",
        default="",
        help="Generate declarative flakes",
    )
    parser.add_argument(
        "--email_enable",
        action="store_true",
        help="Generate declarative flakes",
    )
    parser.add_argument(
        "--email_responsible",
        action="store_true",
        help="Email the authors of commits in the PR if something fails.",
    )
    parser.add_argument(
        "--keep_evalutions",
        default=3,
        help="Generate declarative flakes",
    )
    parser.add_argument(
        "--input_name",
        default="src",
        help="Generate declarative flakes",
    )
    parser.add_argument(
        "--input_path",
        default="default.nix",
        help="Generate declarative flakes",
    )
    parser.add_argument(
        "--template",
        dest="template",
        type=str,
        help="Input template.",
    )

    args = parser.parse_args()

    if args.template is not None and args.use_flakes is True:
        print("Cannot combine --template and --flakes", file=sys.stderr)
        exit(1)

    input_template: JobInputCollection = {}
    if args.template is not None:
        input_template = json.load(open(args.template))

    job_config = JobConfig(
        checkinterval=args.check_interval,
        emailoverride=args.email_override,
        enableemail=args.email_enable,
        email_responsible=args.email_responsible,
        inputname=args.input_name,
        inputpath=args.input_path,
        keepnr=args.keep_evalutions,
        schedulingshares=args.scheduling_shares,
        input_template=input_template,
    )

    make_definition = (
        make_flake_definition if args.use_flakes else make_legacy_definition
    )

    main(args.pull_requests_file, job_config, make_definition)
