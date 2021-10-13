from typing import TypedDict
from .hydra_types import JobInputCollection


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
