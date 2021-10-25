use serde::Deserialize;
use std::collections::BTreeMap;

/// https://docs.github.com/en/graphql/reference/enums#commentauthorassociation
#[derive(Deserialize)]
pub enum AuthorAssociation {
    #[serde(rename = "MEMBER")]
    Member,
    #[serde(rename = "CONTRIBUTOR")]
    Contributor,
    #[serde(rename = "NONE")]
    None,
    #[serde(rename = "COLLABORATOR")]
    Collaborator,
    #[serde(rename = "FIRST_TIMER")]
    FirstTimer,
    #[serde(rename = "FIRST_TIME_CONTRIBUTOR")]
    FirstTimeContributor,
    #[serde(rename = "MANNEQUIN")]
    Mannequin,
    #[serde(rename = "OWNER")]
    Owner,
}

#[derive(Deserialize)]
pub struct User {
    pub login: String,
}

#[derive(Deserialize)]
pub struct Repo {
    pub git_url: String,
    pub ssh_url: String,
}

#[derive(Deserialize)]
pub struct Branch {
    pub r#ref: String,
    pub repo: Repo,
    pub sha: String,
    pub user: User,
}

#[derive(Deserialize)]
pub struct PullRequest {
    pub author_association: AuthorAssociation,
    pub base: Branch,
    pub head: Branch,
    pub html_url: String,
    pub number: String,
    pub title: String,
    pub user: User,
}

pub type PullRequests = BTreeMap<String, PullRequest>;
