{ pkgs }:
pkgs.rustPlatform.buildRustPackage rec {
  pname = "jobset-generator";
  version = "unreleased";

  src = ./.;
  cargoLock.lockFile = src + "/Cargo.lock";

  meta = with pkgs.lib; {
    homepage = "https://github.com/DeterminateSystems/hydra-github-project";
    description = "Declaratively generate jobsets for all a project's branches and PRs.";
    license = licenses.mit;
    maintainers = teams.determinatesystems.members;
  };
}
