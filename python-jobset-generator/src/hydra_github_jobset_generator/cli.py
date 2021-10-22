import argparse
import sys
import json
from .config import JobConfig
from .hydra_types import JobInputCollection
from .pr_builder import make_legacy_definition, make_flake_definition, build_pr_jobsets


def argument_parser():
    parser = argparse.ArgumentParser(
        description="Generate declarative jobsets given pull request data."
    )
    parser.add_argument(
        "pull_requests_file",
        type=str,
        help="Path to the file containing pull request data.",
    )
    parser.add_argument(
        "--template",
        dest="template",
        type=str,
        help="Input template.  Incompatible with --flakes.",
    )
    parser.add_argument(
        "--flakes",
        dest="use_flakes",
        action="store_true",
        help="Generate declarative flakes. Incompatible with --template.",
    )
    parser.add_argument(
        "--check_interval",
        default=300,
        help="Number of seconds between each Hydra evaluation of the generated jobs.",
    )
    parser.add_argument(
        "--scheduling_shares",
        default=1,
        help="How many shares each generated job should have.",
    )
    parser.add_argument(
        "--email_enable",
        action="store_true",
        help="Enable sending email for broken jobs.",
    )
    parser.add_argument(
        "--email_override",
        default="",
        help="The email address to send mail to for your project's changes.",
    )
    parser.add_argument(
        "--email_responsible",
        action="store_true",
        help="Email the authors of commits in the PR if something fails.",
    )
    parser.add_argument(
        "--keep_evalutions",
        default=3,
        help="How many evaluations to keep GC rooted on Hydra.",
    )
    parser.add_argument(
        "--input_name",
        default="src",
        help="Name of the project's input.",
    )
    parser.add_argument(
        "--input_path",
        default="default.nix",
        help="Path to the .nix file within your project's directory which defines jobs.",
    )

    return parser


def main():
    args = argument_parser().parse_args()

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

    print(
        json.dumps(
            build_pr_jobsets(args.pull_requests_file, job_config, make_definition)
        )
    )
