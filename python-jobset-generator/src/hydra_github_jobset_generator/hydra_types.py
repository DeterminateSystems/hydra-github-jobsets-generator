from typing import TypedDict, Union, Dict, Optional


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
