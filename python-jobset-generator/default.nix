{ pkgs }:
pkgs.poetry2nix.mkPoetryApplication {
  projectDir = ./src;

  meta = with pkgs.lib; {
    homepage = "https://github.com/DeterminateSystems/hydra-github-project";
    description = "Declaratively generate jobsets for all a project's branches and PRs.";
    license = licenses.mit;
    maintainers = teams.determinatesystems.members;
  };
}
