# Generate declarative jobsets for Hydra for a GitHub Project

This tool uses Hydra's [_declarative jobsets_](https://github.com/NixOS/hydra/blob/master/doc/manual/src/plugins/declarative-projects.md) to automatically create Hydra jobsets for your GitHub project.

Configuring declarative jobsets use several points of indirection, and this repository's goal is to simplify and streamline the setup process.

## How Declarative Jobsets Work

The goal of Declarative Jobsets is to have a Hydra Project whose Jobsets are entirely defined outside of Hydra's interface.
This means Declarative Jobsets is configured at the _Project_ level.

Briefly:

* The Project's configuration points to a JSON document configuring the Hydra jobset `.jobsets`.
* The `.jobsets` jobset has a single Hydra job called `jobsets`.
* The `jobsets` job should produce a JSON document at `$out` with a map of jobset names and jobset configurations. 

Some implementation details:

* Before the `.jobsets` jobset is evaluated, the Project's configuration is updated. This means updates to the data pointed to by the Project is reflected.
* The `jobsets` job does not and should not include the configuration for the `.jobsets` job.


## How to use this tool

This example is assuming you will build PRs for a project at github.com/YOURORGNAME/YOURREPONAME.

### Configuring your repository: .jobsets

In your project's repository, create a file at `.hydra/project.json`. This contains the configuration for your Hydra's `.jobsets` jobset. It should look like this:

```json
{
    "enabled": 1,
    "hidden": false,
    "description": "GitHub Pull Request Jobset Generator",
    "nixexprinput": "generator",
    "nixexprpath": "jobset/generate.nix",
    "checkinterval": 300,
    "schedulingshares": 100,
    "enableemail": false,
    "emailoverride": "",
    "keepnr": 3,
    "inputs": {
        "generator_config": {
            "type": "git",
            "value": "https://github.com/YOURORGNAME/YOURREPONAME.git main",
            "emailresponsible": false
        },
        "generator": {
            "type": "git",
            "value": "https://github.com/DeterminateSystems/hydra-github-jobsets-generator.git main",
            "emailresponsible": false
        },
        "nixpkgs": {
            "type": "git",
            "value": "git://github.com/NixOS/nixpkgs.git nixos-unstable-small",
            "emailresponsible": false
        },
        "pull_requests": {
            "type": "githubpulls",
            "value": "YOURORGNAME YOURREPONAME",
            "emailresponsible": false
        }
    }
}
```

Take care to replace both `YOURORGNAME` and `YOURREPONAME` with your GitHub organization and repo's names.

### Configuring your repository

Then, create a file at `.hydra/config.json`, and include all of the other Hydra inputs you want to use.

For example, if your project depends on Nixpkgs, write the following to `.hydra/config.json`:

```json
{
    "inputs": {
        "nixpkgs": {
            "type": "git",
            "value": "git://github.com/NixOS/nixpkgs.git nixos-unstable-small",
            "emailresponsible": false
        }
    }
}
```

If your project has no other inputs, write `{}` to the file.

### Configuring your Hydra

Create a project on your Hydra which uses a declarative input of type `git` with the file pointing to `.hydra/project.json`.

If you use the [Terraform Hydra provider](https://registry.terraform.io/providers/DeterminateSystems/hydra/latest), it would look like this:

```terraform
resource "hydra_project" "pr-example" {
  name         = "pr-example"
  display_name = "PR Example"
  description  = "An example of building a GitHub repository's PRs."
  owner        = "alice"
  enabled      = true
  visible      = true

  declarative {
      file = ".hydra/project.json"
      type = "git"
      value = "https://github.com/YOURORGNAME/YOURREPONAME.git main"
  }
}
```

### Recap

When the `.jobsets` jobset evaluates and builds you should now see a jobset for every open PR. This jobset will re-evaluate every 300 seconds (5 minutes).
