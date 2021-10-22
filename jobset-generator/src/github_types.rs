use serde::Deserialize;
use std::collections::BTreeMap;

#[derive(Deserialize)]
pub enum AuthorAssociation {
    #[serde(rename = "MEMBER")]
    Member,
    // !!! Flesh out this structure so we don't get errors at run time
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
    pub number: u64,
    pub title: String,
    pub user: User,
}

pub type PullRequests = BTreeMap<String, PullRequest>;
