use serde::Deserialize;

#[derive(Deserialize)]
enum AuthorAssociation {
    #[serde(rename = "MEMBER")]
    Member,
    // !!! Flesh out this structure so we don't get errors at run time
}

#[derive(Deserialize)]
struct User {
    login: String,
}

#[derive(Deserialize)]
struct Repo {
    git_url: String,
    ssh_url: String,
}

#[derive(Deserialize)]
struct Branch {
    r#ref: String,
    repo: Repo,
    sha: String,
    user: User,
}

struct PullRequest {
    author_association: AuthorAssociation,
    base: String,
    head: String,
    html_url: String,
    number: u64,
    title: String,
    user: String,
}
