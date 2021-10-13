from typing import TypedDict, Union, Literal

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
