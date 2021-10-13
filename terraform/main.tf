resource "hydra_project" "pr-example" {
  name         = "pr-example"
  display_name = "PR Example"
  description  = "An example of building a GitHub repository's PRs."
  owner        = "alice"
  enabled      = true
  visible      = true

  declarative {
    file  = ".hydra/project.json"
    type  = "git"
    value = "https://github.com/DeterminateSystems/hydra-github-jobsets-example-project.git main"
  }
}
